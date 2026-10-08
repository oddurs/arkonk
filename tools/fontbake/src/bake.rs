//! Shapes, rasterizes and encodes one group's faces. The byte layout is
//! documented in `crates/ark-glyphs/src/data.rs`, which reads it.
use crate::{
    charsets::{Group, Sets, Style, Unit},
    script,
    spec::{self, Weight},
};
use ark_text::Role;
use miniz_oxide::deflate::compress_to_vec;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};
use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Scaler, Source, image::Content},
    shape::{Direction, ShapeContext},
    text::{Codepoint, Script},
    zeno::{Format, Vector},
};

pub struct Baked {
    pub bytes: Vec<u8>,
    pub report: String,
}

/// Shaping features: kerning and tabular figures on, so digits never shift
/// as values change; Latin ligatures off, because the game draws one glyph
/// per character. Thai mark positioning and Arabic joining are the fonts'
/// required features and stay on.
const FEATURES: [(&str, u16); 5] = [
    ("kern", 1),
    ("tnum", 1),
    ("liga", 0),
    ("clig", 0),
    ("calt", 0),
];

const TATWEEL: char = '\u{0640}';

/// A shaped glyph, in font units.
#[derive(Clone, Copy)]
struct Shaped {
    id: GlyphId,
    x: f32,
    y: f32,
    advance: f32,
}

/// Shapes `s`, returning each cluster's source byte range and glyphs.
fn shape(
    ctx: &mut ShapeContext,
    font: FontRef<'_>,
    s: &str,
) -> Vec<(std::ops::Range<usize>, Vec<Shaped>)> {
    let script = s
        .chars()
        .map(|c| c.script())
        .find(|s| !matches!(s, Script::Common | Script::Inherited | Script::Unknown))
        .unwrap_or(Script::Latin);
    let direction = if script == Script::Arabic {
        Direction::RightToLeft
    } else {
        Direction::LeftToRight
    };
    let mut shaper = ctx
        .builder(font)
        .script(script)
        .direction(direction)
        .features(FEATURES)
        .build();
    shaper.add_str(s);
    let mut clusters = Vec::new();
    shaper.shape_with(|cluster| {
        let glyphs = cluster
            .glyphs
            .iter()
            .map(|g| Shaped {
                id: g.id,
                x: g.x,
                y: g.y,
                advance: g.advance,
            })
            .collect();
        clusters.push((cluster.source.to_range(), glyphs));
    });
    clusters
}

/// The glyphs that draw `unit`, and its advance in font units. Some fonts
/// compose one character from several glyphs (Vietnamese ị is a dotless i
/// and a dot below); they are baked together as one image. An Arabic
/// presentation form is shaped from its letters in the context its form
/// implies (a tatweel on the joining sides), and its own cluster is kept.
fn recipe(
    ctx: &mut ShapeContext,
    font: FontRef<'_>,
    file: &str,
    unit: &[char],
) -> Result<(Vec<Shaped>, f32), String> {
    let name = || {
        unit.iter()
            .map(|c| format!("U+{:04X}", *c as u32))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let (text, keep) = match (unit, script::arabic_source(unit[0])) {
        (&[_], Some((letters, len, form))) => {
            let letters: String = letters[..len].iter().collect();
            let before = matches!(form, script::Form::Final | script::Form::Medial);
            let after = matches!(form, script::Form::Initial | script::Form::Medial);
            let lead = if before { TATWEEL.len_utf8() } else { 0 };
            let mut text = String::new();
            if before {
                text.push(TATWEEL);
            }
            text.push_str(&letters);
            if after {
                text.push(TATWEEL);
            }
            (text, lead..lead + letters.len())
        }
        _ => {
            let text: String = unit.iter().collect();
            let all = 0..text.len();
            (text, all)
        }
    };
    let glyphs: Vec<Shaped> = shape(ctx, font, &text)
        .into_iter()
        .filter(|(source, _)| keep.contains(&source.start))
        .flat_map(|(_, glyphs)| glyphs)
        .collect();
    if glyphs.is_empty() || glyphs.iter().any(|g| g.id == 0) {
        return Err(format!("{file} has no glyph for {}", name()));
    }
    let advance = glyphs.iter().map(|g| g.advance).sum();
    Ok((glyphs, advance))
}

pub fn role_bit(role: Role) -> u8 {
    let index = spec::ROLES.iter().position(|&r| r == role).unwrap_or(0);
    1 << index
}

/// Coverage quantized to 16 levels, stored a byte per pixel. Sixteen
/// levels are plenty for hinted text, and the fewer distinct values let
/// deflate shrink a strike to about half of a 4-bit run-length code.
fn quantize(alpha: &[u8]) -> impl Iterator<Item = u8> + '_ {
    alpha
        .iter()
        .map(|&a| ((u32::from(a) * 15 + 127) / 255 * 17) as u8)
}

fn put<T: TryFrom<usize>>(n: usize, what: &str) -> Result<T, String> {
    T::try_from(n).map_err(|_| format!("{what} {n} does not fit the format"))
}

/// A face's glyphs: what each draws, how, and its advance in font units.
type Glyphs = Vec<(Unit, Vec<Shaped>, f32)>;

/// A rendered glyph: size, placement from the pen (left, and top above the
/// baseline), and coverage.
struct Bitmap {
    width: usize,
    height: usize,
    left: i32,
    top: i32,
    alpha: Vec<u8>,
}

/// Renders `glyphs` together, each at its shaped offset, into one bitmap.
/// A single glyph renders exactly as the scaler draws it.
fn render(sc: &mut Scaler<'_>, glyphs: &[Shaped], scale: f32) -> Option<Bitmap> {
    let mut parts = Vec::new();
    let mut pen = 0.0;
    for g in glyphs {
        let offset = Vector::new((pen + g.x) * scale, g.y * scale);
        let image = Render::new(&[Source::Outline])
            .format(Format::Alpha)
            .offset(offset)
            .render(sc, g.id)
            .filter(|i| {
                i.content == Content::Mask && i.placement.width > 0 && i.placement.height > 0
            });
        if let Some(image) = image {
            parts.push(image);
        }
        pen += g.advance;
    }
    let left = parts.iter().map(|i| i.placement.left).min()?;
    let top = parts.iter().map(|i| i.placement.top).max()?;
    let right = parts
        .iter()
        .map(|i| i.placement.left + i.placement.width as i32)
        .max()?;
    let bottom = parts
        .iter()
        .map(|i| i.placement.top - i.placement.height as i32)
        .min()?;
    let (width, height) = ((right - left) as usize, (top - bottom) as usize);
    let mut alpha = vec![0_u8; width * height];
    for image in &parts {
        let p = image.placement;
        let (dx, dy) = ((p.left - left) as usize, (top - p.top) as usize);
        for row in 0..p.height as usize {
            for col in 0..p.width as usize {
                let a = image.data[row * p.width as usize + col];
                let at = &mut alpha[(dy + row) * width + dx + col];
                *at = (*at).max(a);
            }
        }
    }
    Some(Bitmap {
        width,
        height,
        left,
        top,
        alpha,
    })
}

pub fn group(group: &Group, sets: &Sets, fonts: &BTreeMap<&str, Vec<u8>>) -> Result<Baked, String> {
    let mut bytes = b"ARKG".to_vec();
    bytes.push(2);
    let mut report = String::new();
    let weights: Vec<Weight> = Weight::ALL
        .into_iter()
        .filter(|&w| styles(sets, w).next().is_some())
        .collect();
    bytes.push(put(weights.len(), "face count")?);
    let mut shaper = ShapeContext::new();
    let mut scaler = ScaleContext::new();
    for weight in weights {
        let file = group.fonts[weight as usize];
        let data = fonts
            .get(file)
            .ok_or_else(|| format!("{file} not fetched"))?;
        let font = FontRef::from_index(data, 0).ok_or_else(|| format!("{file} is not a font"))?;
        let units: BTreeSet<Unit> = styles(sets, weight)
            .flat_map(|s| sets.units[&s].iter().cloned())
            .collect();
        let mut glyphs: Glyphs = Vec::new();
        for unit in &units {
            let (shaped, advance) = recipe(&mut shaper, font, file, unit)?;
            glyphs.push((unit.clone(), shaped, advance));
        }
        let kerns = kerns(&mut shaper, font, &glyphs, sets.pairs.get(&weight))?;
        let key = units.iter().map(Vec::len).max().unwrap_or(1);
        if key > script::CLUSTER {
            return Err(format!("{file}: a {key}-character cluster is too long"));
        }

        let metrics = font.metrics(&[]);
        bytes.push(weight as u8);
        bytes.push(put(key, "key length")?);
        bytes.extend(metrics.units_per_em.to_le_bytes());
        for v in [
            metrics.ascent,
            -metrics.descent.abs(),
            metrics.cap_height,
            metrics.x_height,
        ] {
            bytes.extend((v.round() as i16).to_le_bytes());
        }
        bytes.extend(put::<u16>(glyphs.len(), "glyph count")?.to_le_bytes());
        for (unit, _, _) in &glyphs {
            for k in 0..key {
                bytes.extend(unit.get(k).map_or(0, |&c| c as u32).to_le_bytes());
            }
        }
        for (_, _, advance) in &glyphs {
            bytes.extend((advance.round() as u16).to_le_bytes());
        }
        bytes.extend(put::<u16>(kerns.len(), "kern count")?.to_le_bytes());
        for (a, b, k) in &kerns {
            bytes.extend(a.to_le_bytes());
            bytes.extend(b.to_le_bytes());
            bytes.extend(k.to_le_bytes());
        }
        let _ = writeln!(
            report,
            "{} {weight:?}: {} glyphs, {} kerning pairs, upem {}",
            group.name,
            glyphs.len(),
            kerns.len(),
            metrics.units_per_em
        );

        // Each ladder size a role in this weight needs, holding the union
        // of those roles' glyphs.
        let mut strikes: BTreeMap<u8, (u8, BTreeSet<Unit>)> = BTreeMap::new();
        for style in styles(sets, weight) {
            for ppem in spec::rungs(style.0) {
                let strike = strikes.entry(ppem).or_default();
                strike.0 |= role_bit(style.0);
                strike.1.extend(sets.units[&style].iter().cloned());
            }
        }
        bytes.push(put(strikes.len(), "strike count")?);
        let scale_of = |ppem: u8| f32::from(ppem) / f32::from(metrics.units_per_em);
        for (&ppem, (roles, members)) in &strikes {
            let mut sc = scaler
                .builder(font)
                .size(f32::from(ppem))
                .hint(true)
                .build();
            // Entries and pixels share one deflate stream; each glyph's
            // pixels follow the previous one's, so no offsets are stored.
            let mut entries = Vec::new();
            let mut pixels = Vec::new();
            let mut count = 0;
            for (index, (unit, shaped, _)) in glyphs.iter().enumerate() {
                if !members.contains(unit) {
                    continue;
                }
                let (w, h, left, top) = match render(&mut sc, shaped, scale_of(ppem)) {
                    Some(b) => {
                        pixels.extend(quantize(&b.alpha));
                        (b.width, b.height, b.left, b.top)
                    }
                    // Spaces: an advance and nothing to draw.
                    None => (0, 0, 0, 0),
                };
                entries.extend(put::<u16>(index, "glyph")?.to_le_bytes());
                entries.push(put(w, "width")?);
                entries.push(put(h, "height")?);
                entries.push(
                    i8::try_from(left).map_err(|_| format!("left {left} at {ppem} px"))? as u8,
                );
                entries
                    .push(i8::try_from(top).map_err(|_| format!("top {top} at {ppem} px"))? as u8);
                count += 1;
            }
            let mut unpacked = put::<u16>(count, "strike glyphs")?.to_le_bytes().to_vec();
            unpacked.extend(&entries);
            unpacked.extend(&pixels);
            let packed = compress_to_vec(&unpacked, 10);
            bytes.push(ppem);
            bytes.extend(put::<u32>(unpacked.len(), "strike size")?.to_le_bytes());
            bytes.extend(put::<u32>(packed.len(), "packed size")?.to_le_bytes());
            bytes.extend(&packed);
            let _ = writeln!(
                report,
                "  {ppem:>2} px {:<22} {count:>4} glyphs {:>7} B unpacked {:>7} B deflated",
                role_names(*roles),
                unpacked.len(),
                packed.len(),
            );
        }
    }
    Ok(Baked { bytes, report })
}

/// The styles set in `weight` that have glyphs in this group.
fn styles(sets: &Sets, weight: Weight) -> impl Iterator<Item = Style> + '_ {
    sets.units
        .iter()
        .filter(move |(s, c)| s.1 == weight && !c.is_empty())
        .map(|(&s, _)| s)
}

/// GPOS pair kerning, in font units, for the pairs that occur: how far the
/// second glyph sits from where the first's plain advance would put it.
fn kerns(
    shaper: &mut ShapeContext,
    font: FontRef<'_>,
    glyphs: &Glyphs,
    pairs: Option<&BTreeSet<(char, char)>>,
) -> Result<Vec<(u16, u16, i16)>, String> {
    let find = |c: char| glyphs.iter().position(|g| g.0 == [c]);
    let mut out = Vec::new();
    for &(a, b) in pairs.into_iter().flatten() {
        let (Some(ia), Some(ib)) = (find(a), find(b)) else {
            continue;
        };
        let (ga, plain) = (glyphs[ia].1[0].id, glyphs[ia].2);
        let gb = glyphs[ib].1[0].id;
        let pair: String = [a, b].iter().collect();
        let shaped: Vec<Shaped> = shape(shaper, font, &pair)
            .into_iter()
            .flat_map(|(_, g)| g)
            .collect();
        if let [first, second] = shaped.as_slice()
            && (first.id, second.id) == (ga, gb)
        {
            let kern = (first.advance + second.x - plain).round();
            if kern != 0.0 {
                out.push((put(ia, "glyph")?, put(ib, "glyph")?, kern as i16));
            }
        }
    }
    Ok(out)
}

fn role_names(mask: u8) -> String {
    spec::ROLES
        .iter()
        .filter(|&&r| mask & role_bit(r) != 0)
        .map(|r| format!("{r:?}"))
        .collect::<Vec<_>>()
        .join("+")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_keeps_sixteen_levels_and_both_ends() {
        let levels: Vec<u8> = quantize(&[0, 8, 9, 128, 238, 255]).collect();
        assert_eq!(levels, [0, 0, 17, 136, 238, 255]);
        let every: Vec<u8> = (0..=255).collect();
        let distinct: BTreeSet<u8> = quantize(&every).collect();
        assert_eq!(distinct.len(), 16);
    }
}

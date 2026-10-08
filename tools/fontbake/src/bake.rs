//! Shapes, rasterizes and encodes one group's faces. The byte layout is
//! documented in `crates/ark-glyphs/src/data.rs`, which reads it.
use crate::{
    charsets::{Group, Sets},
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
    scale::{Render, ScaleContext, Source, image::Content},
    shape::ShapeContext,
    text::{Codepoint, Script},
    zeno::Format,
};

pub struct Baked {
    pub bytes: Vec<u8>,
    pub report: String,
}

/// Shaping features: kerning and tabular figures on, so digits never shift
/// as values change; ligatures and contextual forms off, because the game
/// draws one glyph per character.
const FEATURES: [(&str, u16); 5] = [
    ("kern", 1),
    ("tnum", 1),
    ("liga", 0),
    ("clig", 0),
    ("calt", 0),
];

/// Glyph ids, x offsets and advances, in font units.
fn shape(ctx: &mut ShapeContext, font: FontRef<'_>, s: &str) -> Vec<(GlyphId, f32, f32)> {
    let script = s
        .chars()
        .map(|c| c.script())
        .find(|s| !matches!(s, Script::Common | Script::Inherited | Script::Unknown))
        .unwrap_or(Script::Latin);
    let mut shaper = ctx.builder(font).script(script).features(FEATURES).build();
    shaper.add_str(s);
    let mut glyphs = Vec::new();
    shaper.shape_with(|cluster| {
        glyphs.extend(cluster.glyphs.iter().map(|g| (g.id, g.x, g.advance)));
    });
    glyphs
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

/// A face's characters with their glyphs and advances, in font units.
type Glyphs = Vec<(char, GlyphId, f32)>;

pub fn group(group: &Group, sets: &Sets, fonts: &BTreeMap<&str, Vec<u8>>) -> Result<Baked, String> {
    let mut bytes = b"ARKG".to_vec();
    bytes.push(1);
    let mut report = String::new();
    let weights: Vec<Weight> = Weight::ALL
        .into_iter()
        .filter(|&w| roles(sets, w).next().is_some())
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
        let chars: BTreeSet<char> = roles(sets, weight)
            .flat_map(|r| sets.chars[&r].iter().copied())
            .collect();
        let glyphs = glyphs(&mut shaper, font, file, &chars)?;
        let kerns = kerns(&mut shaper, font, &glyphs, sets.pairs.get(&weight))?;

        let metrics = font.metrics(&[]);
        bytes.push(weight as u8);
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
        for &(c, _, _) in &glyphs {
            bytes.extend((c as u32).to_le_bytes());
        }
        for &(_, _, advance) in &glyphs {
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
        // of those roles' characters.
        let mut strikes: BTreeMap<u8, (u8, BTreeSet<char>)> = BTreeMap::new();
        for role in roles(sets, weight) {
            for ppem in spec::rungs(role) {
                let strike = strikes.entry(ppem).or_default();
                strike.0 |= role_bit(role);
                strike.1.extend(sets.chars[&role].iter().copied());
            }
        }
        bytes.push(put(strikes.len(), "strike count")?);
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
            for (index, &(c, id, _)) in glyphs.iter().enumerate() {
                if !members.contains(&c) {
                    continue;
                }
                let image = Render::new(&[Source::Outline])
                    .format(Format::Alpha)
                    .render(&mut sc, id)
                    .filter(|i| {
                        i.content == Content::Mask
                            && i.placement.width > 0
                            && i.placement.height > 0
                    });
                let (w, h, left, top) = match &image {
                    Some(i) => {
                        pixels.extend(quantize(&i.data));
                        let p = i.placement;
                        (p.width as usize, p.height as usize, p.left, p.top)
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
                "  {ppem:>2} px {:<15} {count:>4} glyphs {:>7} B unpacked {:>7} B deflated",
                role_names(*roles),
                unpacked.len(),
                packed.len(),
            );
        }
    }
    Ok(Baked { bytes, report })
}

/// The roles set in `weight` that have characters in this group.
fn roles(sets: &Sets, weight: Weight) -> impl Iterator<Item = Role> + '_ {
    spec::ROLES.into_iter().filter(move |r| {
        spec::style(*r).1 == weight && sets.chars.get(r).is_some_and(|c| !c.is_empty())
    })
}

/// One glyph and advance per character, each shaped alone.
fn glyphs(
    shaper: &mut ShapeContext,
    font: FontRef<'_>,
    file: &str,
    chars: &BTreeSet<char>,
) -> Result<Glyphs, String> {
    let mut out = Vec::new();
    for &c in chars {
        match shape(shaper, font, c.encode_utf8(&mut [0; 4])).as_slice() {
            &[(id, _, advance)] if id != 0 => out.push((c, id, advance)),
            [(0, _, _)] | [] => {
                return Err(format!("{file} has no glyph for U+{:04X} {c:?}", c as u32));
            }
            more => {
                return Err(format!(
                    "{file}: U+{:04X} shapes to {} glyphs",
                    c as u32,
                    more.len()
                ));
            }
        }
    }
    Ok(out)
}

/// GPOS pair kerning, in font units, for the pairs that occur: how far the
/// second glyph sits from where the first's plain advance would put it.
fn kerns(
    shaper: &mut ShapeContext,
    font: FontRef<'_>,
    glyphs: &Glyphs,
    pairs: Option<&BTreeSet<(char, char)>>,
) -> Result<Vec<(u16, u16, i16)>, String> {
    let find = |c: char| glyphs.iter().position(|g| g.0 == c);
    let mut out = Vec::new();
    for &(a, b) in pairs.into_iter().flatten() {
        let (Some(ia), Some(ib)) = (find(a), find(b)) else {
            continue;
        };
        let (ga, plain) = (glyphs[ia].1, glyphs[ia].2);
        let gb = glyphs[ib].1;
        let pair: String = [a, b].iter().collect();
        if let [(pa, _, advance), (pb, x, _)] = shape(shaper, font, &pair).as_slice()
            && (*pa, *pb) == (ga, gb)
        {
            let kern = (advance + x - plain).round();
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

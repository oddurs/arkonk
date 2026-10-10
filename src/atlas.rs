//! The one texture the renderer samples: a white cell for shapes, the 5×7
//! pixel font, the pieces' glow and capsule pictograms, and the baked Noto
//! strikes of the current locale that the screen's density sets text in,
//! packed on shelves. Shapes, light and text share it, so a frame stays
//! one batch. Built on the CPU, so the layout tests can use it without a
//! window.
use crate::{pictogram, pixel_font};
use ark::Power;
use ark_glyphs::{
    Error, Fonts, Source,
    spec::{LADDER, Weight},
};
use std::{collections::HashMap, sync::OnceLock};

/// The largest texture side GLES2-class GPUs commonly accept, which the
/// game must still run on.
const WIDTH: usize = 2048;
/// Transparent pixels between glyphs, so no sample ever reaches a neighbour.
const PAD: usize = 1;

/// Where a glyph's bitmap sits in the atlas, and how it sits on the pen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub x: u16,
    pub y: u16,
    pub w: u8,
    pub h: u8,
    pub left: i8,
    pub top: i8,
}

pub type Key = (Source, Weight, u8, u16);

/// Which baked strikes an atlas holds: for each cut, a bit per ladder
/// size. Every density's sizes together would not fit one texture in
/// Chinese, and one scale factor maps the scene to the screen, so an atlas
/// holds only what the current density sets text in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Strikes([u32; Weight::ALL.len()]);

impl Strikes {
    /// Adds `weight` at `ppem`; a size off the ladder was never baked.
    pub fn add(&mut self, weight: Weight, ppem: u8) {
        if let Some(i) = LADDER.iter().position(|&r| r == ppem) {
            self.0[weight as usize] |= 1 << i;
        }
    }

    pub fn has(&self, weight: Weight, ppem: u8) -> bool {
        LADDER
            .iter()
            .position(|&r| r == ppem)
            .is_some_and(|i| self.0[weight as usize] & (1 << i) != 0)
    }
}

pub struct Atlas {
    pub width: usize,
    pub height: usize,
    /// Coverage, one byte per pixel.
    pub alpha: Vec<u8>,
    strikes: Strikes,
    glyphs: HashMap<Key, Cell>,
    /// The radial glow every piece casts.
    pub glow: Cell,
    /// Each power's pictogram at every height from [`pictogram::MIN_H`].
    icons: [[Cell; pictogram::HEIGHTS]; Power::ALL.len()],
}

/// What a bitmap in the atlas is, which also orders bitmaps of the same
/// height, so packing is the same on every run.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Pixel,
    Glow,
    Icon(usize, u8),
    Glyph(Key),
}

/// One bitmap to pack, borrowing its pixels.
#[derive(Clone, Copy)]
struct Bitmap<'a> {
    kind: Kind,
    w: usize,
    h: usize,
    left: i8,
    top: i8,
    pixels: &'a [u8],
}

/// The bitmaps every atlas holds besides glyphs: the pixel font first,
/// the glow and every pictogram. Rasterized once, so repacking for a new
/// density or locale only inflates and places strikes.
fn fixed() -> &'static [Bitmap<'static>] {
    static FIXED: OnceLock<Vec<Bitmap<'static>>> = OnceLock::new();
    FIXED.get_or_init(|| {
        // Held for the whole run, as the cache itself is.
        let leak = |pixels: Vec<u8>| -> &'static [u8] { pixels.leak() };
        let mut out = vec![
            Bitmap {
                kind: Kind::Pixel,
                w: pixel_font::ATLAS_W,
                h: pixel_font::ATLAS_H,
                left: 0,
                top: 0,
                pixels: leak(pixel_font::atlas()),
            },
            Bitmap {
                kind: Kind::Glow,
                w: pictogram::GLOW,
                h: pictogram::GLOW,
                left: 0,
                top: 0,
                pixels: leak(pictogram::glow()),
            },
        ];
        for (p, &power) in Power::ALL.iter().enumerate() {
            for h in pictogram::MIN_H..=pictogram::MAX_H {
                let (w, pixels) = pictogram::raster(power, h);
                out.push(Bitmap {
                    kind: Kind::Icon(p, h),
                    w,
                    h: usize::from(h),
                    left: 0,
                    top: 0,
                    pixels: leak(pixels),
                });
            }
        }
        out
    })
}

impl Atlas {
    /// The pixel font's 128 × 48 block sits at the origin; its DEL cell is
    /// solid white, and shapes sample its centre.
    pub const WHITE: (f32, f32) = (124.5, 44.5);

    /// Packs the fixed bitmaps and the `strikes` of `fonts`.
    pub fn build(fonts: &Fonts, strikes: Strikes) -> Result<Self, Error> {
        let mut inflated = Vec::new();
        for (source, font) in [
            (Source::Latin, Some(&fonts.latin)),
            (Source::Local, fonts.local.as_ref()),
        ] {
            for face in font.iter().flat_map(|f| f.faces()) {
                for strike in face.strikes().filter(|s| strikes.has(face.weight, s.ppem)) {
                    let buffer = vec![0; strike.unpacked_len()];
                    inflated.push((source, face, strike, buffer));
                }
            }
        }
        let mut bitmaps: Vec<Bitmap> = fixed().to_vec();
        for (source, face, strike, buffer) in &mut inflated {
            // Noto Sans holds every locale's letters; only this one's are packed.
            let images = strike.unpack(buffer)?;
            for image in images.filter(|i| face.serves(i.glyph, fonts.locale)) {
                bitmaps.push(Bitmap {
                    kind: Kind::Glyph((*source, face.weight, strike.ppem, image.glyph)),
                    w: usize::from(image.width),
                    h: usize::from(image.height),
                    left: image.left,
                    top: image.top,
                    pixels: image.pixels,
                });
            }
        }
        // Tallest first packs shelves tightly; the kind keeps the order, and
        // so the texture, the same on every run. The pixel font stays first.
        bitmaps[1..].sort_by(|a, b| b.h.cmp(&a.h).then(a.kind.cmp(&b.kind)));
        let (mut x, mut y, mut shelf) = (0, 0, 0);
        let mut placed = Vec::with_capacity(bitmaps.len());
        for b in &bitmaps {
            if x + b.w > WIDTH {
                (x, y, shelf) = (0, y + shelf + PAD, 0);
            }
            placed.push((x, y));
            x += b.w + PAD;
            shelf = shelf.max(b.h);
        }
        let height = (y + shelf).next_multiple_of(64);
        let mut alpha = vec![0; WIDTH * height];
        let mut glyphs = HashMap::with_capacity(bitmaps.len());
        let empty = Cell {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            left: 0,
            top: 0,
        };
        let mut glow = empty;
        let mut icons = [[empty; pictogram::HEIGHTS]; Power::ALL.len()];
        for (b, &(x, y)) in bitmaps.iter().zip(&placed) {
            for row in 0..b.h {
                let at = (y + row) * WIDTH + x;
                alpha[at..at + b.w].copy_from_slice(&b.pixels[row * b.w..(row + 1) * b.w]);
            }
            // Both fit: the atlas is 2048 wide and glyphs under 256 px.
            let cell = Cell {
                x: x as u16,
                y: y as u16,
                w: b.w as u8,
                h: b.h as u8,
                left: b.left,
                top: b.top,
            };
            match b.kind {
                Kind::Pixel => {}
                Kind::Glow => glow = cell,
                Kind::Icon(p, h) => icons[p][usize::from(h - pictogram::MIN_H)] = cell,
                Kind::Glyph(key) => {
                    glyphs.insert(key, cell);
                }
            }
        }
        Ok(Self {
            width: WIDTH,
            height,
            alpha,
            strikes,
            glyphs,
            glow,
            icons,
        })
    }

    pub fn strikes(&self) -> Strikes {
        self.strikes
    }

    pub fn glyph(&self, key: Key) -> Option<Cell> {
        self.glyphs.get(&key).copied()
    }

    /// `power`'s pictogram baked nearest `h` pixels tall.
    pub fn icon(&self, power: Power, h: f32) -> Cell {
        let h = h
            .round()
            .clamp(f32::from(pictogram::MIN_H), f32::from(pictogram::MAX_H));
        let p = Power::ALL.iter().position(|&p| p == power).unwrap_or(0);
        self.icons[p][h as usize - usize::from(pictogram::MIN_H)]
    }

    /// White with the coverage as alpha, ready to upload.
    pub fn rgba(&self) -> Vec<u8> {
        self.alpha
            .iter()
            .flat_map(|&a| [255, 255, 255, a])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{rungs_at, sizes, strikes_at};
    use ark_glyphs::spec;
    use ark_text::{Arg, Form, Locale, Role, TextId, icon_power};
    use std::collections::{BTreeMap, BTreeSet};

    /// The most rows an atlas may have up to 1080p ([`SMALL`]): a square
    /// texture, which GLES2-class GPUs on small machines accept.
    const MOST_ROWS_SMALL: usize = 2048;
    /// The density of a 1080p screen, the densest such machines drive.
    const SMALL: f32 = 1.2;
    /// The most rows on denser screens, whose GPUs all take 4096.
    const MOST_ROWS: usize = 4096;

    fn most_rows(density: f32) -> usize {
        if density <= SMALL {
            MOST_ROWS_SMALL
        } else {
            MOST_ROWS
        }
    }

    /// One density inside every span where the packed strikes stay the
    /// same: either side of each density where a size the renderer sets
    /// moves to another ladder entry, plus the checked displays.
    fn densities() -> Vec<f32> {
        let mut out = spec::DENSITIES.to_vec();
        for (_, size) in sizes() {
            for pair in spec::LADDER.windows(2) {
                // `spec::nearest` switches where the ratios to both are equal.
                let switch = (f32::from(pair[0]) * f32::from(pair[1])).sqrt() / size;
                out.extend([switch * 0.999, switch * 1.001]);
            }
        }
        out.sort_by(f32::total_cmp);
        out.dedup();
        out
    }

    /// Each distinct set of strikes the renderer packs, at the lowest of
    /// [`densities`] that needs it, where its row limit is strictest.
    fn packings() -> Vec<(f32, Strikes)> {
        let mut out: Vec<(f32, Strikes)> = Vec::new();
        for d in densities() {
            let strikes = strikes_at(d);
            if out.iter().all(|&(_, s)| s != strikes) {
                out.push((d, strikes));
            }
        }
        out
    }

    fn drawable() -> impl Iterator<Item = Locale> {
        Locale::ALL.into_iter().filter(|&l| ark_glyphs::supports(l))
    }

    /// Runs `check` for every drawable locale on its own thread.
    fn each_locale(check: impl Fn(Locale) + Sync) {
        std::thread::scope(|threads| {
            let check = &check;
            let runs: Vec<_> = drawable()
                .map(|l| threads.spawn(move || check(l)))
                .collect();
            for run in runs {
                run.join().expect("a locale's check failed");
            }
        });
    }

    #[test]
    fn every_strike_packs_without_overlap_and_the_white_cell_is_white() {
        let packings = packings();
        each_locale(|locale| {
            let fonts = ark_glyphs::fonts(locale).unwrap();
            for &(density, strikes) in &packings {
                let atlas = Atlas::build(&fonts, strikes).unwrap();
                assert!(
                    atlas.height <= most_rows(density),
                    "{locale:?} at {density:.3}: {} rows",
                    atlas.height
                );
                let (wx, wy) = Atlas::WHITE;
                assert_eq!(atlas.alpha[wy as usize * atlas.width + wx as usize], 255);
                let mut cells: Vec<_> = atlas.glyphs.values().filter(|c| c.w > 0).collect();
                cells.sort_by_key(|c| (c.y, c.x));
                for pair in cells.windows(2) {
                    let (a, b) = (pair[0], pair[1]);
                    let apart = a.y != b.y || u32::from(a.x) + u32::from(a.w) < u32::from(b.x);
                    assert!(apart || a.y + u16::from(a.h) <= b.y, "{a:?} overlaps {b:?}");
                }
                let (gx, gy) = (usize::from(atlas.glow.x), usize::from(atlas.glow.y));
                let middle = (gy + pictogram::GLOW / 2) * atlas.width + gx + pictogram::GLOW / 2;
                assert!(atlas.alpha[middle] > 240, "the glow is packed");
                for power in Power::ALL {
                    let small = atlas.icon(power, 0.0);
                    let large = atlas.icon(power, 99.0);
                    assert_eq!(u16::from(small.h), u16::from(pictogram::MIN_H));
                    assert_eq!(u16::from(large.h), u16::from(pictogram::MAX_H));
                }
            }
            let (_, largest) = packings[packings.len() - 1];
            let once = Atlas::build(&fonts, largest).unwrap();
            let again = Atlas::build(&fonts, largest).unwrap();
            assert!(once.alpha == again.alpha, "packing must be deterministic");
        });
    }

    /// The glyphs each role and cut sets for `locale`: every string, full
    /// and short, in its own role, each role [`TextId::also`] names and, for
    /// text that may be emphasised, body text's strong cut; and the
    /// language's own name, which the Settings sheet sets as body text and
    /// as a caption. A glyph the fonts lack fails here.
    fn glyphs(locale: Locale, fonts: &Fonts) -> BTreeMap<(Role, Weight), BTreeSet<(Source, u16)>> {
        let mut out: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
        let mut add = |text: &str, role: Role, weight: Weight| {
            let set = out.entry((role, weight)).or_default();
            fonts.layout(text, weight, 20, 0.0, |p| {
                if icon_power(p.c).is_some() {
                    return;
                }
                let glyph = p.glyph.unwrap_or_else(|| {
                    panic!("{locale:?}: no {weight:?} glyph for {:?} in {text:?}", p.c)
                });
                set.insert((p.source, glyph));
            });
        };
        for id in TextId::all() {
            let args = vec![Arg::Count(1_234_567); id.arity()];
            let mut styles: Vec<(Role, Weight)> =
                id.roles().map(|r| (r, spec::style(r).1)).collect();
            if id.strong() {
                styles.push((Role::Body, spec::strong(Role::Body)));
            }
            for form in [Form::Full, Form::Short] {
                let mut text = String::new();
                ark_text::write_icons(&mut text, locale, form, id, &args).unwrap();
                for &(role, weight) in &styles {
                    add(&text, role, weight);
                }
            }
        }
        if locale != Locale::Pseudo {
            for role in [Role::Body, Role::Caption] {
                add(locale.native_name(), role, Weight::Regular);
            }
        }
        out
    }

    /// No missing glyphs: at every density, every string has each of its
    /// glyphs in the atlas in every role it is set in, at every size the
    /// renderer sets that role at there and the size the fit chain steps
    /// down to.
    #[test]
    fn every_string_has_its_glyphs_in_every_role_at_every_density() {
        let densities = densities();
        each_locale(|locale| {
            let fonts = ark_glyphs::fonts(locale).unwrap();
            let glyphs = glyphs(locale, &fonts);
            let mut atlas: Option<Atlas> = None;
            for &d in &densities {
                let strikes = strikes_at(d);
                if atlas.as_ref().is_none_or(|a| a.strikes() != strikes) {
                    atlas = Some(Atlas::build(&fonts, strikes).unwrap());
                }
                let atlas = atlas.as_ref().unwrap();
                for (role, weight, ppem) in rungs_at(d) {
                    if let Some(set) = glyphs.get(&(role, weight)) {
                        for &(source, glyph) in set {
                            assert!(
                                atlas.glyph((source, weight, ppem, glyph)).is_some(),
                                "{locale:?} at {d:.3}: {role:?} {weight:?} {ppem} px lacks {source:?} glyph {glyph}"
                            );
                        }
                    }
                }
            }
        });
    }

    /// Prints each locale's atlas height at the 960 × 900 window, 1080p,
    /// Retina and 4K densities, the tallest up to 1080p and the tallest at
    /// any density, for the tables in `docs/localization.md`.
    #[test]
    #[ignore = "a report, not a check"]
    fn report_atlas_heights() {
        let packings = packings();
        for locale in drawable() {
            let fonts = ark_glyphs::fonts(locale).unwrap();
            let rows = |strikes| Atlas::build(&fonts, strikes).unwrap().height;
            let heights: Vec<(f32, usize)> = packings.iter().map(|&(d, s)| (d, rows(s))).collect();
            let small = heights.iter().filter(|h| h.0 <= SMALL).map(|h| h.1).max();
            let (at, most) = heights.iter().copied().max_by_key(|&(_, h)| h).unwrap();
            let some: Vec<String> = [1.0, 1.2, 2.0, 2.4]
                .iter()
                .map(|&d| format!("{d:.1}: {:>4}", rows(strikes_at(d))))
                .collect();
            println!(
                "{:<8} {}  most to 1.2: {:>4}  most: {most:>4} at {at:.3}",
                locale.tag(),
                some.join("  "),
                small.unwrap_or(0)
            );
        }
    }
}

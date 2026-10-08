//! The one texture the renderer samples: a white cell for shapes, the 5×7
//! pixel font, and every baked Noto strike of the current locale, packed
//! on shelves. Shapes and text share it, so a frame stays one batch. Built
//! on the CPU, so the layout tests can use it without a window.
use crate::pixel_font;
use ark_glyphs::{Error, Fonts, Source, spec::Weight};
use std::collections::HashMap;

/// Wide enough that the Latin and CJK strikes pack under 2048 rows.
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

pub struct Atlas {
    pub width: usize,
    pub height: usize,
    /// Coverage, one byte per pixel.
    pub alpha: Vec<u8>,
    glyphs: HashMap<Key, Cell>,
}

impl Atlas {
    /// The pixel font's 128 × 48 block sits at the origin; its DEL cell is
    /// solid white, and shapes sample its centre.
    pub const WHITE: (f32, f32) = (124.5, 44.5);

    pub fn build(fonts: &Fonts) -> Result<Self, Error> {
        struct Bitmap {
            key: Option<Key>,
            w: usize,
            h: usize,
            left: i8,
            top: i8,
            pixels: Vec<u8>,
        }
        let mut bitmaps = vec![Bitmap {
            key: None,
            w: pixel_font::ATLAS_W,
            h: pixel_font::ATLAS_H,
            left: 0,
            top: 0,
            pixels: pixel_font::atlas(),
        }];
        for (source, font) in [
            (Source::Latin, Some(fonts.latin)),
            (Source::Local, fonts.local),
        ] {
            for face in font.iter().flat_map(|f| f.faces()) {
                for strike in face.strikes() {
                    let mut buffer = vec![0; strike.unpacked_len()];
                    for image in strike.unpack(&mut buffer)? {
                        bitmaps.push(Bitmap {
                            key: Some((source, face.weight, strike.ppem, image.glyph)),
                            w: usize::from(image.width),
                            h: usize::from(image.height),
                            left: image.left,
                            top: image.top,
                            pixels: image.pixels.to_vec(),
                        });
                    }
                }
            }
        }
        // Tallest first packs shelves tightly; the key keeps the order, and
        // so the texture, the same on every run. The pixel font stays first.
        bitmaps[1..].sort_by(|a, b| b.h.cmp(&a.h).then(a.key.cmp(&b.key)));
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
        for (b, &(x, y)) in bitmaps.iter().zip(&placed) {
            for row in 0..b.h {
                let at = (y + row) * WIDTH + x;
                alpha[at..at + b.w].copy_from_slice(&b.pixels[row * b.w..(row + 1) * b.w]);
            }
            if let Some(key) = b.key {
                // Both fit: the atlas is 2048 wide and glyphs under 256 px.
                let cell = Cell {
                    x: x as u16,
                    y: y as u16,
                    w: b.w as u8,
                    h: b.h as u8,
                    left: b.left,
                    top: b.top,
                };
                glyphs.insert(key, cell);
            }
        }
        Ok(Self {
            width: WIDTH,
            height,
            alpha,
            glyphs,
        })
    }

    pub fn glyph(&self, key: Key) -> Option<Cell> {
        self.glyphs.get(&key).copied()
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
    use ark_text::Locale;

    #[test]
    fn every_strike_packs_without_overlap_and_the_white_cell_is_white() {
        for locale in [Locale::En, Locale::ZhHans] {
            if !ark_glyphs::supports(locale) {
                continue;
            }
            let atlas = Atlas::build(&ark_glyphs::fonts(locale).unwrap()).unwrap();
            assert!(atlas.height <= 4096, "{locale:?}: {} rows", atlas.height);
            let (wx, wy) = Atlas::WHITE;
            assert_eq!(atlas.alpha[wy as usize * atlas.width + wx as usize], 255);
            let mut cells: Vec<_> = atlas.glyphs.values().filter(|c| c.w > 0).collect();
            cells.sort_by_key(|c| (c.y, c.x));
            for pair in cells.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                let apart = a.y != b.y || u32::from(a.x) + u32::from(a.w) < u32::from(b.x);
                assert!(apart || a.y + u16::from(a.h) <= b.y, "{a:?} overlaps {b:?}");
            }
            let again = Atlas::build(&ark_glyphs::fonts(locale).unwrap()).unwrap();
            assert!(again.alpha == atlas.alpha, "packing must be deterministic");
        }
    }
}

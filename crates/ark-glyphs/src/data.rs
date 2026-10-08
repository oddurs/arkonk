//! Reads the baked atlas files. Every read is bounds-checked; damaged data
//! is an [`Error`], never a panic.
//!
//! Layout, little-endian throughout:
//!
//! ```text
//! file    "ARKG" version:u8=1 faces:u8 face…
//! face    weight:u8 upem:u16 ascent:i16 descent:i16 cap_height:i16 x_height:i16
//!         glyphs:u16 chars:[u32; glyphs] advances:[u16; glyphs]   (sorted by char)
//!         kerns:u16 [left:u16 right:u16 value:i16; kerns]          (sorted by pair)
//!         strikes:u8 strike…
//! strike  ppem:u8 unpacked:u32 packed:u32 deflate(unpacked)[packed]
//! unpacked  count:u16 [glyph:u16 w:u8 h:u8 left:i8 top:i8; count] pixels
//! ```
//!
//! Glyph indices count into the face's `chars`. Each glyph's pixels are
//! `w × h` coverage bytes, row by row, following the previous glyph's.
//! Advances and kerning are in font units; `left` and `top` place the
//! bitmap relative to the pen on the baseline, in pixels, `top` upward.
use crate::spec::Weight;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Not an atlas file, or a version this code does not read.
    Header,
    /// The data ends early or a count does not add up.
    Truncated,
    /// A strike's deflate stream is damaged.
    Inflate,
}

/// Reads little-endian values from a slice, failing instead of panicking.
#[derive(Clone, Copy)]
struct Reader<'a> {
    bytes: &'a [u8],
}
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if n > self.bytes.len() {
            return Err(Error::Truncated);
        }
        let (head, tail) = self.bytes.split_at(n);
        self.bytes = tail;
        Ok(head)
    }
    fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        let mut out = [0; N];
        out.copy_from_slice(self.take(N)?);
        Ok(out)
    }
    fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.array::<1>()?[0])
    }
    fn u16(&mut self) -> Result<u16, Error> {
        self.array().map(u16::from_le_bytes)
    }
    fn i16(&mut self) -> Result<i16, Error> {
        self.array().map(i16::from_le_bytes)
    }
    fn u32(&mut self) -> Result<u32, Error> {
        self.array().map(u32::from_le_bytes)
    }
}

fn u16_at(bytes: &[u8], i: usize) -> Option<u16> {
    let b = bytes.get(i * 2..i * 2 + 2)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}
fn u32_at(bytes: &[u8], i: usize) -> Option<u32> {
    let b = bytes.get(i * 4..i * 4 + 4)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// One atlas file: a script group's faces.
#[derive(Clone, Copy, Debug)]
pub struct Font {
    faces: [Option<Face>; 2],
}

impl Font {
    /// Parses and checks the whole file; the strikes' pixels are inflated
    /// later, by [`Strike::unpack`].
    pub fn parse(bytes: &'static [u8]) -> Result<Self, Error> {
        let mut r = Reader { bytes };
        if r.take(4)? != b"ARKG" || r.u8()? != 1 {
            return Err(Error::Header);
        }
        let mut faces = [None; 2];
        for _ in 0..r.u8()? {
            let face = Face::parse(&mut r)?;
            *faces.get_mut(face.weight as usize).ok_or(Error::Header)? = Some(face);
        }
        Ok(Self { faces })
    }

    pub fn face(&self, weight: Weight) -> Option<&Face> {
        self.faces.get(weight as usize)?.as_ref()
    }

    pub fn faces(&self) -> impl Iterator<Item = &Face> {
        self.faces.iter().flatten()
    }
}

/// One weight of one font: metrics, glyphs, kerning and strikes.
#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub weight: Weight,
    pub units_per_em: u16,
    pub ascent: i16,
    /// Negative: below the baseline.
    pub descent: i16,
    pub cap_height: i16,
    pub x_height: i16,
    chars: &'static [u8],
    advances: &'static [u8],
    kerns: &'static [u8],
    strikes: &'static [u8],
    strike_count: u8,
}

impl Face {
    fn parse(r: &mut Reader<'static>) -> Result<Self, Error> {
        let weight = match r.u8()? {
            0 => Weight::Regular,
            1 => Weight::Medium,
            _ => return Err(Error::Header),
        };
        let units_per_em = r.u16()?;
        let (ascent, descent, cap_height, x_height) = (r.i16()?, r.i16()?, r.i16()?, r.i16()?);
        if units_per_em == 0 {
            return Err(Error::Header);
        }
        let glyphs = usize::from(r.u16()?);
        let chars = r.take(glyphs * 4)?;
        let advances = r.take(glyphs * 2)?;
        let kern_count = usize::from(r.u16()?);
        let kerns = r.take(kern_count * 6)?;
        let strike_count = r.u8()?;
        let start = r.bytes;
        for _ in 0..strike_count {
            Strike::parse(r)?;
        }
        let strikes = &start[..start.len() - r.bytes.len()];
        Ok(Self {
            weight,
            units_per_em,
            ascent,
            descent,
            cap_height,
            x_height,
            chars,
            advances,
            kerns,
            strikes,
            strike_count,
        })
    }

    /// How many characters the face holds.
    pub fn len(&self) -> usize {
        self.chars.len() / 4
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// The glyph index for `c`, if the face has it.
    pub fn glyph(&self, c: char) -> Option<u16> {
        let (mut low, mut high) = (0, self.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match u32_at(self.chars, mid)?.cmp(&u32::from(c)) {
                core::cmp::Ordering::Less => low = mid + 1,
                core::cmp::Ordering::Greater => high = mid,
                core::cmp::Ordering::Equal => return u16::try_from(mid).ok(),
            }
        }
        None
    }

    /// The character at a glyph index.
    pub fn char(&self, glyph: u16) -> Option<char> {
        char::from_u32(u32_at(self.chars, usize::from(glyph))?)
    }

    /// The advance, in font units.
    pub fn advance(&self, glyph: u16) -> u16 {
        u16_at(self.advances, usize::from(glyph)).unwrap_or(0)
    }

    /// Pair kerning between two glyphs, in font units.
    pub fn kern(&self, left: u16, right: u16) -> i16 {
        let key = (left, right);
        let (mut low, mut high) = (0, self.kerns.len() / 6);
        while low < high {
            let mid = low + (high - low) / 2;
            let pair = (
                u16_at(self.kerns, mid * 3).unwrap_or(0),
                u16_at(self.kerns, mid * 3 + 1).unwrap_or(0),
            );
            match pair.cmp(&key) {
                core::cmp::Ordering::Less => low = mid + 1,
                core::cmp::Ordering::Greater => high = mid,
                core::cmp::Ordering::Equal => {
                    return u16_at(self.kerns, mid * 3 + 2).map_or(0, |v| v as i16);
                }
            }
        }
        0
    }

    pub fn strikes(&self) -> impl Iterator<Item = Strike> {
        let mut r = Reader {
            bytes: self.strikes,
        };
        (0..self.strike_count).map_while(move |_| Strike::parse(&mut r).ok())
    }

    pub fn strike(&self, ppem: u8) -> Option<Strike> {
        self.strikes().find(|s| s.ppem == ppem)
    }
}

/// One baked pixel size of a face.
#[derive(Clone, Copy, Debug)]
pub struct Strike {
    pub ppem: u8,
    unpacked: usize,
    packed: &'static [u8],
}

impl Strike {
    fn parse(r: &mut Reader<'static>) -> Result<Self, Error> {
        let ppem = r.u8()?;
        let unpacked = r.u32()? as usize;
        let packed_len = r.u32()? as usize;
        let packed = r.take(packed_len)?;
        Ok(Self {
            ppem,
            unpacked,
            packed,
        })
    }

    /// The buffer size [`Strike::unpack`] needs.
    pub fn unpacked_len(&self) -> usize {
        self.unpacked
    }

    /// Inflates the strike into `out`, which must be exactly
    /// [`Strike::unpacked_len`] bytes, and returns its glyphs.
    pub fn unpack<'b>(&self, out: &'b mut [u8]) -> Result<Images<'b>, Error> {
        if out.len() != self.unpacked {
            return Err(Error::Truncated);
        }
        let written = miniz_oxide::inflate::decompress_slice_iter_to_slice(
            out,
            core::iter::once(self.packed),
            false,
            true,
        )
        .map_err(|_| Error::Inflate)?;
        if written != self.unpacked {
            return Err(Error::Truncated);
        }
        let mut r = Reader { bytes: out };
        let count = usize::from(r.u16()?);
        let entries = r.take(count * 6)?;
        let pixels = r.bytes;
        let needed: usize = entries
            .as_chunks::<6>()
            .0
            .iter()
            .map(|e| usize::from(e[2]) * usize::from(e[3]))
            .sum();
        if needed != pixels.len() {
            return Err(Error::Truncated);
        }
        Ok(Images { entries, pixels })
    }
}

/// A strike's glyph bitmaps, in glyph order.
pub struct Images<'b> {
    entries: &'b [u8],
    pixels: &'b [u8],
}

/// One glyph's coverage bitmap and where it sits relative to the pen.
#[derive(Clone, Copy, Debug)]
pub struct Image<'b> {
    pub glyph: u16,
    pub width: u8,
    pub height: u8,
    /// Pixels from the pen to the bitmap's left edge.
    pub left: i8,
    /// Pixels from the baseline up to the bitmap's top edge.
    pub top: i8,
    /// `width × height` coverage values, row by row.
    pub pixels: &'b [u8],
}

impl<'b> Iterator for Images<'b> {
    type Item = Image<'b>;
    fn next(&mut self) -> Option<Image<'b>> {
        let (entry, rest) = self.entries.split_at_checked(6)?;
        self.entries = rest;
        let (width, height) = (entry[2], entry[3]);
        let (pixels, rest) = self
            .pixels
            .split_at_checked(usize::from(width) * usize::from(height))?;
        self.pixels = rest;
        Some(Image {
            glyph: u16::from_le_bytes([entry[0], entry[1]]),
            width,
            height,
            left: entry[4] as i8,
            top: entry[5] as i8,
            pixels,
        })
    }
}

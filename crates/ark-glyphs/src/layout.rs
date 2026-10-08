//! Places glyphs along a line. The pen moves in fractional pixels: advances,
//! kerning and tracking accumulate unrounded, and only each glyph's final
//! position is rounded, so measuring and drawing agree to the pixel.
use crate::{
    data::{Face, Font},
    spec::{self, Weight},
};
use ark_text::Script;

/// The faces a locale draws with: Noto Sans for everything, and the
/// locale's Noto Sans CJK subset for its CJK characters.
#[derive(Clone, Copy, Debug)]
pub struct Fonts {
    pub latin: Font,
    pub cjk: Option<Font>,
    pub script: Script,
}

/// Which font a glyph comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Latin,
    Cjk,
}

/// One glyph of a laid-out line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placed {
    pub source: Source,
    /// `None` when no baked face has the character; it then takes no space.
    pub glyph: Option<u16>,
    pub c: char,
    /// The pen position in pixels from the line's start, unrounded.
    pub x: f32,
}

impl Fonts {
    /// The face for `c`: CJK characters from the CJK font when there is one.
    pub fn face(&self, c: char, weight: Weight) -> (Source, Option<&Face>) {
        match &self.cjk {
            Some(cjk) if spec::is_cjk(c) => (Source::Cjk, cjk.face(weight)),
            _ => (Source::Latin, self.latin.face(weight)),
        }
    }

    /// Letter spacing in pixels at `ppem`. Ideographic text is never tracked.
    pub fn tracking(&self, em: f32, ppem: u8) -> f32 {
        if self.script == Script::Ideographic {
            0.0
        } else {
            em * f32::from(ppem)
        }
    }

    /// Lays out `text` at `ppem`, calling `place` for each glyph, and
    /// returns the line's advance width in pixels.
    pub fn layout(
        &self,
        text: &str,
        weight: Weight,
        ppem: u8,
        tracking: f32,
        mut place: impl FnMut(Placed),
    ) -> f32 {
        let mut pen = 0.0;
        let mut previous: Option<(Source, u16)> = None;
        let mut first = true;
        for c in text.chars() {
            let (source, face) = self.face(c, weight);
            let Some((face, glyph)) = face.and_then(|f| Some((f, f.glyph(c)?))) else {
                place(Placed {
                    source,
                    glyph: None,
                    c,
                    x: pen,
                });
                previous = None;
                continue;
            };
            let scale = f32::from(ppem) / f32::from(face.units_per_em);
            if let Some((prior, left)) = previous
                && prior == source
            {
                pen += f32::from(face.kern(left, glyph)) * scale;
            }
            if !first {
                pen += tracking;
            }
            first = false;
            place(Placed {
                source,
                glyph: Some(glyph),
                c,
                x: pen,
            });
            pen += f32::from(face.advance(glyph)) * scale;
            previous = Some((source, glyph));
        }
        pen
    }

    /// The advance width of `text` in pixels.
    pub fn measure(&self, text: &str, weight: Weight, ppem: u8, tracking: f32) -> f32 {
        self.layout(text, weight, ppem, tracking, |_| {})
    }

    /// Splits `text` into lines no wider than `max` pixels, calling `line`
    /// for each. Breaks at spaces, and between CJK characters except before
    /// closing punctuation or after opening punctuation. A word wider than
    /// `max` gets a line of its own. Returns the number of lines.
    pub fn wrap<'t>(
        &self,
        text: &'t str,
        weight: Weight,
        ppem: u8,
        tracking: f32,
        max: f32,
        mut line: impl FnMut(&'t str),
    ) -> usize {
        let mut lines = 0;
        let mut rest = text.trim();
        while !rest.is_empty() {
            let mut end = rest.len();
            let mut last_break = None;
            let mut previous: Option<char> = None;
            for (i, c) in rest.char_indices() {
                if let Some(p) = previous
                    && breaks_between(p, c, self.script == Script::Ideographic)
                {
                    let candidate = rest[..i].trim_end();
                    if self.measure(candidate, weight, ppem, tracking) > max {
                        end = last_break.unwrap_or(i);
                        break;
                    }
                    last_break = Some(i);
                }
                previous = Some(c);
            }
            if end == rest.len() && self.measure(rest, weight, ppem, tracking) > max {
                end = last_break.unwrap_or(rest.len());
            }
            line(rest[..end].trim_end());
            lines += 1;
            rest = rest[end..].trim_start();
        }
        lines
    }
}

/// Whether a line may break between `a` and `b`. Chinese and Japanese may
/// break between any two characters; Korean, like alphabetic text, breaks
/// between words.
fn breaks_between(a: char, b: char, ideographic: bool) -> bool {
    const NO_START: &str =
        "、。，．・：；！？」』）】〉》ーゝゞヽヾぁぃぅぇぉっゃゅょァィゥェォッャュョ,.:;!?)]";
    const NO_END: &str = "「『（【〈《([";
    if NO_START.contains(b) || NO_END.contains(a) {
        return false;
    }
    a == ' ' || (ideographic && ((spec::is_cjk(a) && b != ' ') || (spec::is_cjk(b) && a != ' ')))
}

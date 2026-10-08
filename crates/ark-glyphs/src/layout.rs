//! Places glyphs along a line. The pen moves in fractional pixels: advances,
//! kerning and tracking accumulate unrounded, and only each glyph's final
//! position is rounded, so measuring and drawing agree to the pixel.
//!
//! Thai consonants and their marks are placed as one baked cluster. Arabic
//! is shaped into contextual letter forms and laid out right to left
//! ([`crate::script`]); `x` always grows left to right on screen.
use crate::{
    data::{Face, Font},
    script,
    spec::{self, Weight},
};
use ark_text::Script;

/// The faces a locale draws with: Noto Sans for everything, and the
/// locale's own script font (Noto Sans CJK, Thai or Arabic UI) for the
/// characters of that script.
#[derive(Clone, Copy, Debug)]
pub struct Fonts {
    pub latin: Font,
    pub local: Option<Font>,
    pub script: Script,
}

/// Which font a glyph comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Latin,
    /// The locale's own script font.
    Local,
}

/// The longest line, in characters, that Arabic layout reorders; longer
/// text lays out its first part and marks the rest missing, so the layout
/// tests fail on it instead of it vanishing.
const RTL_LINE: usize = 192;

/// U+200B ZERO WIDTH SPACE: where a Thai line may break. It takes no room.
const BREAK: char = '\u{200B}';

/// How wide a capsule icon is, in em: the in-game capsule's 3:2 pill at
/// the text's size. Its advance is exactly this; the spaces around it in
/// the text give it room.
pub const ICON_EM: f32 = 1.5;

/// One glyph of a laid-out line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placed {
    pub source: Source,
    /// `None` when no baked face has the character; it then takes no space,
    /// unless it is a capsule [`ark_text::icon`], which takes [`ICON_EM`].
    pub glyph: Option<u16>,
    pub c: char,
    /// The pen position in pixels from the line's start, unrounded.
    pub x: f32,
}

impl Fonts {
    /// The face for `c`: the locale's own script from its font when there
    /// is one, everything else from Noto Sans.
    pub fn face(&self, c: char, weight: Weight) -> (Source, Option<&Face>) {
        match &self.local {
            Some(local) if spec::is_local(c) => (Source::Local, local.face(weight)),
            _ => (Source::Latin, self.latin.face(weight)),
        }
    }

    /// Letter spacing in pixels at `ppem`. Only alphabetic and Hangul text
    /// is tracked: spacing would break Arabic joins, Thai clusters, and the
    /// even grid of Chinese and Japanese.
    pub fn tracking(&self, em: f32, ppem: u8) -> f32 {
        match self.script {
            Script::Alphabetic | Script::Hangul => em * f32::from(ppem),
            Script::Ideographic | Script::Thai | Script::Arabic => 0.0,
        }
    }

    /// Calls `unit` with each cluster of `text` in visual order: Arabic
    /// shaped and reordered, Thai marks kept with their consonant.
    fn units(&self, text: &str, mut unit: impl FnMut(&[char])) {
        if self.script != Script::Arabic {
            script::thai_clusters(text, |c| {
                if c != [BREAK] {
                    unit(c);
                }
            });
            return;
        }
        let mut logical = ['\0'; RTL_LINE];
        let mut n = 0;
        let mut overflow = false;
        script::arabic_forms(text, |c| match logical.get_mut(n) {
            Some(slot) => {
                *slot = c;
                n += 1;
            }
            None => overflow = true,
        });
        let mut visual = ['\0'; RTL_LINE];
        script::visual(&logical[..n], &mut visual[..n]);
        for c in &visual[..n] {
            unit(core::slice::from_ref(c));
        }
        if overflow {
            unit(&['\u{FFFD}']);
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
        self.units(text, |unit| {
            let c = unit[0];
            if ark_text::icon_power(c).is_some() {
                if !first {
                    pen += tracking;
                }
                first = false;
                place(Placed {
                    source: Source::Latin,
                    glyph: None,
                    c,
                    x: pen,
                });
                pen += ICON_EM * f32::from(ppem);
                previous = None;
                return;
            }
            let (source, face) = self.face(c, weight);
            let Some((face, glyph)) = face.and_then(|f| Some((f, f.cluster(unit)?))) else {
                place(Placed {
                    source,
                    glyph: None,
                    c,
                    x: pen,
                });
                previous = None;
                return;
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
        });
        pen
    }

    /// The advance width of `text` in pixels.
    pub fn measure(&self, text: &str, weight: Weight, ppem: u8, tracking: f32) -> f32 {
        self.layout(text, weight, ppem, tracking, |_| {})
    }

    /// Splits `text` into lines no wider than `max` pixels, calling `line`
    /// for each. Breaks at spaces and at Thai break marks, and between
    /// Chinese or Japanese characters except before closing punctuation or
    /// after opening punctuation. A word wider than `max` gets a line of its
    /// own. Returns the number of lines.
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
    a == ' '
        || a == BREAK
        || (ideographic && ((spec::is_cjk(a) && b != ' ') || (spec::is_cjk(b) && a != ' ')))
}

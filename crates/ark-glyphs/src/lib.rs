//! ARKONK's UI type: Noto Sans for Latin, Greek and Cyrillic, plus Noto
//! Sans CJK, Noto Sans Thai and Noto Sans Arabic UI for the languages that
//! need them, pre-rasterized by `tools/fontbake` into hinted strikes at the
//! pixel sizes in [`spec`], with advances and GPOS pair kerning. No font
//! file ships and nothing parses one at run time: the game turns these
//! strikes into one texture and draws each glyph 1:1 at a whole pixel.
//!
//! The Chinese, Japanese, Korean, Thai and Arabic atlases are large, so
//! they are linked only with the `scripts` feature; without it those
//! locales are unavailable and a build draws Latin, Greek and Cyrillic.
#![no_std]

mod data;
mod layout;
pub mod script;
pub mod spec;

pub use data::{Error, Face, Font, Image, Images, Strike};
pub use layout::{Fonts, ICON_EM, Placed, Source};

use ark_text::Locale;

static LATIN: &[u8] = include_bytes!("../data/latin.bin");
#[cfg(feature = "scripts")]
static SCRIPTS: [(Locale, &[u8]); 6] = [
    (Locale::ZhHans, include_bytes!("../data/zh.bin")),
    (Locale::ZhHant, include_bytes!("../data/tw.bin")),
    (Locale::Ja, include_bytes!("../data/ja.bin")),
    (Locale::Ko, include_bytes!("../data/ko.bin")),
    (Locale::Th, include_bytes!("../data/th.bin")),
    (Locale::Ar, include_bytes!("../data/ar.bin")),
];

/// The script atlas a locale needs, if any; `Some(None)` means the locale
/// needs one that this build does not link.
fn local(locale: Locale) -> Option<Option<&'static [u8]>> {
    // Latin, Greek and Cyrillic come from Noto Sans alone.
    if locale.script() == ark_text::Script::Alphabetic {
        return None;
    }
    #[cfg(feature = "scripts")]
    return Some(SCRIPTS.iter().find(|s| s.0 == locale).map(|s| s.1));
    #[cfg(not(feature = "scripts"))]
    return Some(None);
}

/// Whether this build can draw `locale`.
pub fn supports(locale: Locale) -> bool {
    !matches!(local(locale), Some(None))
}

/// The fonts for `locale`. Fails only if the linked data is damaged, which
/// the tests rule out for every committed file.
pub fn fonts(locale: Locale) -> Result<Fonts, Error> {
    let local = match local(locale) {
        Some(Some(bytes)) => Some(Font::parse(bytes)?),
        Some(None) => return Err(Error::Header),
        None => None,
    };
    Ok(Fonts {
        latin: Font::parse(LATIN)?,
        local,
        script: locale.script(),
    })
}

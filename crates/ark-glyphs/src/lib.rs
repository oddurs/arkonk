//! ARKONK's UI type: Noto Sans and Noto Sans CJK, pre-rasterized by
//! `tools/fontbake` into hinted strikes at the pixel sizes in [`spec`], with
//! advances and GPOS pair kerning. No font file ships and nothing parses
//! one at run time: the game turns these strikes into one texture and
//! draws each glyph 1:1 at a whole pixel.
//!
//! The Chinese, Japanese and Korean subsets are large, so they are linked
//! only with the `cjk` feature; without it those locales are unavailable.
#![no_std]

mod data;
mod layout;
pub mod spec;

pub use data::{Error, Face, Font, Image, Images, Strike};
pub use layout::{Fonts, ICON_EM, Placed, Source};

use ark_text::Locale;

static LATIN: &[u8] = include_bytes!("../data/latin.bin");
#[cfg(feature = "cjk")]
static ZH: &[u8] = include_bytes!("../data/zh.bin");
#[cfg(feature = "cjk")]
static JA: &[u8] = include_bytes!("../data/ja.bin");
#[cfg(feature = "cjk")]
static KO: &[u8] = include_bytes!("../data/ko.bin");

/// The CJK atlas a locale needs, if any; `None` from `Some(None)` means the
/// locale needs one that this build does not link.
fn cjk(locale: Locale) -> Option<Option<&'static [u8]>> {
    match locale {
        #[cfg(feature = "cjk")]
        Locale::ZhHans => Some(Some(ZH)),
        #[cfg(feature = "cjk")]
        Locale::Ja => Some(Some(JA)),
        #[cfg(feature = "cjk")]
        Locale::Ko => Some(Some(KO)),
        #[cfg(not(feature = "cjk"))]
        Locale::ZhHans | Locale::Ja | Locale::Ko => Some(None),
        _ => None,
    }
}

/// Whether this build can draw `locale`.
pub fn supports(locale: Locale) -> bool {
    !matches!(cjk(locale), Some(None))
}

/// The fonts for `locale`. Fails only if the linked data is damaged, which
/// the tests rule out for every committed file.
pub fn fonts(locale: Locale) -> Result<Fonts, Error> {
    let cjk = match cjk(locale) {
        Some(Some(bytes)) => Some(Font::parse(bytes)?),
        Some(None) => return Err(Error::Header),
        None => None,
    };
    Ok(Fonts {
        latin: Font::parse(LATIN)?,
        cjk,
        script: locale.script(),
    })
}

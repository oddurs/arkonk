//! The type scale: what each text role is set in, and the pixel sizes that
//! get baked. The bake tool compiles this same file, so the sizes it bakes
//! and the sizes the game asks for cannot disagree.
use ark_text::Role;

/// The cuts in use: Noto Sans Regular and Medium, and Noto Sans Display
/// Medium for headings. CJK has no Display cut, so its Display face is
/// baked from Noto Sans CJK Medium.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weight {
    Regular,
    Medium,
    Display,
}

impl Weight {
    pub const ALL: [Self; 3] = [Self::Regular, Self::Medium, Self::Display];
}

/// A role's size in scene units (the fixed 960 × 900 scene), its cut, and
/// its letter spacing in em. Labels are small capitals, so they get air;
/// large headings close up a little. Ideographic scripts are never tracked.
pub const fn style(role: Role) -> (f32, Weight, f32) {
    match role {
        Role::Display => (40.0, Weight::Display, -0.01),
        Role::Title => (26.0, Weight::Display, -0.005),
        Role::Figure => (32.0, Weight::Medium, 0.0),
        // 20 puts Steam Deck body text on the 18 px strike, whose lowercase
        // is just over 9 px tall.
        Role::Body => (20.0, Weight::Regular, 0.0),
        Role::Caption => (16.0, Weight::Regular, 0.0),
        Role::Label => (15.0, Weight::Medium, 0.10),
    }
}

/// Baseline to baseline, in em, for lines of `role` that wrap.
pub const fn line(role: Role) -> f32 {
    match role {
        Role::Display => 1.1,
        Role::Title => 1.2,
        Role::Body | Role::Caption => 1.4,
        Role::Figure | Role::Label => 1.0,
    }
}

/// The cut a role takes when it is emphasised: body text is Medium on a
/// primary or focused action. The other roles have one cut.
pub const fn strong(role: Role) -> Weight {
    match role {
        Role::Body => Weight::Medium,
        _ => style(role).1,
    }
}

pub const ROLES: [Role; 6] = [
    Role::Display,
    Role::Title,
    Role::Figure,
    Role::Body,
    Role::Caption,
    Role::Label,
];

/// Every pixel size a strike may be baked at. Steps stay within about 10 %
/// up to 32 px, then widen where a pixel matters less. The largest serve
/// the Display role on Retina and 4K screens.
pub const LADDER: [u8; 25] = [
    10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 22, 24, 26, 28, 30, 32, 36, 40, 44, 48, 56, 64, 72, 80,
    96,
];

/// Physical pixels per scene unit on the displays the layouts are checked
/// at: 720p (1280 × 720), Steam Deck (1280 × 800), a 960 × 900 window at
/// 100 %, 1080p, 1440p, a Retina window at 200 %, and 4K. The 720p strikes
/// keep body text in small windows near its planned size instead of a
/// fifth larger.
pub const DENSITIES: [f32; 7] = [0.8, 800.0 / 900.0, 1.0, 1.2, 1.6, 2.0, 2.4];

/// The ladder entry nearest `px`, comparing ratios rather than differences.
pub fn nearest(px: f32) -> u8 {
    closest(px, LADDER).unwrap_or(LADDER[0])
}

/// How far apart two sizes are, as a ratio of at least 1.
fn spread(px: f32, rung: u8) -> f32 {
    let ratio = px / f32::from(rung);
    if ratio >= 1.0 { ratio } else { 1.0 / ratio }
}

fn closest(px: f32, rungs: impl IntoIterator<Item = u8>) -> Option<u8> {
    let mut best: Option<u8> = None;
    for rung in rungs {
        if best.is_none_or(|b| spread(px, rung) < spread(px, b)) {
            best = Some(rung);
        }
    }
    best
}

/// The sizes baked for `role`: the nearest ladder entry at each density.
pub fn rungs(role: Role) -> impl Iterator<Item = u8> {
    let (size, _, _) = style(role);
    let mut seen = [0_u8; DENSITIES.len()];
    DENSITIES.into_iter().enumerate().filter_map(move |(i, d)| {
        let rung = nearest(size * d);
        let fresh = !seen[..i].contains(&rung);
        seen[i] = rung;
        fresh.then_some(rung)
    })
}

/// The baked size for `role` at `density`: the nearest ladder entry, or
/// the largest baked size below it when that entry was not baked (the
/// density lies between the checked ones). Text is then never more than
/// half a ladder step larger than its layout planned. `None` below about
/// 87 % of the smallest baked size, where the pixel font takes over.
pub fn ppem(role: Role, density: f32) -> Option<u8> {
    ppem_px(role, style(role).0 * density)
}

/// [`ppem`] for text of `role` laid out `px` physical pixels tall, for the
/// few places set off the role's own size (a card's name, the band's
/// smaller figures).
pub fn ppem_px(role: Role, px: f32) -> Option<u8> {
    let smallest = rungs(role).min()?;
    if px < f32::from(smallest) * 0.87 {
        return None;
    }
    let ladder = nearest(px);
    rungs(role)
        .filter(|&r| r <= ladder)
        .max()
        .or(Some(smallest))
}
/// Characters drawn from the CJK faces; everything else comes from Noto Sans.
pub const fn is_cjk(c: char) -> bool {
    matches!(c as u32,
        0x1100..=0x11FF      // Hangul Jamo
        | 0x2E80..=0x2FDF    // CJK radicals
        | 0x3000..=0x303F    // CJK symbols and punctuation
        | 0x3040..=0x30FF    // Hiragana, Katakana
        | 0x3130..=0x318F    // Hangul compatibility Jamo
        | 0x31F0..=0x31FF    // Katakana extensions
        | 0x3400..=0x4DBF    // CJK extension A
        | 0x4E00..=0x9FFF    // CJK unified ideographs
        | 0xAC00..=0xD7AF    // Hangul syllables
        | 0xF900..=0xFAFF    // CJK compatibility ideographs
        | 0xFF00..=0xFFEF) // Half- and full-width forms
}

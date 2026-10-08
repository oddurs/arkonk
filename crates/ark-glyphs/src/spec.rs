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
/// at. Regular frames: the 720-pixel-wide frame where Regular begins,
/// 720p (1280 × 720), Steam Deck (1280 × 800), a 960 × 900 window at
/// 100 %, 1080p, 1440p, a Retina window at 200 %, and 4K. Small frames,
/// 400 to 719 pixels wide: where Small begins, the smallest desktop window
/// (480 × 450) and one between, where floors hold text above its scale.
pub const DENSITIES: [f32; 11] = [
    400.0 / 960.0,
    0.5,
    0.6,
    0.75,
    0.8,
    800.0 / 900.0,
    1.0,
    1.2,
    1.6,
    2.0,
    2.4,
];

/// The smallest pixel size a role is ever set at: the physical floors.
/// Floors only raise sizes; the layout reflows around them. A Label's
/// capitals stay at least 7 px tall (10 px type); headings and figures
/// never drop below body text.
pub const fn floor(role: Role) -> f32 {
    match role {
        Role::Body => 12.0,
        Role::Caption => 11.0,
        Role::Label => 10.0,
        Role::Display | Role::Title | Role::Figure => 14.0,
    }
}

/// The ladder entry nearest `px`, comparing ratios rather than differences.
pub const fn nearest(px: f32) -> u8 {
    let mut best = LADDER[0];
    let mut i = 1;
    while i < LADDER.len() {
        if spread(px, LADDER[i]) < spread(px, best) {
            best = LADDER[i];
        }
        i += 1;
    }
    best
}

/// How far apart two sizes are, as a ratio of at least 1.
const fn spread(px: f32, rung: u8) -> f32 {
    let ratio = px / rung as f32;
    if ratio >= 1.0 { ratio } else { 1.0 / ratio }
}

/// The sizes baked for `role`: the nearest ladder entry at each density,
/// never below the role's floor.
pub fn rungs(role: Role) -> impl Iterator<Item = u8> {
    let (sizes, count) = RUNGS[role_index(role)];
    sizes.into_iter().take(count)
}

/// [`rungs`] for every role in [`ROLES`] order, worked out at compile
/// time: drawing asks for a role's sizes several times for every word.
const RUNGS: [([u8; DENSITIES.len()], usize); ROLES.len()] = {
    let mut out = [([0; DENSITIES.len()], 0); ROLES.len()];
    let mut r = 0;
    while r < ROLES.len() {
        let role = ROLES[r];
        let (mut sizes, mut count) = ([0; DENSITIES.len()], 0);
        let mut d = 0;
        while d < DENSITIES.len() {
            let rung = nearest(f32::max(style(role).0 * DENSITIES[d], floor(role)));
            let mut seen = false;
            let mut j = 0;
            while j < count {
                seen |= sizes[j] == rung;
                j += 1;
            }
            if !seen {
                sizes[count] = rung;
                count += 1;
            }
            d += 1;
        }
        out[r] = (sizes, count);
        r += 1;
    }
    out
};

/// Where `role` sits in [`ROLES`].
const fn role_index(role: Role) -> usize {
    match role {
        Role::Display => 0,
        Role::Title => 1,
        Role::Figure => 2,
        Role::Body => 3,
        Role::Caption => 4,
        Role::Label => 5,
    }
}

/// The baked size for `role` at `density`: the nearest ladder entry, or
/// the largest baked size below it when that entry was not baked (the
/// density lies between the checked ones). Text is then never more than
/// half a ladder step larger than its layout planned, except where a
/// floor raises it.
pub fn ppem(role: Role, density: f32) -> u8 {
    ppem_px(role, style(role).0 * density)
}

/// [`ppem`] for text of `role` laid out `px` physical pixels tall, for the
/// few places set off the role's own size (a card's name, the band's
/// smaller figures).
pub fn ppem_px(role: Role, px: f32) -> u8 {
    let ladder = nearest(px.max(floor(role)));
    let smallest = rungs(role).min().unwrap_or(LADDER[0]);
    rungs(role)
        .filter(|&r| r <= ladder)
        .max()
        .unwrap_or(smallest)
}

/// The baked size one step below `ppem` for `role`, the fit chain's
/// third step, if it does not breach the floor.
pub fn step_down(role: Role, ppem: u8) -> Option<u8> {
    let floor = nearest(floor(role));
    rungs(role).filter(|&r| r < ppem && r >= floor).max()
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

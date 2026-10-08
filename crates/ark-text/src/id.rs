//! Typed ids for every piece of display text.
use ark::{
    Power,
    sectors::{Chapter, SectorId},
};

/// Declares [`TextId`] and the list of its argument-free variants together,
/// so the list cannot miss one.
macro_rules! text_ids {
    ($($(#[$doc:meta])* $unit:ident,)*) => {
        /// A piece of display text. Templates may hold `{0}`, `{1}`… for the
        /// arguments listed on each id, and `{icon:<power slug>}` for a
        /// capsule's fixed letter.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum TextId {
            $($(#[$doc])* $unit,)*
            SectorName(SectorId),
            /// One line of advice shown before serving.
            SectorTip(SectorId),
            ChapterName(Chapter),
            /// Shown for a moment when the power is picked up.
            PowerName(Power),
        }
        impl TextId {
            const UNITS: &[Self] = &[$(Self::$unit),*];
        }
    };
}

text_ids! {
    // Title screen.
    Tagline,
    ContinueJourney,
    NewJourney,
    SectorSelect,
    /// The second line of the Continue button: `{0}` the saved sector's
    /// name, `{1}` the score saved with it.
    ContinueDetail,
    StatMedals,
    StatBest,
    /// The journey under the title band's pips: `{0}` sectors open of
    /// `{1}`.
    SectorsOf,

    // Keyboard and mouse hints.
    // Key names on keycap glyphs.
    KeySpace,
    KeyEsc,
    // Gamepad hints: a button glyph, then one of these.
    ActionServe,
    ActionRelease,
    ActionSelect,
    ActionResume,
    ActionBack,

    // Settings.
    Settings,
    HelpSettings,
    SettingSound,
    SettingVolume,
    SettingDisplay,
    SettingLanguage,
    DisplayWindow,
    Fullscreen,
    /// Follow Steam's language, then the system's.
    LanguageSystem,
    ActionAdjust,

    // Sector select.
    SectorsHeading,
    PracticeNote,
    MedalClear,
    MedalClean,
    MedalSwift,
    /// The Swift target, `{0}`, and the best time, `{1}`.
    TargetBest,
    /// `{0}` sector number.
    PlaySector,
    /// What opens a locked sector: `{0}` the number of the sector before it.
    UnlockHint,

    // Play.
    /// `{0}` points.
    Plus,
    /// `{0}` chapter name, `{1}` sector number.
    ReadyEyebrow,

    // Pause, results.
    Paused,
    RetrySector,
    MainMenu,
    JourneyComplete,
    OneMoreOrbit,
    StatPoints,
    SectorClear,
    StatTime,
    StatBonus,
    StatChain,
    ExtraLife,
    NextSector,
    BackToSectors,

    SaveFailed,
    /// Under the band's lives when a chapter earns one.
    LifeGained,
    PerfTitle,

    // Help for the focused action, one caption line under a sheet's list.
    HelpResume,
    HelpRetry,
    HelpMainMenu,
    HelpSectors,
    HelpNewJourney,

    // Steam rich presence, shown in friends' lists in their own language.
    PresenceMenus,
    /// `{0}` sector number, `{1}` sector name.
    PresenceJourney,
    /// `{0}` sector number, `{1}` sector name.
    PresencePractice,

}

/// Which of the six text roles an id is set in, which decides the glyphs
/// baked for each size: labels and headings use few characters, so their
/// sizes carry only those. Any id may also be set as [`Role::Body`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    /// The one hero line on a screen, in the Display cut.
    Display,
    /// Sheet titles and screen headers, in the Display cut.
    Title,
    /// Scores and results, tabular.
    Figure,
    /// Actions, names, tips.
    Body,
    /// Help lines and notes: quieter than body text.
    Caption,
    /// Small, tracked, authored in capitals where the script has them:
    /// names of values and eyebrows, never actions.
    Label,
}

impl TextId {
    /// Every id, each once.
    pub fn all() -> impl Iterator<Item = Self> {
        Self::UNITS
            .iter()
            .copied()
            .chain(SectorId::all().map(Self::SectorName))
            .chain(SectorId::all().map(Self::SectorTip))
            .chain(Chapter::ALL.map(Self::ChapterName))
            .chain(Power::ALL.map(Self::PowerName))
    }

    /// How many `{n}` arguments the template takes.
    pub const fn arity(self) -> usize {
        use TextId::*;
        match self {
            ContinueDetail | SectorsOf | ReadyEyebrow | TargetBest | PresenceJourney
            | PresencePractice => 2,
            PlaySector | UnlockHint | Plus => 1,
            _ => 0,
        }
    }

    pub const fn role(self) -> Role {
        use TextId::*;
        match self {
            StatMedals | StatBest | MedalClear | MedalClean | MedalSwift | ReadyEyebrow
            | StatPoints | StatTime | StatBonus | StatChain | ChapterName(_) | PowerName(_) => {
                Role::Label
            }
            SectorsHeading | Paused | JourneyComplete | OneMoreOrbit | SectorClear => Role::Title,
            SectorName(_) => Role::Display,
            ActionServe | ActionRelease | ActionSelect | ActionBack | Fullscreen | SaveFailed
            | ContinueDetail | PracticeNote | SectorsOf | TargetBest | UnlockHint | Tagline
            | HelpResume | HelpRetry | HelpMainMenu | HelpSectors | HelpNewJourney | ExtraLife
            | LifeGained | HelpSettings | DisplayWindow | LanguageSystem | ActionAdjust => {
                Role::Caption
            }
            _ => Role::Body,
        }
    }

    /// Other roles this text is set in besides [`Self::role`] and body
    /// text, including where another string quotes it through an
    /// [`crate::Arg::Text`] slot: a sector's name is the ready card's hero
    /// line, the detail sheet's title, and part of the Continue button's
    /// caption. The atlases bake its characters at those roles' sizes too.
    pub const fn also(self) -> &'static [Role] {
        use TextId::*;
        match self {
            SectorName(_) => &[Role::Title, Role::Caption],
            // The ready card sets a tip as body text, the detail sheet as a caption.
            SectorTip(_) => &[Role::Caption],
            // Results set points as figures; play floats them as captions.
            Plus => &[Role::Figure, Role::Caption],
            Settings => &[Role::Title],
            // Keycaps set their names in the Label cut, untracked.
            KeySpace | KeyEsc => &[Role::Label],
            _ => &[],
        }
    }

    /// Whether this text may be set emphasised, in its role's strong cut:
    /// actions, which are Medium when primary or focused, and the sector
    /// name in the band.
    pub const fn strong(self) -> bool {
        use TextId::*;
        matches!(
            self,
            ContinueJourney
                | NewJourney
                | SectorSelect
                | ActionResume
                | RetrySector
                | MainMenu
                | NextSector
                | BackToSectors
                | PlaySector
                | Settings
                | SectorName(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_id_is_listed_once() {
        extern crate std;
        use std::vec::Vec;
        let all: Vec<_> = TextId::all().collect();
        for (i, id) in all.iter().enumerate() {
            assert!(!all[..i].contains(id), "{id:?} listed twice");
        }
        assert_eq!(all.len(), TextId::UNITS.len() + 12 + 12 + 3 + 5);
    }
}

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
    /// number, `{1}` its name, `{2}` the score saved with it.
    ContinueDetail,
    StatSectors,
    StatMedals,
    StatBest,
    /// `{0}` of `{1}`, such as unlocked sectors of all sectors.
    Fraction,

    // Keyboard and mouse hints.
    KeysMove,
    KeysServe,
    KeysPause,
    KeysRelease,
    KeysBrowse,
    KeysPlay,
    KeysBack,
    KeysContinue,
    // Gamepad hints: a button glyph, then one of these.
    PadMove,
    PadBrowse,
    ActionServe,
    ActionPause,
    ActionRelease,
    ActionSelect,
    ActionResume,
    ActionRetry,
    ActionPlay,
    ActionBack,
    ActionContinue,

    // Settings shortcuts.
    SoundOn,
    SoundOff,
    /// `{0}` volume step.
    Volume,
    Fullscreen,

    // Sector select.
    SectorsHeading,
    PracticeNote,
    MedalClear,
    MedalClean,
    MedalSwift,
    MedalClearHow,
    MedalCleanHow,
    /// How to earn Swift: `{0}` the sector's target time.
    SwiftWithin,
    /// `{0}` sector number.
    PlaySector,
    /// What opens a locked sector: `{0}` the number of the sector before it.
    UnlockHint,

    // Play.
    Score,
    Lives,
    /// `{0}` sector number.
    SectorNumber,
    /// `{0}` sector number.
    PracticeNumber,
    /// `{0}` points.
    Plus,
    /// `{0}` chapter name, `{1}` sector number.
    ReadyEyebrow,

    // Pause, results.
    Paused,
    RetrySector,
    MainMenu,
    RetryNote,
    JourneyComplete,
    OneMoreOrbit,
    StatPoints,
    ProgressSaved,
    SectorClear,
    StatTime,
    StatBonus,
    StatBestChain,
    ExtraLife,
    NextSector,
    BackToSectors,

    SaveFailed,
    PerfTitle,

}

/// Which text style an id is set in, which decides the glyphs baked for
/// each size: labels and display lines use few characters, so their sizes
/// carry only those. Hints are captions. Any id may also be set as
/// [`Role::Body`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    /// Small, tracked, authored in capitals where the script has them.
    Label,
    Body,
    /// Control hints and other secondary lines: quieter than body text.
    Caption,
    /// Headings and large figures.
    Display,
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
            ContinueDetail => 3,
            Fraction | ReadyEyebrow => 2,
            Volume | SwiftWithin | PlaySector | UnlockHint | SectorNumber | PracticeNumber
            | Plus => 1,
            _ => 0,
        }
    }

    pub const fn role(self) -> Role {
        use TextId::*;
        match self {
            StatSectors | StatMedals | StatBest | MedalClear | MedalClean | MedalSwift | Score
            | Lives | SectorNumber | PracticeNumber | ReadyEyebrow | StatPoints | StatTime
            | StatBonus | StatBestChain | ChapterName(_) => Role::Label,
            SectorsHeading | Paused | JourneyComplete | OneMoreOrbit | SectorClear
            | SectorName(_) => Role::Display,
            KeysMove | KeysServe | KeysPause | KeysRelease | KeysBrowse | KeysPlay | KeysBack
            | KeysContinue | PadMove | PadBrowse | ActionServe | ActionPause | ActionRelease
            | ActionSelect | ActionResume | ActionRetry | ActionPlay | ActionBack
            | ActionContinue | SoundOn | SoundOff | Volume | Fullscreen | SaveFailed
            | ContinueDetail | PracticeNote => Role::Caption,
            _ => Role::Body,
        }
    }

    /// The role of another string that quotes this one through an
    /// [`crate::Arg::Text`] slot, when that differs from [`Self::role`]:
    /// the Continue button's caption names the saved sector. The atlases
    /// bake the quoted characters at that role's sizes too.
    pub const fn quoted_as(self) -> Option<Role> {
        match self {
            TextId::SectorName(_) => Some(Role::Caption),
            _ => None,
        }
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

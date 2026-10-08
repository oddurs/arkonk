//! English display text for what the simulation names only by id: sectors,
//! chapters and powers. A stand-in until string tables and other locales
//! replace it; callers already ask by [`TextId`], so they will not change.
use ark::{
    Power,
    sectors::{Chapter, SECTOR_COUNT, SectorId},
};

/// A piece of display text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextId {
    SectorName(SectorId),
    /// One line of advice shown before serving.
    SectorTip(SectorId),
    ChapterName(Chapter),
    /// Shown for a moment when the power is picked up.
    PowerName(Power),
}

/// The English text for `id`.
pub fn text(id: TextId) -> &'static str {
    match id {
        TextId::SectorName(sector) => SECTOR_NAMES[sector.index()],
        TextId::SectorTip(sector) => SECTOR_TIPS[sector.index()],
        TextId::ChapterName(chapter) => match chapter {
            Chapter::Daybreak => "DAYBREAK",
            Chapter::BlueHour => "BLUE HOUR",
            Chapter::Afterlight => "AFTERLIGHT",
        },
        TextId::PowerName(power) => match power {
            Power::Wide => "WIDE",
            Power::Slow => "SLOW",
            Power::Multi => "MULTIBALL",
            Power::Anchor => "ANCHOR",
            Power::Phase => "PHASE",
        },
    }
}

/// In journey order. The array length makes a missing sector a build error.
const SECTOR_NAMES: [&str; SECTOR_COUNT] = [
    "FIRST LIGHT",
    "SATELLITES",
    "SLIPSTREAM",
    "RESONANCE",
    "PRISM",
    "CROSSFADE",
    "UNDERTOW",
    "MOONRISE",
    "AFTERGLOW",
    "PARALLAX",
    "SUPERNOVA",
    "HOMECOMING",
];

/// In journey order. Capsule letters (W, S, M, A, P) are the capsules'
/// fixed icons, not words to translate.
const SECTOR_TIPS: [&str; SECTOR_COUNT] = [
    "W WIDE / S SLOW / CATCH THE FALLING CAPSULES",
    "A ANCHOR / CATCH, REPOSITION, CLICK TO RELEASE",
    "AMBER CORES / EACH BLAST REACHES FOUR NEIGHBORS",
    "NEIGHBORING CORES CARRY THE REACTION",
    "M MULTIBALL / THREE BALLS, ONE OPENING",
    "OPEN A ROUTE THROUGH THE TWO RELAY LINES",
    "BREAK INTO THE POCKETS BEHIND THE ARMOR",
    "FOLLOW THE RELAY AROUND THE OPEN CENTER",
    "P PHASE / THREE BRICK CONTACTS WITHOUT A BOUNCE",
    "PIERCE THE SHELL, THEN IGNITE THE INNER ROUTE",
    "CATCH A RETURN WHILE THE OTHER BALLS KEEP GOING",
    "ONE LAST ORBIT / MAKE EACH OPENING COUNT",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sector_has_a_distinct_name_and_a_tip() {
        let names: Vec<_> = SectorId::all()
            .map(|s| text(TextId::SectorName(s)))
            .collect();
        for (i, name) in names.iter().enumerate() {
            assert!(!name.is_empty() && !names[..i].contains(name), "{name}");
        }
        assert!(SectorId::all().all(|s| !text(TextId::SectorTip(s)).is_empty()));
    }
}

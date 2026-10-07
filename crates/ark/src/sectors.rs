//! The twelve hand-authored sectors. Breathing room and readable routes
//! matter more than raw brick count; each chapter introduces a denser rhythm.
//!
//! Layouts are written as rows of text and parsed at compile time: `.` is
//! empty, `1`–`3` a brick with that many hit points, and `R` a one-hit relay
//! core. A malformed row is a build error, not a runtime surprise.
use crate::{
    Power,
    field::{CELLS, COLS, Cell, CellSet, ROWS},
};

/// Sectors in the journey.
pub const SECTOR_COUNT: usize = 12;

/// One of the twelve sectors. Always in range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectorId(u8);

impl SectorId {
    /// Where a journey starts.
    pub const FIRST: Self = Self(0);

    /// The sector at `index` in journey order, if there is one.
    pub const fn new(index: usize) -> Option<Self> {
        if index < SECTOR_COUNT {
            Some(Self(index as u8))
        } else {
            None
        }
    }
    /// The sector at `index`, or the last sector when `index` is past it.
    pub const fn clamped(index: usize) -> Self {
        match Self::new(index) {
            Some(id) => id,
            None => Self(SECTOR_COUNT as u8 - 1),
        }
    }
    /// Position in the journey, `0..SECTOR_COUNT`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// The sector after this one; `None` after the last.
    pub const fn next(self) -> Option<Self> {
        Self::new(self.index() + 1)
    }
    /// The authored content.
    pub const fn sector(self) -> &'static Sector {
        &SECTORS[self.index()]
    }
    /// Every sector in journey order.
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        (0..SECTOR_COUNT as u8).map(Self)
    }
    /// Whether this is the last sector of its chapter.
    pub fn ends_chapter(self) -> bool {
        self.next()
            .is_none_or(|next| next.sector().chapter != self.sector().chapter)
    }
}

/// The journey's three chapters, four sectors each.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Chapter {
    /// Sectors 1 to 4: isolated cores and open routes.
    Daybreak,
    /// Sectors 5 to 8: branching relay lines.
    BlueHour,
    /// Sectors 9 to 12: armoured pockets and dense chains.
    Afterlight,
}

impl Chapter {
    /// Every chapter, in journey order.
    pub const ALL: [Self; 3] = [Self::Daybreak, Self::BlueHour, Self::Afterlight];

    /// A stable lowercase key for string tables and stats; never displayed.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Daybreak => "daybreak",
            Self::BlueHour => "blue_hour",
            Self::Afterlight => "afterlight",
        }
    }
    /// The chapter's first sector.
    pub const fn first_sector(self) -> SectorId {
        SectorId(match self {
            Self::Daybreak => 0,
            Self::BlueHour => 4,
            Self::Afterlight => 8,
        })
    }
}

/// A sector's authored content. Display text lives with the front end,
/// keyed by [`Sector::slug`] or [`SectorId`].
#[derive(Debug)]
pub struct Sector {
    /// A stable lowercase key for string tables and stats; never displayed.
    pub slug: &'static str,
    /// The capsule that drops first, teaching this sector's idea.
    pub opening: Power,
    /// The chapter it belongs to.
    pub chapter: Chapter,
    /// Serve speed in pixels per second, before rally speed-up.
    pub speed: f32,
    /// The Swift medal's target time.
    pub par_seconds: u32,
    /// The starting bricks.
    pub layout: Layout,
}

/// A sector's starting bricks.
#[derive(Clone, Copy, Debug)]
pub struct Layout {
    /// Hit points per cell; zero is empty.
    pub hp: [u8; CELLS],
    /// Relay cores, which damage their neighbours when destroyed.
    pub cores: CellSet,
}

/// Parses seven rows of twelve cells; see the module docs for the syntax.
const fn layout(rows: [&str; ROWS]) -> Layout {
    let mut hp = [0; CELLS];
    let mut cores = CellSet::EMPTY;
    let mut row = 0;
    while row < ROWS {
        let text = rows[row].as_bytes();
        assert!(text.len() == COLS, "a layout row has twelve cells");
        let mut col = 0;
        while col < COLS {
            let Some(cell) = Cell::new(row * COLS + col) else {
                panic!("rows and columns stay inside the grid");
            };
            match text[col] {
                b'.' => {}
                b @ b'1'..=b'3' => hp[cell.index()] = b - b'0',
                b'R' => {
                    hp[cell.index()] = 1;
                    cores.insert(cell);
                }
                _ => panic!("a layout cell is '.', '1' to '3', or 'R'"),
            }
            col += 1;
        }
        row += 1;
    }
    Layout { hp, cores }
}

/// The journey, in order.
pub const SECTORS: [Sector; SECTOR_COUNT] = [
    Sector {
        slug: "first_light",
        opening: Power::Wide,
        chapter: Chapter::Daybreak,
        speed: 420.0,
        par_seconds: 100,
        layout: layout([
            "............",
            ".1111..1111.",
            ".1111..1111.",
            ".1111..1111.",
            "..111..111..",
            "...11..11...",
            "............",
        ]),
    },
    Sector {
        slug: "satellites",
        opening: Power::Anchor,
        chapter: Chapter::Daybreak,
        speed: 440.0,
        par_seconds: 110,
        layout: layout([
            "..11....11..",
            ".1111..1111.",
            "..22....22..",
            "..11....11..",
            ".1111..1111.",
            "..11....11..",
            "....1111....",
        ]),
    },
    Sector {
        slug: "slipstream",
        opening: Power::Anchor,
        chapter: Chapter::Daybreak,
        speed: 455.0,
        par_seconds: 100,
        layout: layout([
            "..111..111..",
            "..1R1..1R1..",
            "..111..111..",
            "............",
            ".111....111.",
            ".1R1....1R1.",
            ".111....111.",
        ]),
    },
    Sector {
        slug: "resonance",
        opening: Power::Anchor,
        chapter: Chapter::Daybreak,
        speed: 470.0,
        par_seconds: 115,
        layout: layout([
            "....1111....",
            "...11RR11...",
            "..11RRRR11..",
            ".121R..R121.",
            "..11R..R11..",
            "...111111...",
            "....1111....",
        ]),
    },
    Sector {
        slug: "prism",
        opening: Power::Multi,
        chapter: Chapter::BlueHour,
        speed: 480.0,
        par_seconds: 120,
        layout: layout([
            ".1111..1111.",
            ".1RR1..1RR1.",
            ".12R1..1R21.",
            ".12R1111R21.",
            ".1222..2221.",
            "..111..111..",
            "...11..11...",
        ]),
    },
    Sector {
        slug: "crossfade",
        opening: Power::Anchor,
        chapter: Chapter::BlueHour,
        speed: 495.0,
        par_seconds: 125,
        layout: layout([
            "111......111",
            "1R111..111R1",
            "1RRR1221RRR1",
            ".11R1..1R11.",
            "..1RRRRRR1..",
            "...122221...",
            "....1111....",
        ]),
    },
    Sector {
        slug: "undertow",
        opening: Power::Wide,
        chapter: Chapter::BlueHour,
        speed: 510.0,
        par_seconds: 135,
        layout: layout([
            "1111....1111",
            "1RR1....1RR1",
            "12R111111R21",
            "12RRR22RRR21",
            "12221..12221",
            "11111..11111",
            "..11....11..",
        ]),
    },
    Sector {
        slug: "moonrise",
        opening: Power::Multi,
        chapter: Chapter::BlueHour,
        speed: 525.0,
        par_seconds: 135,
        layout: layout([
            "..11111111..",
            ".11RRRRRR11.",
            "11RR....RR11",
            "12R......R21",
            "112......211",
            ".122....221.",
            "..111..111..",
        ]),
    },
    Sector {
        slug: "afterglow",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 540.0,
        par_seconds: 135,
        layout: layout([
            "1111....1111",
            "1RR1....1RR1",
            "12R1....1R21",
            "13R111111R31",
            "122211112221",
            ".111....111.",
            "..11....11..",
        ]),
    },
    Sector {
        slug: "parallax",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 555.0,
        par_seconds: 150,
        layout: layout([
            ".1122..2211.",
            "112R3..3R211",
            "12RRR11RRR21",
            "123R1221R321",
            "12RRR11RRR21",
            "11222..22211",
            ".1111..1111.",
        ]),
    },
    Sector {
        slug: "supernova",
        opening: Power::Multi,
        chapter: Chapter::Afterlight,
        speed: 570.0,
        par_seconds: 145,
        layout: layout([
            "1....11....1",
            ".11..RR..11.",
            "..1RRRRRR1..",
            "112R2332R211",
            "..1RRRRRR1..",
            ".11..22..11.",
            "1....11....1",
        ]),
    },
    Sector {
        slug: "homecoming",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 585.0,
        par_seconds: 160,
        layout: layout([
            "...111111...",
            "..12RRRR21..",
            ".12RR33RR21.",
            "112R3..3R211",
            ".12RRRRRR21.",
            "..12222221..",
            "...111111...",
        ]),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authored_layouts_are_valid_and_distinct() {
        for id in SectorId::all() {
            let layout = id.sector().layout;
            assert_eq!(!layout.cores.is_empty(), id.index() >= 2);
            for core in layout.cores.iter() {
                assert_eq!(layout.hp[core.index()], 1);
            }
            assert!(layout.hp.iter().filter(|&&hp| hp > 0).count() >= 24);
            for earlier in SectorId::all().take_while(|&e| e < id) {
                assert_ne!(layout.hp, earlier.sector().layout.hp);
            }
        }
        assert_eq!(SectorId::new(SECTOR_COUNT - 1).unwrap().next(), None);
    }
    #[test]
    fn chapters_are_four_consecutive_sectors() {
        for chapter in Chapter::ALL {
            let sectors: Vec<_> = SectorId::all()
                .filter(|s| s.sector().chapter == chapter)
                .map(SectorId::index)
                .collect();
            let first = chapter.first_sector().index();
            assert_eq!(sectors, (first..first + 4).collect::<Vec<_>>());
        }
        let ends: Vec<_> = SectorId::all()
            .filter(|s| s.ends_chapter())
            .map(SectorId::index)
            .collect();
        assert_eq!(ends, [3, 7, 11]);
    }
    #[test]
    fn slugs_are_unique_lowercase_keys() {
        let slugs = SectorId::all()
            .map(|s| s.sector().slug)
            .chain(Chapter::ALL.map(Chapter::slug))
            .chain(Power::ALL.map(Power::slug));
        let mut seen = Vec::new();
        for slug in slugs {
            assert!(
                slug.bytes().all(|b| b.is_ascii_lowercase() || b == b'_'),
                "{slug}"
            );
            assert!(!seen.contains(&slug), "{slug}");
            seen.push(slug);
        }
    }
}

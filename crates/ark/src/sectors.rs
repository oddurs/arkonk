//! The 64 hand-authored sectors: one day, from dawn to the next dawn, in
//! eight chapters of eight. Each chapter teaches one idea in its first
//! sectors, develops it, then remixes it with everything before; its eighth
//! sector is a set piece. `docs/journey.md` records the design.
//!
//! Layouts are written as rows of text and parsed at compile time:
//!
//! - `.` is empty;
//! - `1` to `3` is a brick with that many hit points;
//! - `R` is a one-hit relay core;
//! - `G` is a one-hit gate, which is solid and then a ghost on its sector's
//!   [`Beat`], and which balls and blasts pass while it is a ghost.
//!
//! A malformed row is a build error, not a runtime surprise.
use crate::{
    Power,
    clock::TICK_HZ,
    field::{CELLS, COLS, Cell, CellSet, ROWS},
};

/// Sectors in the journey.
pub const SECTOR_COUNT: usize = 64;
/// Sectors in each chapter.
pub const CHAPTER_SECTORS: usize = 8;

/// One of the journey's sectors. Always in range.
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
    /// The sector whose slug is `slug`, if there is one.
    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::all().find(|s| s.sector().slug == slug)
    }
    /// Position in the journey, `0..SECTOR_COUNT`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// Position in its chapter, `0..CHAPTER_SECTORS`.
    pub const fn chapter_index(self) -> usize {
        self.index() % CHAPTER_SECTORS
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

/// The journey's eight chapters, eight sectors each: one day, dawn to dawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Chapter {
    /// Sectors 1 to 8: steering, Wide and Slow, in open layouts.
    Daybreak,
    /// Sectors 9 to 16: Anchor, aiming into pockets, the first armour.
    Morning,
    /// Sectors 17 to 24: relay cores and chains.
    Zenith,
    /// Sectors 25 to 32: Multiball and dense fields.
    GoldenHour,
    /// Sectors 33 to 40: Phase, and armoured shells around cores.
    Afterlight,
    /// Sectors 41 to 48: darkness, which only the front end draws.
    BlueHour,
    /// Sectors 49 to 56: gates on a beat.
    Eclipse,
    /// Sectors 57 to 64: everything, faster.
    Aurora,
}

impl Chapter {
    /// Every chapter, in journey order.
    pub const ALL: [Self; 8] = [
        Self::Daybreak,
        Self::Morning,
        Self::Zenith,
        Self::GoldenHour,
        Self::Afterlight,
        Self::BlueHour,
        Self::Eclipse,
        Self::Aurora,
    ];

    /// A stable lowercase key for string tables and stats; never displayed.
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Daybreak => "daybreak",
            Self::Morning => "morning",
            Self::Zenith => "zenith",
            Self::GoldenHour => "golden_hour",
            Self::Afterlight => "afterlight",
            Self::BlueHour => "blue_hour",
            Self::Eclipse => "eclipse",
            Self::Aurora => "aurora",
        }
    }
    /// Position in the journey, `0..8`.
    pub const fn index(self) -> usize {
        self as usize
    }
    /// The chapter at `index`, if there is one.
    pub const fn new(index: usize) -> Option<Self> {
        if index < Self::ALL.len() {
            Some(Self::ALL[index])
        } else {
            None
        }
    }
    /// The chapter's first sector.
    pub const fn first_sector(self) -> SectorId {
        SectorId((self.index() * CHAPTER_SECTORS) as u8)
    }
    /// The chapter's sectors, in order.
    pub fn sectors(self) -> impl DoubleEndedIterator<Item = SectorId> {
        let first = self.first_sector().0;
        (first..first + CHAPTER_SECTORS as u8).map(SectorId)
    }
    /// The chapter before this one; `None` before the first.
    pub const fn previous(self) -> Option<Self> {
        match self.index().checked_sub(1) {
            Some(i) => Self::new(i),
            None => None,
        }
    }
    /// The chapter after this one; `None` after the last.
    pub const fn next(self) -> Option<Self> {
        Self::new(self.index() + 1)
    }
}

/// How a sector's gates keep time: solid for `solid` ticks, then a ghost for
/// `ghost` ticks, over and over, counted in the sector's play time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Beat {
    /// Ticks solid at the start of each beat.
    pub solid: u32,
    /// Ticks a ghost after that.
    pub ghost: u32,
}

/// Where a [`Beat`] is at a moment of play.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BeatPhase {
    /// Whether gates are solid now.
    pub solid: bool,
    /// Ticks until they turn, at least one.
    pub left: u32,
    /// Ticks the current half of the beat lasts.
    pub length: u32,
}

impl Beat {
    /// A beat authored in tenths of a second.
    pub const fn tenths(solid: u32, ghost: u32) -> Self {
        Self {
            solid: solid * TICK_HZ / 10,
            ghost: ghost * TICK_HZ / 10,
        }
    }
    /// Where the beat is after `ticks` of play. A beat with no ghost half
    /// is always solid.
    pub const fn at(self, ticks: u32) -> BeatPhase {
        let Some(t) = ticks.checked_rem(self.solid.saturating_add(self.ghost)) else {
            return BeatPhase {
                solid: true,
                left: 1,
                length: 1,
            };
        };
        // With no ghost half the period is `solid`, so `t < solid` holds.
        if t < self.solid || self.ghost == 0 {
            BeatPhase {
                solid: true,
                left: self.solid - t,
                length: self.solid,
            }
        } else {
            BeatPhase {
                solid: false,
                left: self.solid + self.ghost - t,
                length: self.ghost,
            }
        }
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
    /// The Swift medal's target time, derived from autopilot clears
    /// (`tests/clears.rs`).
    pub par_seconds: u32,
    /// How the gates keep time; `None` when the layout has no gates.
    pub beat: Option<Beat>,
    /// How dark the front end draws the board, 0 (lit) to 100. The rules
    /// never read it.
    pub darkness: u8,
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
    /// Gates, which follow the sector's beat.
    pub gates: CellSet,
}

/// Parses seven rows of twelve cells; see the module docs for the syntax.
const fn layout(rows: [&str; ROWS]) -> Layout {
    let mut hp = [0; CELLS];
    let mut cores = CellSet::EMPTY;
    let mut gates = CellSet::EMPTY;
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
                b'G' => {
                    hp[cell.index()] = 1;
                    gates.insert(cell);
                }
                _ => panic!("a layout cell is '.', '1' to '3', 'R' or 'G'"),
            }
            col += 1;
        }
        row += 1;
    }
    Layout { hp, cores, gates }
}

/// The journey, in order.
pub const SECTORS: [Sector; SECTOR_COUNT] = [
    Sector {
        slug: "first_light",
        opening: Power::Wide,
        chapter: Chapter::Daybreak,
        speed: 420.0,
        par_seconds: 170,
        beat: None,
        darkness: 0,
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
        slug: "drift",
        opening: Power::Slow,
        chapter: Chapter::Daybreak,
        speed: 425.0,
        par_seconds: 95,
        beat: None,
        darkness: 0,
        layout: layout([
            "............",
            "11..11..11..",
            ".11..11..11.",
            "..11..11..11",
            "............",
            ".1.1.1.1.1.1",
            "............",
        ]),
    },
    Sector {
        slug: "horizon",
        opening: Power::Wide,
        chapter: Chapter::Daybreak,
        speed: 430.0,
        par_seconds: 105,
        beat: None,
        darkness: 0,
        layout: layout([
            "............",
            "111111111111",
            "............",
            ".1111111111.",
            "............",
            "..11111111..",
            "............",
        ]),
    },
    Sector {
        slug: "glimmer",
        opening: Power::Slow,
        chapter: Chapter::Daybreak,
        speed: 435.0,
        par_seconds: 115,
        beat: None,
        darkness: 0,
        layout: layout([
            "1...1..1...1",
            "..1...1...1.",
            "1...1..1...1",
            "..1...1...1.",
            "1...1..1...1",
            "..1...1...1.",
            "1...1..1...1",
        ]),
    },
    Sector {
        slug: "skylark",
        opening: Power::Wide,
        chapter: Chapter::Daybreak,
        speed: 440.0,
        par_seconds: 115,
        beat: None,
        darkness: 0,
        layout: layout([
            "1..........1",
            ".1........1.",
            "..1..11..1..",
            "...1.11.1...",
            "1...1111...1",
            ".1...11...1.",
            "..1......1..",
        ]),
    },
    Sector {
        slug: "lanterns",
        opening: Power::Slow,
        chapter: Chapter::Daybreak,
        speed: 445.0,
        par_seconds: 135,
        beat: None,
        darkness: 0,
        layout: layout([
            "..1..11..1..",
            ".111.11.111.",
            ".111....111.",
            ".111.11.111.",
            "..1..11..1..",
            ".....11.....",
            "....1111....",
        ]),
    },
    Sector {
        slug: "tidewater",
        opening: Power::Wide,
        chapter: Chapter::Daybreak,
        speed: 450.0,
        par_seconds: 85,
        beat: None,
        darkness: 0,
        layout: layout([
            "11........11",
            ".111....111.",
            "...111111...",
            "............",
            "11........11",
            ".111....111.",
            "...111111...",
        ]),
    },
    Sector {
        slug: "sunrise",
        opening: Power::Slow,
        chapter: Chapter::Daybreak,
        speed: 460.0,
        par_seconds: 165,
        beat: None,
        darkness: 0,
        layout: layout([
            ".1...11...1.",
            "..1..11..1..",
            "............",
            "...111111...",
            "..11111111..",
            ".1111111111.",
            "111111111111",
        ]),
    },
    Sector {
        slug: "satellites",
        opening: Power::Anchor,
        chapter: Chapter::Morning,
        speed: 440.0,
        par_seconds: 175,
        beat: None,
        darkness: 0,
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
        slug: "dewpoint",
        opening: Power::Wide,
        chapter: Chapter::Morning,
        speed: 445.0,
        par_seconds: 190,
        beat: None,
        darkness: 0,
        layout: layout([
            "..1......1..",
            ".111....111.",
            ".121....121.",
            "..2..11..2..",
            "....1111....",
            "....1221....",
            ".....22.....",
        ]),
    },
    Sector {
        slug: "aperture",
        opening: Power::Anchor,
        chapter: Chapter::Morning,
        speed: 450.0,
        par_seconds: 175,
        beat: None,
        darkness: 0,
        layout: layout([
            "111111111111",
            ".1111111111.",
            "............",
            "22222..22222",
            "............",
            "..1......1..",
            "............",
        ]),
    },
    Sector {
        slug: "cloister",
        opening: Power::Anchor,
        chapter: Chapter::Morning,
        speed: 455.0,
        par_seconds: 125,
        beat: None,
        darkness: 0,
        layout: layout([
            "............",
            ".2111111112.",
            ".1.111111.1.",
            ".1.111111.1.",
            ".1........1.",
            ".2111..1112.",
            "............",
        ]),
    },
    Sector {
        slug: "keystone",
        opening: Power::Slow,
        chapter: Chapter::Morning,
        speed: 460.0,
        par_seconds: 185,
        beat: None,
        darkness: 0,
        layout: layout([
            ".....22.....",
            "....1111....",
            "...11..11...",
            "..11....11..",
            ".11......11.",
            ".11......11.",
            ".22......22.",
        ]),
    },
    Sector {
        slug: "sundial",
        opening: Power::Wide,
        chapter: Chapter::Morning,
        speed: 470.0,
        par_seconds: 170,
        beat: None,
        darkness: 0,
        layout: layout([
            "1....11....1",
            ".1...11...1.",
            "..1..11..1..",
            "...1.22.1...",
            "111122221111",
            "............",
            "....1111....",
        ]),
    },
    Sector {
        slug: "pinhole",
        opening: Power::Anchor,
        chapter: Chapter::Morning,
        speed: 475.0,
        par_seconds: 120,
        beat: None,
        darkness: 0,
        layout: layout([
            ".1111111111.",
            ".1........1.",
            ".1.111111.1.",
            ".1.1....1.1.",
            ".1.1.11.1.1.",
            "22222.222222",
            "............",
        ]),
    },
    Sector {
        slug: "windrose",
        opening: Power::Anchor,
        chapter: Chapter::Morning,
        speed: 483.0,
        par_seconds: 200,
        beat: None,
        darkness: 0,
        layout: layout([
            ".....11.....",
            "....1221....",
            ".1..1221..1.",
            "111122221111",
            ".1..1221..1.",
            "....1221....",
            ".....11.....",
        ]),
    },
    Sector {
        slug: "slipstream",
        opening: Power::Anchor,
        chapter: Chapter::Zenith,
        speed: 460.0,
        par_seconds: 120,
        beat: None,
        darkness: 0,
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
        slug: "filament",
        opening: Power::Wide,
        chapter: Chapter::Zenith,
        speed: 465.0,
        par_seconds: 70,
        beat: None,
        darkness: 0,
        layout: layout([
            "............",
            ".1.1.1.1.1.1",
            ".1111111111.",
            ".RRRRRRRRRR.",
            ".1111111111.",
            "1.1.1.1.1.1.",
            "............",
        ]),
    },
    Sector {
        slug: "meridian",
        opening: Power::Anchor,
        chapter: Chapter::Zenith,
        speed: 470.0,
        par_seconds: 90,
        beat: None,
        darkness: 0,
        layout: layout([
            "111..RR..111",
            "111..RR..111",
            ".....RR.....",
            ".11..RR..11.",
            ".11..RR..11.",
            ".....RR.....",
            "....1111....",
        ]),
    },
    Sector {
        slug: "cascade",
        opening: Power::Wide,
        chapter: Chapter::Zenith,
        speed: 480.0,
        par_seconds: 75,
        beat: None,
        darkness: 0,
        layout: layout([
            "RR1.....1111",
            "1RR1....1111",
            ".1RR1.......",
            "..1RR1......",
            "...1RR1.....",
            "....1RR1....",
            ".....1RR1...",
        ]),
    },
    Sector {
        slug: "crossfade",
        opening: Power::Anchor,
        chapter: Chapter::Zenith,
        speed: 485.0,
        par_seconds: 105,
        beat: None,
        darkness: 0,
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
        slug: "switchback",
        opening: Power::Anchor,
        chapter: Chapter::Zenith,
        speed: 490.0,
        par_seconds: 60,
        beat: None,
        darkness: 0,
        layout: layout([
            ".1111111111.",
            ".RRRRRRRR...",
            ".1111111111.",
            "...RRRRRRRR.",
            ".1111111111.",
            ".RRRRRRRR...",
            ".1111111111.",
        ]),
    },
    Sector {
        slug: "solstice",
        opening: Power::Slow,
        chapter: Chapter::Zenith,
        speed: 500.0,
        par_seconds: 45,
        beat: None,
        darkness: 0,
        layout: layout([
            "...111111...",
            "..1RRRRRR1..",
            ".1R111111R1.",
            ".1R1....1R1.",
            ".1R111111R1.",
            "..1RRRRRR1..",
            "...111111...",
        ]),
    },
    Sector {
        slug: "resonance",
        opening: Power::Anchor,
        chapter: Chapter::Zenith,
        speed: 510.0,
        par_seconds: 100,
        beat: None,
        darkness: 0,
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
        chapter: Chapter::GoldenHour,
        speed: 480.0,
        par_seconds: 125,
        beat: None,
        darkness: 0,
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
        slug: "honeycomb",
        opening: Power::Multi,
        chapter: Chapter::GoldenHour,
        speed: 485.0,
        par_seconds: 120,
        beat: None,
        darkness: 0,
        layout: layout([
            ".111.111.111",
            ".1R1.1R1.1R1",
            ".111.111.111",
            "111.111.111.",
            "1R1.1R1.1R1.",
            "111.111.111.",
            "............",
        ]),
    },
    Sector {
        slug: "spindrift",
        opening: Power::Multi,
        chapter: Chapter::GoldenHour,
        speed: 495.0,
        par_seconds: 140,
        beat: None,
        darkness: 0,
        layout: layout([
            "1.1.1.1.1.1.",
            ".1.1.1.1.1.1",
            "1.2.1.1.2.1.",
            ".1.1.1.1.1.1",
            "1.1.2.2.1.1.",
            ".1.1.1.1.1.1",
            "1.1.1.1.1.1.",
        ]),
    },
    Sector {
        slug: "undertow",
        opening: Power::Wide,
        chapter: Chapter::GoldenHour,
        speed: 500.0,
        par_seconds: 110,
        beat: None,
        darkness: 0,
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
        slug: "harvest",
        opening: Power::Multi,
        chapter: Chapter::GoldenHour,
        speed: 510.0,
        par_seconds: 175,
        beat: None,
        darkness: 0,
        layout: layout([
            ".2.2.2.2.2.2",
            "111111111111",
            "1R11R11R11R1",
            "111111111111",
            "2.2.2.2.2.2.",
            "111111111111",
            "1R11R11R11R1",
        ]),
    },
    Sector {
        slug: "kaleidoscope",
        opening: Power::Anchor,
        chapter: Chapter::GoldenHour,
        speed: 515.0,
        par_seconds: 110,
        beat: None,
        darkness: 0,
        layout: layout([
            "R1..2112..1R",
            "1R1.1RR1.1R1",
            ".1R1.11.1R1.",
            "2.1R1..1R1.2",
            ".1R1.11.1R1.",
            "1R1.1RR1.1R1",
            "R1..2112..1R",
        ]),
    },
    Sector {
        slug: "tapestry",
        opening: Power::Multi,
        chapter: Chapter::GoldenHour,
        speed: 525.0,
        par_seconds: 160,
        beat: None,
        darkness: 0,
        layout: layout([
            "112211221122",
            "1RR11RR11RR1",
            "221122112211",
            "1RR11RR11RR1",
            "112211221122",
            "............",
            ".1.1.1.1.1.1",
        ]),
    },
    Sector {
        slug: "long_shadows",
        opening: Power::Multi,
        chapter: Chapter::GoldenHour,
        speed: 535.0,
        par_seconds: 100,
        beat: None,
        darkness: 0,
        layout: layout([
            "1.1.1..1.1.1",
            "1.1.1..1.1.1",
            "1.1.1..1.1.1",
            "2.2.2..2.2.2",
            "R.R.R..R.R.R",
            "RRRRRRRRRRRR",
            "111111111111",
        ]),
    },
    Sector {
        slug: "afterglow",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 500.0,
        par_seconds: 135,
        beat: None,
        darkness: 0,
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
        slug: "chrysalis",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 510.0,
        par_seconds: 125,
        beat: None,
        darkness: 0,
        layout: layout([
            "....3333....",
            "...3RRRR3...",
            "...3RRRR3...",
            "....3333....",
            "..1......1..",
            ".111....111.",
            "..1......1..",
        ]),
    },
    Sector {
        slug: "geode",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 515.0,
        par_seconds: 120,
        beat: None,
        darkness: 0,
        layout: layout([
            ".222....222.",
            "2RR2....2RR2",
            ".222....222.",
            "....1111....",
            "...2RRRR2...",
            "....1111....",
            "11........11",
        ]),
    },
    Sector {
        slug: "parallax",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 525.0,
        par_seconds: 160,
        beat: None,
        darkness: 0,
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
        slug: "citadel",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 530.0,
        par_seconds: 200,
        beat: None,
        darkness: 0,
        layout: layout([
            "2.2.2..2.2.2",
            "223322223322",
            "2.11111111.2",
            "2.1RRRRRR1.2",
            "2.11111111.2",
            "22222..22222",
            "............",
        ]),
    },
    Sector {
        slug: "nautilus",
        opening: Power::Phase,
        chapter: Chapter::Afterlight,
        speed: 540.0,
        par_seconds: 165,
        beat: None,
        darkness: 0,
        layout: layout([
            ".3333333333.",
            ".3RRRRRRRR3.",
            ".3R222222.3.",
            ".3R2RRRR2.3.",
            ".3R222222.3.",
            ".3RRRRRRRR3.",
            ".3333..3333.",
        ]),
    },
    Sector {
        slug: "vespers",
        opening: Power::Anchor,
        chapter: Chapter::Afterlight,
        speed: 550.0,
        par_seconds: 125,
        beat: None,
        darkness: 0,
        layout: layout([
            ".33..33..33.",
            "3RR33RR33RR3",
            "3RR33RR33RR3",
            ".11..11..11.",
            "............",
            "2.2.2..2.2.2",
            ".1.1.11.1.1.",
        ]),
    },
    Sector {
        slug: "supernova",
        opening: Power::Multi,
        chapter: Chapter::Afterlight,
        speed: 555.0,
        par_seconds: 90,
        beat: None,
        darkness: 0,
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
        slug: "moonrise",
        opening: Power::Multi,
        chapter: Chapter::BlueHour,
        speed: 490.0,
        par_seconds: 125,
        beat: None,
        darkness: 0,
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
        slug: "gloaming",
        opening: Power::Slow,
        chapter: Chapter::BlueHour,
        speed: 500.0,
        par_seconds: 115,
        beat: None,
        darkness: 30,
        layout: layout([
            ".....11.....",
            "...1....1...",
            ".1........1.",
            "............",
            "1R11R11R11R1",
            "122112211221",
            ".1111111111.",
        ]),
    },
    Sector {
        slug: "lamplight",
        opening: Power::Wide,
        chapter: Chapter::BlueHour,
        speed: 505.0,
        par_seconds: 135,
        beat: None,
        darkness: 50,
        layout: layout([
            ".121....121.",
            ".1R1....1R1.",
            ".111.11.111.",
            ".....RR.....",
            ".111.11.111.",
            ".1R1....1R1.",
            ".121....121.",
        ]),
    },
    Sector {
        slug: "fireflies",
        opening: Power::Slow,
        chapter: Chapter::BlueHour,
        speed: 515.0,
        par_seconds: 115,
        beat: None,
        darkness: 65,
        layout: layout([
            "11..R...11..",
            "..11..R...11",
            "R...11..11..",
            "..11..R...11",
            "11..R...11..",
            "..R...11..11",
            "11..11..R...",
        ]),
    },
    Sector {
        slug: "lighthouse",
        opening: Power::Anchor,
        chapter: Chapter::BlueHour,
        speed: 520.0,
        par_seconds: 125,
        beat: None,
        darkness: 80,
        layout: layout([
            ".....RR.....",
            "1...1221...1",
            "11...11...11",
            "....1111....",
            "11..1221..11",
            "...111111...",
            "1.11111111.1",
        ]),
    },
    Sector {
        slug: "nocturne",
        opening: Power::Multi,
        chapter: Chapter::BlueHour,
        speed: 530.0,
        par_seconds: 120,
        beat: None,
        darkness: 90,
        layout: layout([
            "11.11.11.11.",
            ".2...2...2..",
            "11.11.11.11.",
            "..R...R...R.",
            ".11.11.11.11",
            "...2...2...2",
            ".11.11.11.11",
        ]),
    },
    Sector {
        slug: "deep_field",
        opening: Power::Phase,
        chapter: Chapter::BlueHour,
        speed: 540.0,
        par_seconds: 90,
        beat: None,
        darkness: 95,
        layout: layout([
            "1.2..1.R.1..",
            ".1..R..1..21",
            "2..1..1..1..",
            ".R..21..R..1",
            "..1..1..2..R",
            "1..R..1..1..",
            ".1..2..1..1.",
        ]),
    },
    Sector {
        slug: "constellation",
        opening: Power::Anchor,
        chapter: Chapter::BlueHour,
        speed: 550.0,
        par_seconds: 50,
        beat: None,
        darkness: 100,
        layout: layout([
            "R1........1R",
            ".2R......R2.",
            "..1..RR..1..",
            "...1RRRR1...",
            "..1..RR..1..",
            ".2R......R2.",
            "R1........1R",
        ]),
    },
    Sector {
        slug: "penumbra",
        opening: Power::Slow,
        chapter: Chapter::Eclipse,
        speed: 520.0,
        par_seconds: 130,
        beat: Some(Beat::tenths(30, 20)),
        darkness: 0,
        layout: layout([
            "............",
            ".1111111111.",
            ".1111111111.",
            "............",
            ".GGGG..GGGG.",
            "............",
            "..11111111..",
        ]),
    },
    Sector {
        slug: "metronome",
        opening: Power::Wide,
        chapter: Chapter::Eclipse,
        speed: 530.0,
        par_seconds: 140,
        beat: Some(Beat::tenths(15, 15)),
        darkness: 0,
        layout: layout([
            "1G11G11G11G1",
            "111111111111",
            "............",
            "1G11G11G11G1",
            "111111111111",
            "............",
            ".1..1..1..1.",
        ]),
    },
    Sector {
        slug: "corona",
        opening: Power::Anchor,
        chapter: Chapter::Eclipse,
        speed: 535.0,
        par_seconds: 95,
        beat: Some(Beat::tenths(40, 20)),
        darkness: 0,
        layout: layout([
            "....GGGG....",
            "...G1RR1G...",
            "..G1RRRR1G..",
            "..G1RRRR1G..",
            "...G1RR1G...",
            "....GGGG....",
            ".11......11.",
        ]),
    },
    Sector {
        slug: "syzygy",
        opening: Power::Slow,
        chapter: Chapter::Eclipse,
        speed: 545.0,
        par_seconds: 120,
        beat: Some(Beat::tenths(20, 20)),
        darkness: 0,
        layout: layout([
            "............",
            ".111.RRR.111",
            ".1G1.RGR.1G1",
            ".111.RRR.111",
            "............",
            "2.2.2.2.2.2.",
            ".2.2.2.2.2.2",
        ]),
    },
    Sector {
        slug: "pulsar",
        opening: Power::Multi,
        chapter: Chapter::Eclipse,
        speed: 550.0,
        par_seconds: 120,
        beat: Some(Beat::tenths(10, 10)),
        darkness: 0,
        layout: layout([
            "G....GG....G",
            ".G...11...G.",
            "..G.1RR1.G..",
            "GG1.RRRR.1GG",
            "..G.1RR1.G..",
            ".G...11...G.",
            "G....GG....G",
        ]),
    },
    Sector {
        slug: "shutter",
        opening: Power::Phase,
        chapter: Chapter::Eclipse,
        speed: 560.0,
        par_seconds: 140,
        beat: Some(Beat::tenths(30, 15)),
        darkness: 0,
        layout: layout([
            "111111111111",
            "GGGGGGGGGGGG",
            "222222222222",
            "............",
            "GGGG....GGGG",
            "............",
            "....1111....",
        ]),
    },
    Sector {
        slug: "umbra",
        opening: Power::Anchor,
        chapter: Chapter::Eclipse,
        speed: 582.0,
        par_seconds: 95,
        beat: Some(Beat::tenths(30, 30)),
        darkness: 0,
        layout: layout([
            "RRR1........",
            "R2R1.GGGG...",
            "RRR1.G22G...",
            "1111.G22G..1",
            ".....GGGG.11",
            "..........1R",
            "11111111..1R",
        ]),
    },
    Sector {
        slug: "totality",
        opening: Power::Phase,
        chapter: Chapter::Eclipse,
        speed: 585.0,
        par_seconds: 190,
        beat: Some(Beat::tenths(25, 25)),
        darkness: 0,
        layout: layout([
            "...222222...",
            "..2GGGGGG2..",
            ".2GGRRRRGG2.",
            ".2GRRRRRRG2.",
            ".2GGRRRRGG2.",
            "..2GGGGGG2..",
            "...222222...",
        ]),
    },
    Sector {
        slug: "solar_wind",
        opening: Power::Wide,
        chapter: Chapter::Aurora,
        speed: 550.0,
        par_seconds: 125,
        beat: Some(Beat::tenths(20, 20)),
        darkness: 0,
        layout: layout([
            "11..22..RR..",
            ".11..22..RR.",
            "..11..22..RR",
            "G..11..22..R",
            "GG..11..22..",
            ".GG..11..22.",
            "..GG..11..22",
        ]),
    },
    Sector {
        slug: "borealis",
        opening: Power::Multi,
        chapter: Chapter::Aurora,
        speed: 554.0,
        par_seconds: 95,
        beat: Some(Beat::tenths(25, 15)),
        darkness: 0,
        layout: layout([
            "1..2..1..2..",
            "1R.2..1R.2..",
            ".1..2G.1..2G",
            ".1R.2G.1R.2G",
            "..1..2..1..2",
            "..1R.2..1R.2",
            "...1..2..1..",
        ]),
    },
    Sector {
        slug: "ribbons",
        opening: Power::Slow,
        chapter: Chapter::Aurora,
        speed: 565.0,
        par_seconds: 65,
        beat: Some(Beat::tenths(30, 20)),
        darkness: 0,
        layout: layout([
            "1.1.1..1.1.1",
            "1RRRRGGRRRR1",
            ".2........2.",
            ".1RRRGGRRR1.",
            "..2......2..",
            "..1RRGGRR1..",
            "1.1.1..1.1.1",
        ]),
    },
    Sector {
        slug: "polar_night",
        opening: Power::Anchor,
        chapter: Chapter::Aurora,
        speed: 580.0,
        par_seconds: 100,
        beat: Some(Beat::tenths(30, 20)),
        darkness: 80,
        layout: layout([
            "2.2.2..2.2.2",
            ".1R1.11.1R1.",
            "2.1..GG..1.2",
            ".1R1.11.1R1.",
            "2.2.2..2.2.2",
            "...11..11...",
            "..1..GG..1..",
        ]),
    },
    Sector {
        slug: "cathedral",
        opening: Power::Phase,
        chapter: Chapter::Aurora,
        speed: 590.0,
        par_seconds: 135,
        beat: Some(Beat::tenths(20, 20)),
        darkness: 0,
        layout: layout([
            ".....33.....",
            "....3RR3....",
            "...3RRRR3...",
            "..32.RR.23..",
            ".32..33..23.",
            "32...GG...23",
            "3.11.GG.11.3",
        ]),
    },
    Sector {
        slug: "shimmer",
        opening: Power::Multi,
        chapter: Chapter::Aurora,
        speed: 600.0,
        par_seconds: 170,
        beat: Some(Beat::tenths(15, 15)),
        darkness: 0,
        layout: layout([
            "12G1R21G12R1",
            "2R12G1R21G12",
            "1G21R12G1R21",
            "............",
            ".1.2.1.2.1.2",
            "2.1.2.1.2.1.",
            ".R.1.G.1.R.1",
        ]),
    },
    Sector {
        slug: "singularity",
        opening: Power::Phase,
        chapter: Chapter::Aurora,
        speed: 610.0,
        par_seconds: 140,
        beat: Some(Beat::tenths(20, 20)),
        darkness: 0,
        layout: layout([
            "..33333333..",
            ".3GGGGGGGG3.",
            "3G.RRRRRR.G3",
            "3G.R....R.G3",
            "3G.RRRRRR.G3",
            ".3GGGGGGGG3.",
            "..3......3..",
        ]),
    },
    Sector {
        slug: "homecoming",
        opening: Power::Phase,
        chapter: Chapter::Aurora,
        speed: 625.0,
        par_seconds: 75,
        beat: None,
        darkness: 0,
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
    use crate::tuning::{DROP_ORDER, POWER_UNLOCKS};

    #[test]
    fn authored_layouts_are_valid_and_distinct() {
        for id in SectorId::all() {
            let sector = id.sector();
            let layout = sector.layout;
            let chapter = sector.chapter.index();
            for core in layout.cores.iter() {
                assert_eq!(layout.hp[core.index()], 1);
            }
            for gate in layout.gates.iter() {
                assert_eq!(layout.hp[gate.index()], 1);
                assert!(!layout.cores.contains(gate));
            }
            let bricks = layout.hp.iter().filter(|&&hp| hp > 0).count();
            assert!(bricks >= 24, "{}", sector.slug);
            // Each chapter's idea arrives in its own chapter, not before.
            let armour = layout.hp.iter().any(|&hp| hp > 1);
            assert!(chapter >= Chapter::Morning.index() || !armour);
            assert!(chapter >= Chapter::Zenith.index() || layout.cores.is_empty());
            assert!(chapter >= Chapter::Eclipse.index() || layout.gates.is_empty());
            assert_eq!(
                layout.gates.is_empty(),
                sector.beat.is_none(),
                "{}",
                sector.slug
            );
            let dark = chapter == Chapter::BlueHour.index() || chapter == Chapter::Aurora.index();
            assert!(dark || sector.darkness == 0);
            assert!(sector.darkness <= 100);
            for earlier in SectorId::all().take_while(|&e| e < id) {
                assert_ne!(layout.hp, earlier.sector().layout.hp);
            }
        }
        assert_eq!(SectorId::new(SECTOR_COUNT - 1).unwrap().next(), None);
    }

    #[test]
    fn the_curve_climbs_in_each_chapter_and_steps_back_at_its_start() {
        for chapter in Chapter::ALL {
            let speeds: Vec<_> = chapter.sectors().map(|s| s.sector().speed).collect();
            assert!(speeds.windows(2).all(|w| w[0] < w[1]), "{chapter:?}");
            if let Some(before) = chapter.previous() {
                let last = before.sectors().last().unwrap().sector().speed;
                assert!(speeds[0] < last, "{chapter:?}");
            }
        }
        let first = SectorId::FIRST.sector().speed;
        let last = SectorId::clamped(SECTOR_COUNT).sector().speed;
        assert!(first < last);
    }

    #[test]
    fn chapters_are_eight_consecutive_sectors() {
        for chapter in Chapter::ALL {
            let sectors: Vec<_> = SectorId::all()
                .filter(|s| s.sector().chapter == chapter)
                .map(SectorId::index)
                .collect();
            let first = chapter.first_sector().index();
            assert_eq!(sectors, (first..first + 8).collect::<Vec<_>>());
            assert_eq!(
                chapter.sectors().map(SectorId::index).collect::<Vec<_>>(),
                sectors
            );
            assert_eq!(Chapter::new(chapter.index()), Some(chapter));
        }
        let ends: Vec<_> = SectorId::all()
            .filter(|s| s.ends_chapter())
            .map(SectorId::index)
            .collect();
        assert_eq!(ends, [7, 15, 23, 31, 39, 47, 55, 63]);
        assert_eq!(Chapter::Daybreak.previous(), None);
        assert_eq!(Chapter::Aurora.next(), None);
    }

    #[test]
    fn every_power_is_taught_by_an_opening_before_it_drops() {
        let mut taught = Vec::new();
        for id in SectorId::all() {
            let opening = id.sector().opening;
            if !taught.contains(&opening) {
                taught.push(opening);
            }
            let unlocked = &DROP_ORDER[..POWER_UNLOCKS[id.index()]];
            assert!(unlocked.iter().all(|p| taught.contains(p)), "{id:?}");
            assert!(unlocked.contains(&opening), "{id:?}");
        }
        assert_eq!(taught, DROP_ORDER);
        let teachers: Vec<_> = Power::ALL
            .map(|p| {
                SectorId::all()
                    .find(|s| s.sector().opening == p)
                    .unwrap()
                    .index()
                    + 1
            })
            .into();
        // Wide, Slow, Multi, Anchor, Phase.
        assert_eq!(teachers, [1, 2, 25, 9, 33]);
    }

    #[test]
    fn beats_alternate_from_solid() {
        let beat = Beat::tenths(30, 20);
        assert_eq!((beat.solid, beat.ghost), (720, 480));
        let at = |t| beat.at(t);
        assert_eq!(
            at(0),
            BeatPhase {
                solid: true,
                left: 720,
                length: 720
            }
        );
        assert_eq!(
            at(719),
            BeatPhase {
                solid: true,
                left: 1,
                length: 720
            }
        );
        assert_eq!(
            at(720),
            BeatPhase {
                solid: false,
                left: 480,
                length: 480
            }
        );
        assert_eq!(
            at(1199),
            BeatPhase {
                solid: false,
                left: 1,
                length: 480
            }
        );
        assert_eq!(at(1200), at(0));
        assert_eq!(at(u32::MAX), beat.at(u32::MAX % 1200));
        // Degenerate beats never divide by zero and never ghost.
        assert!(Beat { solid: 0, ghost: 0 }.at(5).solid);
        assert!(
            Beat {
                solid: 10,
                ghost: 0
            }
            .at(15)
            .solid
        );
        assert!(
            !Beat {
                solid: 0,
                ghost: 10
            }
            .at(5)
            .solid
        );
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
        for id in SectorId::all() {
            assert_eq!(SectorId::from_slug(id.sector().slug), Some(id));
        }
        assert_eq!(SectorId::from_slug("nowhere"), None);
    }
}

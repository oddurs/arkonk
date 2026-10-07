//! Hand-authored, fully destructible layouts. Breathing room and readable routes
//! matter more than raw brick count; each chapter introduces a denser rhythm.
use crate::game::Power;

pub const LEVEL_COUNT: usize = 12;
pub const CHAPTERS: [&str; 3] = ["DAYBREAK", "BLUE HOUR", "AFTERLIGHT"];

pub struct Level {
    pub name: &'static str,
    pub opening: Power,
    pub tip: &'static str,
    pub chapter: usize,
    pub speed: f32,
    pub par_seconds: u32,
    pub rows: [&'static str; 7],
}

pub const LEVELS: [Level; LEVEL_COUNT] = [
    Level {
        name: "FIRST LIGHT",
        opening: Power::Wide,
        tip: "W WIDE / S SLOW / CATCH THE FALLING CAPSULES",
        chapter: 0,
        speed: 420.0,
        par_seconds: 100,
        rows: [
            "............",
            ".1111..1111.",
            ".1111..1111.",
            ".1111..1111.",
            "..111..111..",
            "...11..11...",
            "............",
        ],
    },
    Level {
        name: "SATELLITES",
        opening: Power::Anchor,
        tip: "A ANCHOR / CATCH, REPOSITION, CLICK TO RELEASE",
        chapter: 0,
        speed: 440.0,
        par_seconds: 110,
        rows: [
            "..11....11..",
            ".1111..1111.",
            "..22....22..",
            "..11....11..",
            ".1111..1111.",
            "..11....11..",
            "....1111....",
        ],
    },
    Level {
        name: "SLIPSTREAM",
        opening: Power::Anchor,
        tip: "AMBER CORES / EACH BLAST REACHES FOUR NEIGHBORS",
        chapter: 0,
        speed: 455.0,
        par_seconds: 100,
        rows: [
            "..111..111..",
            "..1R1..1R1..",
            "..111..111..",
            "............",
            ".111....111.",
            ".1R1....1R1.",
            ".111....111.",
        ],
    },
    Level {
        name: "RESONANCE",
        opening: Power::Anchor,
        tip: "NEIGHBORING CORES CARRY THE REACTION",
        chapter: 0,
        speed: 470.0,
        par_seconds: 115,
        rows: [
            "....1111....",
            "...11RR11...",
            "..11RRRR11..",
            ".121R..R121.",
            "..11R..R11..",
            "...111111...",
            "....1111....",
        ],
    },
    Level {
        name: "PRISM",
        opening: Power::Multi,
        tip: "M MULTIBALL / THREE BALLS, ONE OPENING",
        chapter: 1,
        speed: 480.0,
        par_seconds: 120,
        rows: [
            ".1111..1111.",
            ".1RR1..1RR1.",
            ".12R1..1R21.",
            ".12R1111R21.",
            ".1222..2221.",
            "..111..111..",
            "...11..11...",
        ],
    },
    Level {
        name: "CROSSFADE",
        opening: Power::Anchor,
        tip: "OPEN A ROUTE THROUGH THE TWO RELAY LINES",
        chapter: 1,
        speed: 495.0,
        par_seconds: 125,
        rows: [
            "111......111",
            "1R111..111R1",
            "1RRR1221RRR1",
            ".11R1..1R11.",
            "..1RRRRRR1..",
            "...122221...",
            "....1111....",
        ],
    },
    Level {
        name: "UNDERTOW",
        opening: Power::Wide,
        tip: "BREAK INTO THE POCKETS BEHIND THE ARMOR",
        chapter: 1,
        speed: 510.0,
        par_seconds: 135,
        rows: [
            "1111....1111",
            "1RR1....1RR1",
            "12R111111R21",
            "12RRR22RRR21",
            "12221..12221",
            "11111..11111",
            "..11....11..",
        ],
    },
    Level {
        name: "MOONRISE",
        opening: Power::Multi,
        tip: "FOLLOW THE RELAY AROUND THE OPEN CENTER",
        chapter: 1,
        speed: 525.0,
        par_seconds: 135,
        rows: [
            "..11111111..",
            ".11RRRRRR11.",
            "11RR....RR11",
            "12R......R21",
            "112......211",
            ".122....221.",
            "..111..111..",
        ],
    },
    Level {
        name: "AFTERGLOW",
        opening: Power::Phase,
        tip: "P PHASE / THREE BRICK CONTACTS WITHOUT A BOUNCE",
        chapter: 2,
        speed: 540.0,
        par_seconds: 135,
        rows: [
            "1111....1111",
            "1RR1....1RR1",
            "12R1....1R21",
            "13R111111R31",
            "122211112221",
            ".111....111.",
            "..11....11..",
        ],
    },
    Level {
        name: "PARALLAX",
        opening: Power::Phase,
        tip: "PIERCE THE SHELL, THEN IGNITE THE INNER ROUTE",
        chapter: 2,
        speed: 555.0,
        par_seconds: 150,
        rows: [
            ".1122..2211.",
            "112R3..3R211",
            "12RRR11RRR21",
            "123R1221R321",
            "12RRR11RRR21",
            "11222..22211",
            ".1111..1111.",
        ],
    },
    Level {
        name: "SUPERNOVA",
        opening: Power::Multi,
        tip: "CATCH A RETURN WHILE THE OTHER BALLS KEEP GOING",
        chapter: 2,
        speed: 570.0,
        par_seconds: 145,
        rows: [
            "1....11....1",
            ".11..RR..11.",
            "..1RRRRRR1..",
            "112R2332R211",
            "..1RRRRRR1..",
            ".11..22..11.",
            "1....11....1",
        ],
    },
    Level {
        name: "HOMECOMING",
        opening: Power::Phase,
        tip: "ONE LAST ORBIT / MAKE EACH OPENING COUNT",
        chapter: 2,
        speed: 585.0,
        par_seconds: 160,
        rows: [
            "...111111...",
            "..12RRRR21..",
            ".12RR33RR21.",
            "112R3..3R211",
            ".12RRRRRR21.",
            "..12222221..",
            "...111111...",
        ],
    },
];

pub fn layout(level: usize) -> [u8; 84] {
    let mut bricks = [0; 84];
    for (row, source) in LEVELS[level.min(LEVEL_COUNT - 1)].rows.iter().enumerate() {
        for (col, byte) in source.bytes().enumerate() {
            bricks[row * 12 + col] = match byte {
                b'1'..=b'3' => byte - b'0',
                b'R' => 1,
                _ => 0,
            };
        }
    }
    bricks
}

pub fn cores(level: usize) -> [bool; 84] {
    let mut cores = [false; 84];
    for (row, source) in LEVELS[level.min(LEVEL_COUNT - 1)].rows.iter().enumerate() {
        for (col, byte) in source.bytes().enumerate() {
            cores[row * 12 + col] = byte == b'R';
        }
    }
    cores
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authored_layouts_are_valid_and_distinct() {
        for (index, level) in LEVELS.iter().enumerate() {
            for row in level.rows {
                assert_eq!(row.len(), 12, "{}", level.name);
                assert!(row.bytes().all(|b| matches!(b, b'.' | b'1'..=b'3' | b'R')));
            }
            let board = layout(index);
            let relays = cores(index);
            assert_eq!(relays.iter().any(|&core| core), index >= 2);
            for (i, core) in relays.into_iter().enumerate() {
                if core {
                    assert_eq!(board[i], 1);
                }
            }
            assert!(board.iter().filter(|&&hp| hp > 0).count() >= 24);
            for earlier in 0..index {
                assert_ne!(board, layout(earlier));
            }
        }
    }
}

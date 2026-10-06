//! Hand-authored, fully destructible layouts. Breathing room and readable routes
//! matter more than raw brick count; each chapter introduces a denser rhythm.
pub const LEVEL_COUNT: usize = 12;
pub const CHAPTERS: [&str; 3] = ["DAYBREAK", "BLUE HOUR", "AFTERLIGHT"];

pub struct Level {
    pub name: &'static str,
    pub chapter: usize,
    pub speed: f32,
    pub par_seconds: u32,
    pub rows: [&'static str; 7],
}

pub const LEVELS: [Level; LEVEL_COUNT] = [
    Level {
        name: "FIRST LIGHT",
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
        chapter: 0,
        speed: 440.0,
        par_seconds: 115,
        rows: [
            "..11....11..",
            ".1111..1111.",
            "112211112211",
            ".1111..1111.",
            "..11....11..",
            "....1111....",
            ".....11.....",
        ],
    },
    Level {
        name: "SLIPSTREAM",
        chapter: 0,
        speed: 455.0,
        par_seconds: 120,
        rows: [
            "111.........",
            ".111....111.",
            "..121..111..",
            "...111121...",
            "..111..111..",
            ".111....111.",
            "111.........",
        ],
    },
    Level {
        name: "RESONANCE",
        chapter: 0,
        speed: 470.0,
        par_seconds: 135,
        rows: [
            "....1111....",
            "...112211...",
            "..111..111..",
            ".121....121.",
            "..111..111..",
            "...112211...",
            "....1111....",
        ],
    },
    Level {
        name: "PRISM",
        chapter: 1,
        speed: 480.0,
        par_seconds: 135,
        rows: [
            "..1......1..",
            ".121....121.",
            "11211..11211",
            "122211122221",
            "11211..11211",
            ".111....111.",
            "..1......1..",
        ],
    },
    Level {
        name: "CROSSFADE",
        chapter: 1,
        speed: 495.0,
        par_seconds: 140,
        rows: [
            "11........11",
            ".21......12.",
            "..211..112..",
            "...122221...",
            "..112..211..",
            ".11......11.",
            "11........11",
        ],
    },
    Level {
        name: "UNDERTOW",
        chapter: 1,
        speed: 510.0,
        par_seconds: 150,
        rows: [
            "1111....1111",
            "1221....1221",
            "1..111111..1",
            "1..122221..1",
            "1..111111..1",
            "1221....1221",
            "1111....1111",
        ],
    },
    Level {
        name: "MOONRISE",
        chapter: 1,
        speed: 525.0,
        par_seconds: 150,
        rows: [
            "..111..111..",
            ".122....221.",
            "122......221",
            "121......121",
            "112......211",
            ".111....111.",
            "..111..111..",
        ],
    },
    Level {
        name: "AFTERGLOW",
        chapter: 2,
        speed: 540.0,
        par_seconds: 155,
        rows: [
            "111111111111",
            ".222....222.",
            "..131..131..",
            "...111111...",
            "..121..121..",
            ".111....111.",
            "11........11",
        ],
    },
    Level {
        name: "PARALLAX",
        chapter: 2,
        speed: 555.0,
        par_seconds: 165,
        rows: [
            "1122....2211",
            "1231....1321",
            "121111111121",
            "11..2222..11",
            "121111111121",
            "1221....1221",
            "1111....1111",
        ],
    },
    Level {
        name: "SUPERNOVA",
        chapter: 2,
        speed: 570.0,
        par_seconds: 170,
        rows: [
            "1....11....1",
            ".21..22..12.",
            "..12122121..",
            "112233332211",
            "..12122121..",
            ".11..22..11.",
            "1....11....1",
        ],
    },
    Level {
        name: "HOMECOMING",
        chapter: 2,
        speed: 585.0,
        par_seconds: 180,
        rows: [
            "...112211...",
            "..12233221..",
            ".1221..1221.",
            "1121....1211",
            ".1221..1221.",
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
                _ => 0,
            };
        }
    }
    bricks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authored_layouts_are_valid_and_distinct() {
        for (index, level) in LEVELS.iter().enumerate() {
            for row in level.rows {
                assert_eq!(row.len(), 12, "{}", level.name);
                assert!(row.bytes().all(|b| matches!(b, b'.' | b'1'..=b'3')));
            }
            let board = layout(index);
            assert!(board.iter().filter(|&&hp| hp > 0).count() >= 24);
            for earlier in 0..index {
                assert_ne!(board, layout(earlier));
            }
        }
    }
}

//! A small original 5x7 alphabet. It draws the ARKONK logo and the app icons,
//! and stands in for Noto when a window is too small for it to stay legible.

/// The pixel font's block in the texture atlas: 16 × 6 cells of 8 × 8.
pub const ATLAS_W: usize = 128;
pub const ATLAS_H: usize = 48;

/// Coverage for the atlas block: ASCII 32–127 in 8 × 8 cells; the DEL cell
/// (127, never drawn as text) is solid, so shapes can sample it.
pub fn atlas() -> Vec<u8> {
    let mut out = vec![0; ATLAS_W * ATLAS_H];
    for code in 32_u8..128 {
        let (x, y) = cell(char::from(code));
        for (row, &bits) in glyph(char::from(code)).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    out[(y + row) * ATLAS_W + x + col] = 255;
                }
            }
        }
    }
    for y in 40..48 {
        out[y * ATLAS_W + 120..y * ATLAS_W + 128].fill(255);
    }
    out
}

/// The top-left pixel of a character's cell in the atlas block.
pub fn cell(c: char) -> (usize, usize) {
    let code = (c as usize).clamp(32, 127) - 32;
    (code % 16 * 8, code / 16 * 8)
}

/// Whether the pixel font can draw `c` (after upper-casing ASCII).
pub fn has(c: char) -> bool {
    c == ' ' || (c.is_ascii() && glyph(c.to_ascii_uppercase()) != [0; 7])
}

pub fn glyph(character: char) -> [u8; 7] {
    match character {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 14],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [14, 4, 4, 4, 4, 4, 14],
        'J' => [7, 2, 2, 2, 18, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 25, 21, 19, 19, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [14, 17, 16, 14, 1, 17, 14],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '/' => [1, 1, 2, 4, 8, 16, 16],
        '\\' => [16, 16, 8, 4, 2, 1, 1],
        '+' => [0, 4, 4, 31, 4, 4, 0],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        '=' => [0, 0, 31, 0, 31, 0, 0],
        '.' => [0, 0, 0, 0, 0, 6, 6],
        ',' => [0, 0, 0, 0, 6, 4, 8],
        ':' => [0, 6, 6, 0, 6, 6, 0],
        '!' => [4, 4, 4, 4, 4, 0, 4],
        '?' => [14, 17, 1, 2, 4, 0, 4],
        '(' => [2, 4, 8, 8, 8, 4, 2],
        ')' => [8, 4, 2, 2, 2, 4, 8],
        '[' => [14, 8, 8, 8, 8, 8, 14],
        ']' => [14, 2, 2, 2, 2, 2, 14],
        '%' => [25, 25, 2, 4, 8, 19, 19],
        '&' => [12, 18, 20, 8, 21, 18, 13],
        '_' => [0, 0, 0, 0, 0, 0, 31],
        '<' => [1, 2, 4, 8, 4, 2, 1],
        '>' => [16, 8, 4, 2, 4, 8, 16],
        _ => [0; 7],
    }
}

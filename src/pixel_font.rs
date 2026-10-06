//! A small original 5x7 alphabet, packed once into a single nearest-filtered atlas.
//! UI glyphs then share a texture batch and stay crisp in the fixed scene buffer.
use macroquad::prelude::*;

#[derive(Clone)]
pub struct PixelFont {
    texture: Texture2D,
}
impl PixelFont {
    pub fn new() -> Self {
        let mut image = Image::gen_image_color(128, 48, Color::new(0.0, 0.0, 0.0, 0.0));
        for code in 32..128 {
            let glyph = glyph(char::from(code as u8));
            let x = (code - 32) % 16 * 8;
            let y = (code - 32) / 16 * 8;
            for (row, &bits) in glyph.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        image.set_pixel(x + col, y + row as u32, WHITE);
                    }
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Self { texture }
    }
    fn pixel(size: f32) -> f32 {
        if size >= 28.0 {
            4.0
        } else if size >= 20.0 {
            3.0
        } else if size >= 12.0 {
            2.0
        } else {
            1.0
        }
    }
    pub fn width(text: &str, size: f32) -> f32 {
        if text.is_empty() {
            0.0
        } else {
            (text.chars().count() as f32 * 6.0 - 1.0) * Self::pixel(size)
        }
    }
    pub fn draw(&self, text: &str, x: f32, baseline: f32, size: f32, color: Color) {
        let pixel = Self::pixel(size);
        for (i, character) in text.chars().enumerate() {
            let character = character.to_ascii_uppercase();
            if character == ' ' {
                continue;
            }
            let code = if character.is_ascii() && (' '..='\u{7f}').contains(&character) {
                character as u32
            } else {
                u32::from(b'?')
            } - 32;
            draw_texture_ex(
                &self.texture,
                (x + i as f32 * 6.0 * pixel).round(),
                (baseline - 7.0 * pixel).round(),
                color,
                DrawTextureParams {
                    source: Some(Rect::new(
                        (code % 16 * 8) as f32,
                        (code / 16 * 8) as f32,
                        5.0,
                        7.0,
                    )),
                    dest_size: Some(vec2(5.0 * pixel, 7.0 * pixel)),
                    ..Default::default()
                },
            );
        }
    }
}

fn glyph(character: char) -> [u8; 7] {
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

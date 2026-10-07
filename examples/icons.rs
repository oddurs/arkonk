//! Renders the ARKONK logo into the committed application icons.
//!
//! `cargo run --example icons` rewrites `packaging/icons/`. The output is
//! deterministic, so a rerun with unchanged glyphs leaves git clean.
use macroquad::texture::Image;
use std::{fs, io, path::Path};

// The game's font module also defines the runtime atlas, which this tool never uses.
#[allow(dead_code)]
#[path = "../src/pixel_font.rs"]
mod pixel_font;

// BG, INK and CYAN from src/render.rs, as 8-bit sRGB.
const BG: [u8; 4] = [7, 8, 14, 255];
const INK: [u8; 4] = [237, 242, 250, 255];
const CYAN: [u8; 4] = [84, 222, 245, 255];

const OUT: &str = "packaging/icons";
const PNG_SIZES: [u32; 10] = [16, 24, 32, 48, 64, 96, 128, 256, 512, 1024];
const ICO_SIZES: [u32; 7] = [16, 24, 32, 48, 64, 128, 256];
// Each icns slot names the pixel size it holds; the @2x slots reuse larger art.
const ICNS: [(&[u8; 4], u32); 10] = [
    (b"icp4", 16),
    (b"ic11", 32),
    (b"icp5", 32),
    (b"ic12", 64),
    (b"ic07", 128),
    (b"ic13", 256),
    (b"ic08", 256),
    (b"ic14", 512),
    (b"ic09", 512),
    (b"ic10", 1024),
];

#[derive(Clone, Copy)]
enum Shape {
    /// Edge-to-edge tile for Windows, Linux and Steam.
    Tile,
    /// Apple's icon grid: an 824/1024 body centred on a transparent canvas.
    Mac,
}

fn main() -> io::Result<()> {
    fs::create_dir_all(OUT)?;
    for size in PNG_SIZES {
        fs::write(
            Path::new(OUT).join(format!("arkonk-{size}.png")),
            png(size, Shape::Tile)?,
        )?;
    }
    let ico = ICO_SIZES
        .iter()
        .map(|&s| Ok((s, png(s, Shape::Tile)?)))
        .collect::<io::Result<Vec<_>>>()?;
    fs::write(Path::new(OUT).join("arkonk.ico"), ico_file(&ico))?;
    let icns = ICNS
        .iter()
        .map(|&(kind, s)| Ok((kind, png(s, Shape::Mac)?)))
        .collect::<io::Result<Vec<_>>>()?;
    fs::write(Path::new(OUT).join("arkonk.icns"), icns_file(&icns))?;
    println!("Wrote icons to {OUT}");
    Ok(())
}

fn render(size: u32, shape: Shape) -> Image {
    let mut image = Image::gen_image_color(size as u16, size as u16, [0.0; 4].into());
    let (body, radius) = match shape {
        Shape::Tile => (size as f32, size as f32 * 0.18),
        Shape::Mac => {
            let body = size as f32 * 824.0 / 1024.0;
            (body, body * 0.225)
        }
    };
    let inset = (size as f32 - body) / 2.0;
    for y in 0..size {
        for x in 0..size {
            let coverage = rounded_square_coverage(x, y, inset, body, radius);
            if coverage > 0.0 {
                let mut c = BG;
                c[3] = (coverage * 255.0).round() as u8;
                put(&mut image, x, y, c);
            }
        }
    }
    // Square icons stack the wordmark as ARK over ONK: 17 x 16 cells. Below
    // one pixel per cell it would be illegible, so the smallest sizes show
    // only the cyan O, the logo's accent.
    let span = body * 0.75;
    let cell = (span / 17.0).floor() as u32;
    if cell >= 1 {
        let gap = cell / 8;
        let (w, h) = (17 * cell, 16 * cell);
        let x0 = (size - w) / 2;
        let y0 = (size - h) / 2;
        for (i, ch) in "ARKONK".chars().enumerate() {
            let col = (i % 3) as u32 * 6;
            let row = (i / 3) as u32 * 9;
            glyph(&mut image, ch, x0 + col * cell, y0 + row * cell, cell, gap);
        }
    } else {
        let cell = ((span / 7.0).floor() as u32).max(1);
        let x0 = (size - 5 * cell) / 2;
        let y0 = (size - 7 * cell) / 2;
        glyph(&mut image, 'O', x0, y0, cell, 0);
    }
    image
}

fn glyph(image: &mut Image, ch: char, x0: u32, y0: u32, cell: u32, gap: u32) {
    let color = if ch == 'O' { CYAN } else { INK };
    for (row, &bits) in pixel_font::glyph(ch).iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) == 0 {
                continue;
            }
            let x = x0 + col * cell;
            let y = y0 + row as u32 * cell;
            for dy in 0..cell - gap {
                for dx in 0..cell - gap {
                    put(image, x + dx, y + dy, color);
                }
            }
        }
    }
}

fn put(image: &mut Image, x: u32, y: u32, c: [u8; 4]) {
    // export_png flips rows because it expects bottom-up GPU readback, so
    // store rows bottom-up to come out upright.
    let row = image.height as u32 - 1 - y;
    let i = (row * image.width as u32 + x) as usize * 4;
    image.bytes[i..i + 4].copy_from_slice(&c);
}

/// 4x4 supersampled coverage so rounded corners are smooth at every size.
fn rounded_square_coverage(x: u32, y: u32, inset: f32, body: f32, radius: f32) -> f32 {
    let mut hits = 0;
    for sy in 0..4 {
        for sx in 0..4 {
            let px = x as f32 + (sx as f32 + 0.5) / 4.0 - inset;
            let py = y as f32 + (sy as f32 + 0.5) / 4.0 - inset;
            if !(0.0..body).contains(&px) || !(0.0..body).contains(&py) {
                continue;
            }
            let dx = (radius - px).max(px - (body - radius)).max(0.0);
            let dy = (radius - py).max(py - (body - radius)).max(0.0);
            if dx * dx + dy * dy <= radius * radius {
                hits += 1;
            }
        }
    }
    hits as f32 / 16.0
}

fn png(size: u32, shape: Shape) -> io::Result<Vec<u8>> {
    // Macroquad only encodes PNG to a path, so round-trip through a temporary file.
    let path = std::env::temp_dir().join(format!("arkonk-icon-{}.png", std::process::id()));
    render(size, shape).export_png(&path.to_string_lossy());
    let bytes = fs::read(&path)?;
    fs::remove_file(&path)?;
    Ok(bytes)
}

/// ICO with PNG-compressed entries, supported since Windows Vista.
fn ico_file(images: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * images.len() as u32;
    for (size, data) in images {
        // A zero dimension byte means 256.
        let side = if *size >= 256 { 0 } else { *size as u8 };
        out.extend_from_slice(&[side, side, 0, 0]);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += data.len() as u32;
    }
    for (_, data) in images {
        out.extend_from_slice(data);
    }
    out
}

fn icns_file(images: &[(&[u8; 4], Vec<u8>)]) -> Vec<u8> {
    let total = 8 + images.iter().map(|(_, d)| 8 + d.len()).sum::<usize>();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(b"icns");
    out.extend_from_slice(&(total as u32).to_be_bytes());
    for (kind, data) in images {
        out.extend_from_slice(*kind);
        out.extend_from_slice(&(8 + data.len() as u32).to_be_bytes());
        out.extend_from_slice(data);
    }
    out
}

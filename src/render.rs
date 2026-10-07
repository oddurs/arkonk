use crate::{
    input::Device,
    perf::Perf,
    pixel_font::{PixelFont, glyph},
    storage::Profile,
    text::{TextId, text},
    ui::{self, Screen, Ui},
};
use ark::{
    Events, Game, Medals, Mode, Power, SectorSummary, Stage,
    clock::{DT, TICK_HZ},
    field::{
        BALL_RADIUS as RADIUS, BOTTOM, CELL_H, CELL_W, CELLS, Cell, LEFT, PADDLE_Y, RIGHT, TOP,
        cell_rect,
    },
    geom::V2,
    sectors::{Chapter, SECTOR_COUNT, SectorId},
    tuning::{ANCHOR_CHARGES, MAX_BALLS, PADDLE_HEIGHT, SLOW_SECONDS, WIDE_SECONDS},
};
use macroquad::models::Vertex;
use macroquad::prelude::*;
use std::{
    f32::consts::{FRAC_PI_2, TAU},
    fmt::Write,
};

/// The fixed scene, in scene units; the window shows it scaled and centered.
pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 900.0;

const BG: Color = Color::new(0.027, 0.033, 0.055, 1.0);
const SURFACE: Color = Color::new(0.050, 0.060, 0.092, 1.0);
const RAISED: Color = Color::new(0.078, 0.091, 0.135, 1.0);
const BORDER: Color = Color::new(0.135, 0.155, 0.210, 1.0);
const INK: Color = Color::new(0.93, 0.95, 0.98, 1.0);
const DIM: Color = Color::new(0.53, 0.58, 0.67, 1.0);
const MUTED: Color = Color::new(0.29, 0.33, 0.41, 1.0);
const CYAN: Color = Color::new(0.33, 0.87, 0.96, 1.0);
const AMBER: Color = Color::new(1.0, 0.76, 0.30, 1.0);
const RED: Color = Color::new(1.0, 0.25, 0.33, 1.0);
const PALETTE: [Color; 7] = [
    RED,
    Color::new(1.0, 0.49, 0.22, 1.0),
    AMBER,
    Color::new(0.35, 0.94, 0.60, 1.0),
    CYAN,
    Color::new(0.49, 0.51, 1.0, 1.0),
    Color::new(0.95, 0.39, 0.78, 1.0),
];

fn opacity(c: Color, alpha: f32) -> Color {
    Color::new(c.r, c.g, c.b, alpha)
}
fn shade(c: Color, value: f32) -> Color {
    Color::new(c.r * value, c.g * value, c.b * value, c.a)
}
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a,
    )
}
/// Thousands separators keep large scores readable at a glance.
fn grouped(out: &mut String, value: u32) {
    let start = out.len();
    let _ = write!(out, "{value}");
    let mut i = out.len();
    while i > start + 3 {
        i -= 3;
        out.insert(i, ',');
    }
}
fn clock(out: &mut String, ticks: u32) {
    let seconds = ticks / TICK_HZ;
    let _ = write!(out, "{:02}:{:02}", seconds / 60, seconds % 60);
}

/// Where the fixed scene sits in the window: uniform scale, centered.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub scale: f32,
    pub x: f32,
    pub y: f32,
}
impl View {
    /// `None` while the window has no drawable area (minimized, or zero-sized
    /// mid-transition): there is nothing to draw and no pointer to map.
    pub fn fit(width: f32, height: f32, dpi: f32) -> Option<Self> {
        // `f32::min` ignores NaN, so every input is checked, not just the scale.
        let usable = |v: f32| v.is_finite() && v > 0.0;
        if !(usable(width) && usable(height) && usable(dpi)) {
            return None;
        }
        let scale = (width / WIDTH).min(height / HEIGHT);
        // A letterbox offset on whole physical pixels keeps pixel text even.
        let snap = |value: f32| (value * dpi).round() / dpi;
        Some(Self {
            scale,
            x: snap((width - WIDTH * scale) / 2.0),
            y: snap((height - HEIGHT * scale) / 2.0),
        })
    }
    pub fn current() -> Option<Self> {
        Self::fit(screen_width(), screen_height(), screen_dpi_scale())
    }
    pub fn to_scene(self, x: f32, y: f32) -> V2 {
        V2::new((x - self.x) / self.scale, (y - self.y) / self.scale)
    }
    /// Maps the whole window onto scene units, centering the fixed scene.
    fn camera(&self, width: f32, height: f32) -> Camera2D {
        let w = width / self.scale;
        let h = height / self.scale;
        Camera2D {
            target: vec2(w / 2.0 - self.x / self.scale, h / 2.0 - self.y / self.scale),
            zoom: vec2(2.0 / w, 2.0 / h),
            ..Default::default()
        }
    }
}
/// The pointer in scene units; `None` when the window cannot map it, so a
/// minimized window can never feed a non-finite position to the paddle.
pub fn mouse() -> Option<V2> {
    let (x, y) = mouse_position();
    let p = View::current()?.to_scene(x, y);
    (p.x.is_finite() && p.y.is_finite()).then_some(p)
}

/// Scene-space drawing helpers. Shapes sample a white cell of the font atlas,
/// so geometry and text share one texture and batch into a single draw call;
/// on Metal every draw call is a full-framebuffer render pass. Meshes are built
/// in fixed arrays: drawing allocates nothing.
struct Scene {
    font: PixelFont,
    device: Device,
}
impl Scene {
    fn mesh(&self, vertices: &[Vertex], indices: &[u16]) {
        // SAFETY: main-thread draw recording between frames, as Macroquad's own
        // shape functions do; no other borrow of the context is live.
        let gl = unsafe { get_internal_gl() }.quad_gl;
        gl.texture(Some(self.font.texture()));
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(vertices, indices);
    }
    fn vertex(&self, p: Vec2, color: Color) -> Vertex {
        let uv = PixelFont::WHITE_UV;
        Vertex::new(p.x, p.y, 0.0, uv.x, uv.y, color)
    }
    fn quad(&self, corners: [Vec2; 4], color: Color) {
        self.mesh(&corners.map(|p| self.vertex(p, color)), &[0, 1, 2, 0, 2, 3]);
    }
    fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.quad(
            [
                vec2(x, y),
                vec2(x + w, y),
                vec2(x + w, y + h),
                vec2(x, y + h),
            ],
            color,
        );
    }
    /// Non-overlapping pieces, so translucent fills stay even at the corners.
    fn rounded(&self, x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
        const STEPS: usize = 6;
        let r = r.min(w / 2.0).min(h / 2.0);
        let zero = self.vertex(Vec2::ZERO, color);
        let mut vertices = [zero; 12 + 4 * (STEPS + 2)];
        let mut indices = [0_u16; 18 + 4 * STEPS * 3];
        for (i, (rx, ry, rw, rh)) in [
            (x + r, y, w - 2.0 * r, h),
            (x, y + r, r, h - 2.0 * r),
            (x + w - r, y + r, r, h - 2.0 * r),
        ]
        .into_iter()
        .enumerate()
        {
            for (k, p) in [
                vec2(rx, ry),
                vec2(rx + rw, ry),
                vec2(rx + rw, ry + rh),
                vec2(rx, ry + rh),
            ]
            .into_iter()
            .enumerate()
            {
                vertices[i * 4 + k] = self.vertex(p, color);
            }
            let base = (i * 4) as u16;
            indices[i * 6..i * 6 + 6].copy_from_slice(&[0, 1, 2, 0, 2, 3].map(|k| base + k));
        }
        for (corner, (cx, cy, start)) in [
            (x + r, y + r, 2.0),
            (x + w - r, y + r, 3.0),
            (x + w - r, y + h - r, 0.0),
            (x + r, y + h - r, 1.0),
        ]
        .into_iter()
        .enumerate()
        {
            let center = vec2(cx, cy);
            let first = 12 + corner * (STEPS + 2);
            vertices[first] = self.vertex(center, color);
            for i in 0..=STEPS {
                let a = (start + i as f32 / STEPS as f32) * FRAC_PI_2;
                vertices[first + 1 + i] = self.vertex(center + vec2(a.cos(), a.sin()) * r, color);
            }
            for i in 0..STEPS {
                let at = 18 + (corner * STEPS + i) * 3;
                let base = first as u16;
                indices[at..at + 3].copy_from_slice(&[
                    base,
                    base + 1 + i as u16,
                    base + 2 + i as u16,
                ]);
            }
        }
        self.mesh(&vertices, &indices);
    }
    fn panel(&self, x: f32, y: f32, w: f32, h: f32) {
        self.rounded(x - 1.0, y - 1.0, w + 2.0, h + 2.0, 13.0, BORDER);
        self.rounded(x, y, w, h, 12.0, SURFACE);
    }
    /// Oversized so it also covers the letterbox at any aspect ratio.
    fn cover(&self, color: Color) {
        self.rect(
            -4.0 * WIDTH,
            -4.0 * HEIGHT,
            9.0 * WIDTH,
            9.0 * HEIGHT,
            color,
        );
    }
    fn scrim(&self) {
        self.cover(opacity(BG, 0.78));
    }
    fn line(&self, a: V2, b: V2, thickness: f32, color: Color) {
        let (a, b) = (vec2(a.x, a.y), vec2(b.x, b.y));
        let normal = (b - a).perp();
        let length = normal.length();
        if length < f32::EPSILON {
            return;
        }
        let t = normal * (thickness * 0.5 / length);
        self.quad([a + t, a - t, b - t, b + t], color);
    }
    /// A 20-sided fan, matching Macroquad's `draw_circle`.
    fn circle(&self, p: V2, r: f32, color: Color) {
        const SIDES: usize = 20;
        let center = vec2(p.x, p.y);
        let mut vertices = [self.vertex(center, color); SIDES + 2];
        let mut indices = [0_u16; SIDES * 3];
        for i in 0..=SIDES {
            let a = i as f32 / SIDES as f32 * TAU;
            vertices[i + 1] = self.vertex(center + vec2(a.cos(), a.sin()) * r, color);
            if i < SIDES {
                indices[i * 3..i * 3 + 3].copy_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }
        self.mesh(&vertices, &indices);
    }
    /// A ring from `r` outward by `thickness`, 30 segments.
    fn ring(&self, p: V2, r: f32, thickness: f32, color: Color) {
        const SIDES: usize = 30;
        let center = vec2(p.x, p.y);
        let mut vertices = [self.vertex(center, color); (SIDES + 1) * 2];
        let mut indices = [0_u16; SIDES * 6];
        for i in 0..=SIDES {
            let direction = Vec2::from_angle(i as f32 / SIDES as f32 * TAU);
            vertices[i * 2] = self.vertex(center + direction * r, color);
            vertices[i * 2 + 1] = self.vertex(center + direction * (r + thickness), color);
            if i < SIDES {
                let k = (i * 2) as u16;
                indices[i * 6..i * 6 + 6].copy_from_slice(&[k, k + 1, k + 2, k + 2, k + 1, k + 3]);
            }
        }
        self.mesh(&vertices, &indices);
    }
    /// Macroquad's `draw_rectangle_lines` at thickness 1: a half-unit outline.
    fn frame(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        let t = 0.5;
        let vertices = [
            vec2(x, y),
            vec2(x + w, y),
            vec2(x + w, y + h),
            vec2(x, y + h),
            vec2(x + t, y + t),
            vec2(x + w - t, y + t),
            vec2(x + w - t, y + h - t),
            vec2(x + t, y + h - t),
        ]
        .map(|p| self.vertex(p, color));
        self.mesh(
            &vertices,
            &[
                0, 1, 4, 1, 4, 5, 1, 5, 6, 1, 2, 6, 3, 7, 2, 2, 7, 6, 0, 4, 3, 3, 4, 7,
            ],
        );
    }
    fn text(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.font.draw(text, x, y, size, color);
    }
    fn right(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.text(text, x - self.font.width(text, size), y, size, color);
    }
    fn center_at(&self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.text(text, x - self.font.width(text, size) / 2.0, y, size, color);
    }
    fn centered(&self, text: &str, y: f32, size: f32, color: Color) {
        self.center_at(text, WIDTH / 2.0, y, size, color);
    }
    fn button(&self, r: Rect, label: &str, selected: bool, enabled: bool) {
        if selected && enabled {
            self.rounded(r.x, r.y, r.w, r.h, 10.0, opacity(CYAN, 0.13));
        }
        let color = match (enabled, selected) {
            (false, _) => MUTED,
            (true, true) => CYAN,
            (true, false) => opacity(INK, 0.78),
        };
        self.centered(label, r.y + r.h / 2.0 + 7.0, 14.0, color);
    }
    fn glyph_width(&self, glyph: Glyph, size: f32) -> f32 {
        let pixel = self.font.pixel(size);
        match glyph {
            Glyph::Start => self.font.width("START", size) + 6.0 * pixel,
            _ => 11.0 * pixel,
        }
    }
    /// A filled button cap, centered on the cap height of text at `baseline`.
    fn glyph(&self, glyph: Glyph, x: f32, baseline: f32, size: f32) {
        let pixel = self.font.pixel(size);
        let (label, fill) = match glyph {
            Glyph::A => ("A", PALETTE[3]),
            Glyph::B => ("B", RED),
            Glyph::X => ("X", Color::new(0.30, 0.56, 1.0, 1.0)),
            Glyph::Start => ("START", DIM),
        };
        let w = self.glyph_width(glyph, size);
        let h = 11.0 * pixel;
        self.rounded(x, baseline - 9.0 * pixel, w, h, h / 2.0, fill);
        self.center_at(label, x + w / 2.0, baseline, size, BG);
    }
    fn prompt(&self, parts: &[Part], center: f32, baseline: f32, size: f32, color: Color) {
        let width = |part: &Part| match *part {
            Part::Text(text) => self.font.width(text, size),
            Part::Pad(glyph) => self.glyph_width(glyph, size),
        };
        let mut x = center - parts.iter().map(width).sum::<f32>() / 2.0;
        for part in parts {
            match *part {
                Part::Text(text) => self.text(text, x, baseline, size, color),
                Part::Pad(glyph) => self.glyph(glyph, x, baseline, size),
            }
            x += width(part);
        }
    }
    /// Names the keys or the buttons of whichever device the player is using.
    fn hint(&self, keys: &str, pad: &[Part], baseline: f32, size: f32, color: Color) {
        match self.device {
            Device::KeyboardMouse => self.centered(keys, baseline, size, color),
            Device::Gamepad => self.prompt(pad, WIDTH / 2.0, baseline, size, color),
        }
    }
    fn logo(&self, x: f32, y: f32, cell: f32) {
        for (letter, character) in "ARKONK".chars().enumerate() {
            let color = if character == 'O' { CYAN } else { INK };
            for (row, &bits) in glyph(character).iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        self.rect(
                            x + (letter as f32 * 6.0 + col as f32) * cell,
                            y + row as f32 * cell,
                            cell - 1.5,
                            cell - 1.5,
                            color,
                        );
                    }
                }
            }
        }
    }
}

/// Xbox face-button names; Steam Deck and Steam Input present this layout.
#[derive(Clone, Copy)]
enum Glyph {
    A,
    B,
    X,
    Start,
}
#[derive(Clone, Copy)]
enum Part {
    Text(&'static str),
    Pad(Glyph),
}
use Part::{Pad, Text};

#[derive(Clone, Copy, Default)]
struct Popup {
    pos: V2,
    life: f32,
    value: u32,
}

pub struct Renderer {
    font: PixelFont,
    /// `None` if the driver rejected the shader; frames then keep blended alpha.
    opaque: Option<Material>,
    metal: bool,
    scratch: String,
    trails: [[V2; 12]; MAX_BALLS],
    trail_len: [usize; MAX_BALLS],
    cursor: usize,
    brick_flash: [f32; CELLS],
    previous_bricks: [u8; CELLS],
    popups: [Popup; 16],
    popup_cursor: usize,
    previous_score: u32,
    previous_sector: Option<SectorId>,
    paddle_flash: f32,
    wall_flash: f32,
    pickup_flash: f32,
}
impl Renderer {
    pub fn new() -> Self {
        let metal =
            unsafe { get_internal_gl().quad_context.info().backend == miniquad::Backend::Metal };
        // Rounded shapes use about 2.5 indices per vertex; the default 5,000
        // indices would split a dense frame long before its 10,000 vertices.
        macroquad::window::gl_set_drawcall_buffer_capacity(10_000, 25_000);
        Self {
            font: PixelFont::new(),
            opaque: opaque_material(metal),
            metal,
            scratch: String::with_capacity(256),
            trails: [[V2::default(); 12]; MAX_BALLS],
            trail_len: [0; MAX_BALLS],
            cursor: 0,
            brick_flash: [0.0; CELLS],
            previous_bricks: [0; CELLS],
            popups: [Popup::default(); 16],
            popup_cursor: 0,
            previous_score: 0,
            previous_sector: None,
            paddle_flash: 0.0,
            wall_flash: 0.0,
            pickup_flash: 0.0,
        }
    }
    pub fn backend(&self) -> &'static str {
        if self.metal { "Metal" } else { "OpenGL" }
    }
    pub fn capture(&self, path: &str) {
        // Miniquad's Metal backend has no framebuffer readback. Native Metal
        // captures use macOS window capture in the external test harness.
        if !self.metal {
            get_screen_data().export_png(path);
        }
    }

    pub fn reset(&mut self) {
        self.trail_len.fill(0);
        self.brick_flash.fill(0.0);
        self.previous_bricks.fill(0);
        self.popups.fill(Popup::default());
        self.previous_score = 0;
        self.previous_sector = None;
        self.paddle_flash = 0.0;
        self.wall_flash = 0.0;
        self.pickup_flash = 0.0;
    }
    /// Updates trails and flashes after one tick that raised `events`.
    pub fn record(&mut self, game: &Game, events: Events) {
        if self.previous_sector != Some(game.sector()) {
            self.reset();
            self.previous_sector = Some(game.sector());
            self.previous_bricks = hp_grid(game);
            self.previous_score = game.score();
        }
        for (i, ball) in game.balls().iter().enumerate() {
            if !ball.active || ball.held || game.stage() != Stage::Playing {
                self.trail_len[i] = 0;
                continue;
            }
            self.trails[i][self.cursor] = ball.pos;
            self.trail_len[i] = (self.trail_len[i] + 1).min(12);
        }
        self.cursor = (self.cursor + 1) % 12;
        self.paddle_flash = (self.paddle_flash - DT).max(0.0);
        self.wall_flash = (self.wall_flash - DT).max(0.0);
        self.pickup_flash = (self.pickup_flash - DT).max(0.0);
        if events.paddle {
            self.paddle_flash = 0.16;
        }
        if events.wall {
            self.wall_flash = 0.12;
        }
        if events.pickup {
            self.pickup_flash = 0.65;
        }
        for popup in &mut self.popups {
            popup.life = (popup.life - DT).max(0.0);
            popup.pos.y -= 22.0 * DT;
        }
        let mut popup_spawned = false;
        for (cell, flash) in Cell::all().zip(&mut self.brick_flash) {
            let i = cell.index();
            *flash = (*flash - DT).max(0.0);
            if events.brick && game.board().hp(cell) < self.previous_bricks[i] {
                *flash = 0.18;
                if !popup_spawned && game.score() > self.previous_score {
                    let r = cell_rect(cell);
                    self.popups[self.popup_cursor] = Popup {
                        pos: V2::new(r.x + r.w / 2.0, r.y),
                        life: 0.65,
                        value: (game.score() - self.previous_score).saturating_sub(
                            if events.clear {
                                game.summary().bonus
                            } else {
                                0
                            },
                        ),
                    };
                    self.popup_cursor = (self.popup_cursor + 1) % self.popups.len();
                    popup_spawned = true;
                }
            }
        }
        self.previous_bricks = hp_grid(game);
        self.previous_score = game.score();
    }

    pub fn draw(
        &mut self,
        game: &Game,
        ui: &Ui,
        profile: &Profile,
        alpha: f32,
        perf: Option<&Perf>,
    ) {
        // Macroquad has already cleared the frame; there is nothing to fit.
        let Some(view) = View::current() else {
            return;
        };
        set_camera(&view.camera(screen_width(), screen_height()));
        self.frame(view, screen_dpi_scale(), game, ui, profile, alpha, perf);
    }
    /// OpenGL only: renders one frame offscreen at an exact physical size, as
    /// on a 1x display, and writes it as PNG. Layouts can then be checked at
    /// resolutions larger than the screen running the test.
    pub fn capture_at(
        &mut self,
        (game, ui, profile): (&Game, &Ui, &Profile),
        (width, height): (u32, u32),
        path: &str,
    ) {
        let Some(view) = View::fit(width as f32, height as f32, 1.0) else {
            return;
        };
        if self.metal {
            return;
        }
        let target = render_target(width, height);
        let mut camera = view.camera(width as f32, height as f32);
        camera.render_target = Some(target.clone());
        // Texture readback is bottom-up; render flipped so the PNG is upright.
        camera.zoom.y = -camera.zoom.y;
        set_camera(&camera);
        self.frame(view, 1.0, game, ui, profile, 1.0, None);
        // SAFETY: main thread, between draw calls; executes the batched frame.
        unsafe { get_internal_gl() }.flush();
        target.texture.get_texture_data().export_png(path);
        set_default_camera();
    }
    #[allow(clippy::too_many_arguments)]
    fn frame(
        &mut self,
        view: View,
        dpi: f32,
        game: &Game,
        ui: &Ui,
        profile: &Profile,
        alpha: f32,
        perf: Option<&Perf>,
    ) {
        self.font.density = view.scale * dpi;
        let v = Scene {
            font: self.font.clone(),
            device: ui.device,
        };
        // Painting the background into the scene batch, rather than with
        // `clear_background`, saves a full-framebuffer pass: Macroquad has
        // already cleared once this frame.
        v.cover(BG);
        self.scene(&v, game, ui, profile, alpha, perf);
        // Translucent shapes also blend into framebuffer alpha. Restore an
        // opaque frame so the compositor never shows anything through it.
        if let Some(opaque) = &self.opaque {
            gl_use_material(opaque);
            v.cover(WHITE);
            gl_use_default_material();
        }
    }
    fn scene(
        &mut self,
        v: &Scene,
        game: &Game,
        ui: &Ui,
        profile: &Profile,
        alpha: f32,
        perf: Option<&Perf>,
    ) {
        // The smoke test previews screens the game has not reached.
        let (stage, summary) = ui
            .preview
            .map_or((game.stage(), game.summary()), |p| (p.stage, p.summary));
        if ui.screen != Screen::Play {
            if ui.screen == Screen::Title {
                self.attract(v, ui, profile);
            } else {
                self.sectors(v, ui, profile);
            }
            if ui.save_error {
                v.centered("PROGRESS COULD NOT BE SAVED", 884.0, 11.0, AMBER);
            }
            return;
        }
        self.playfield(v);
        self.bricks(v, game);
        self.effects(v, game);
        self.paddle(v, game);
        for drop in game.capsules() {
            if drop.active {
                v.rounded(
                    drop.pos.x - 15.0,
                    drop.pos.y - 10.0,
                    30.0,
                    20.0,
                    10.0,
                    power_color(drop.power),
                );
            }
        }
        self.balls(v, game, alpha);
        // Keep font-atlas work together after the geometry batches.
        self.hud(v, game, profile);
        for drop in game.capsules() {
            if drop.active {
                v.center_at(
                    capsule_letter(drop.power),
                    drop.pos.x,
                    drop.pos.y + 7.0,
                    13.0,
                    BG,
                );
            }
        }
        for popup in &self.popups {
            if popup.life > 0.0 {
                self.scratch.clear();
                let _ = write!(self.scratch, "+{}", popup.value);
                v.center_at(
                    &self.scratch,
                    popup.pos.x,
                    popup.pos.y,
                    11.0,
                    opacity(INK, (popup.life * 3.0).min(1.0)),
                );
            }
        }
        if !ui.paused && stage == Stage::Playing {
            if game.balls().iter().any(|b| b.active && b.held) {
                v.hint(
                    "CLICK OR SPACE TO RELEASE",
                    &[Pad(Glyph::A), Text(" TO RELEASE")],
                    720.0,
                    11.0,
                    CYAN,
                );
            }
            if game.effects().notice_ticks > 0
                && let Some(power) = game.effects().notice
            {
                let fade = (game.effects().notice_ticks as f32 / 60.0).min(1.0);
                let name = text(TextId::PowerName(power));
                v.centered(name, 687.0, 13.0, opacity(power_color(power), fade));
            }
        }
        if ui.paused {
            v.scrim();
            v.panel(280.0, 296.0, 400.0, 340.0);
            v.centered("PAUSED", 346.0, 22.0, INK);
            self.menu(v, ui.choice, ["RESUME", "RETRY SECTOR", "MAIN MENU"], None);
            v.centered("RETRY RESTARTS FROM THE CHECKPOINT", 584.0, 11.0, DIM);
            if v.device == Device::Gamepad {
                v.prompt(
                    &[
                        Pad(Glyph::A),
                        Text(" SELECT   "),
                        Pad(Glyph::B),
                        Text(" RESUME   "),
                        Pad(Glyph::X),
                        Text(" RETRY"),
                    ],
                    WIDTH / 2.0,
                    610.0,
                    11.0,
                    MUTED,
                );
            } else {
                self.options(v, profile, 610.0);
            }
        } else {
            match stage {
                Stage::Ready => self.ready(v, game),
                Stage::Cleared => self.cleared(v, game, summary),
                Stage::GameOver | Stage::Victory => {
                    v.scrim();
                    v.panel(280.0, 290.0, 400.0, 346.0);
                    v.centered(
                        if stage == Stage::Victory {
                            "JOURNEY COMPLETE"
                        } else {
                            "ONE MORE ORBIT?"
                        },
                        338.0,
                        22.0,
                        INK,
                    );
                    self.scratch.clear();
                    grouped(&mut self.scratch, game.score());
                    let _ = write!(
                        self.scratch,
                        " POINTS   {} MEDALS",
                        profile.progress.medal_count()
                    );
                    v.centered(&self.scratch, 362.0, 11.0, DIM);
                    self.menu(
                        v,
                        ui.choice,
                        [
                            "SECTOR SELECT",
                            if stage == Stage::Victory {
                                "NEW JOURNEY"
                            } else {
                                "RETRY SECTOR"
                            },
                            "MAIN MENU",
                        ],
                        None,
                    );
                    v.centered("YOUR PROGRESS IS SAVED", 590.0, 11.0, DIM);
                }
                Stage::Playing => {}
            }
        }
        if ui.save_error && (ui.paused || stage != Stage::Playing) {
            v.centered("PROGRESS COULD NOT BE SAVED", 860.0, 11.0, AMBER);
        }
        if let Some(perf) = perf {
            v.panel(80.0, 440.0, 420.0, 184.0);
            v.text("PERFORMANCE / CPU", 100.0, 470.0, 13.0, CYAN);
            for (i, line) in perf.lines.iter().enumerate() {
                v.text(line, 100.0, 496.0 + i as f32 * 21.0, 11.0, INK);
            }
        }
    }

    fn playfield(&self, v: &Scene) {
        v.rect(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP, SURFACE);
        // Three walls; the open bottom edge is where a ball drains.
        let edge = mix(BORDER, CYAN, (self.wall_flash * 4.0).min(0.6));
        v.rect(LEFT, TOP, RIGHT - LEFT, 1.0, edge);
        v.rect(LEFT, TOP, 1.0, BOTTOM - TOP, edge);
        v.rect(RIGHT - 1.0, TOP, 1.0, BOTTOM - TOP, edge);
    }
    fn bricks(&self, v: &Scene, game: &Game) {
        // Quiet connections make the actual orthogonal blast routes readable.
        for cell in Cell::all() {
            if !game.board().is_core(cell) || game.board().hp(cell) == 0 {
                continue;
            }
            let r = cell_rect(cell);
            let [_, right, _, down] = cell.neighbors();
            for other in [right, down].into_iter().flatten() {
                if game.board().is_core(other) && game.board().hp(other) > 0 {
                    let next = cell_rect(other);
                    v.line(
                        V2::new(r.x + r.w / 2.0, r.y + r.h / 2.0),
                        V2::new(next.x + next.w / 2.0, next.y + next.h / 2.0),
                        1.0,
                        opacity(AMBER, 0.32),
                    );
                }
            }
        }
        let pulse = 0.7 + 0.15 * (game.stage_ticks() as f32 * DT * 2.0).sin();
        for cell in Cell::all() {
            let i = cell.index();
            let hp = game.board().hp(cell);
            let r = cell_rect(cell);
            let c = if game.board().is_core(cell) {
                AMBER
            } else {
                sector_color(cell.row(), game.sector().sector().chapter)
            };
            let flash = self.brick_flash[i] / 0.18;
            if hp == 0 {
                if flash > 0.0 {
                    v.frame(
                        r.x - (1.0 - flash) * 5.0,
                        r.y - (1.0 - flash) * 5.0,
                        r.w + (1.0 - flash) * 10.0,
                        r.h + (1.0 - flash) * 10.0,
                        opacity(c, flash * 0.75),
                    );
                }
                continue;
            }
            let fill = if game.board().is_core(cell) {
                shade(AMBER, 0.24)
            } else if hp > 1 {
                shade(c, 0.42)
            } else {
                shade(c, 0.80)
            };
            v.rounded(r.x, r.y, r.w, r.h, 4.0, fill);
            v.rect(r.x + 4.0, r.y, r.w - 8.0, 2.0, mix(fill, INK, 0.22));
            if game.board().is_core(cell) {
                let center = V2::new(r.x + r.w / 2.0, r.y + r.h / 2.0);
                for (from, to) in [
                    (V2::new(-6.0, 0.0), V2::new(0.0, -5.0)),
                    (V2::new(0.0, -5.0), V2::new(6.0, 0.0)),
                    (V2::new(6.0, 0.0), V2::new(0.0, 5.0)),
                    (V2::new(0.0, 5.0), V2::new(-6.0, 0.0)),
                ] {
                    v.line(center + from, center + to, 1.5, opacity(AMBER, pulse));
                }
                v.circle(center, 1.5, INK);
            } else if hp > 1 {
                for j in 0..hp {
                    let x = r.x + r.w / 2.0 - f32::from(hp - 1) * 5.0 + f32::from(j) * 10.0;
                    v.circle(V2::new(x, r.y + r.h / 2.0), 2.5, mix(c, INK, 0.5));
                }
            }
            if flash > 0.0 {
                v.rounded(r.x, r.y, r.w, r.h, 4.0, opacity(INK, flash * 0.8));
            }
        }
    }
    fn effects(&self, v: &Scene, game: &Game) {
        for cell in Cell::all() {
            let flash = game.effects().relay_flash[cell.index()];
            if flash == 0 {
                continue;
            }
            let r = cell_rect(cell);
            let progress = 1.0 - f32::from(flash) / 36.0;
            let center = V2::new(r.x + r.w / 2.0, r.y + r.h / 2.0);
            let color = opacity(AMBER, (1.0 - progress) * 0.75);
            v.frame(
                r.x - progress * 8.0,
                r.y - progress * 5.0,
                r.w + progress * 16.0,
                r.h + progress * 10.0,
                color,
            );
            for direction in [
                V2::new(CELL_W, 0.0),
                V2::new(-CELL_W, 0.0),
                V2::new(0.0, CELL_H),
                V2::new(0.0, -CELL_H),
            ] {
                v.line(
                    center + direction * progress * 0.6,
                    center + direction * progress,
                    1.0,
                    color,
                );
            }
        }
        for p in &game.effects().particles {
            if p.life <= 0.0 {
                continue;
            }
            let c = opacity(
                sector_color(p.hue % 7, game.sector().sector().chapter),
                (p.life * 3.0).min(1.0),
            );
            v.line(
                p.pos - p.velocity * 0.012,
                p.pos,
                1.0,
                opacity(c, c.a * 0.5),
            );
            v.rect(p.pos.x - 1.0, p.pos.y - 1.0, 2.0, 2.0, c);
        }
        if self.pickup_flash > 0.0 {
            let progress = 1.0 - self.pickup_flash / 0.65;
            v.ring(
                V2::new(game.paddle().x, PADDLE_Y),
                20.0 + progress * 90.0,
                2.0,
                opacity(CYAN, (1.0 - progress) * 0.6),
            );
        }
    }
    fn paddle(&self, v: &Scene, game: &Game) {
        let paddle = game.paddle().x;
        let x = paddle - game.paddle().width / 2.0;
        let w = game.paddle().width;
        v.rounded(x, PADDLE_Y, w, PADDLE_HEIGHT, PADDLE_HEIGHT / 2.0, INK);
        // The center sends the ball straight up; the ends steer it.
        v.rounded(paddle - 9.0, PADDLE_Y + 5.0, 18.0, 4.0, 2.0, CYAN);
        if game.powers().anchor_charges > 0 || game.balls().iter().any(|b| b.active && b.held) {
            v.rect(x + 12.0, PADDLE_Y - 3.0, w - 24.0, 1.0, CYAN);
            for i in 0..ANCHOR_CHARGES {
                v.circle(
                    V2::new(paddle - 8.0 + f32::from(i) * 8.0, PADDLE_Y + 22.0),
                    2.0,
                    if i < game.powers().anchor_charges {
                        CYAN
                    } else {
                        MUTED
                    },
                );
            }
        }
        if game.powers().wide() {
            v.rect(
                x,
                PADDLE_Y + 28.0,
                w * (game.powers().wide_seconds / WIDE_SECONDS).min(1.0),
                2.0,
                power_color(Power::Wide),
            );
        }
        if game.powers().slow() {
            v.rect(
                x,
                PADDLE_Y + 32.0,
                w * (game.powers().slow_seconds / SLOW_SECONDS).min(1.0),
                2.0,
                power_color(Power::Slow),
            );
        }
        if self.paddle_flash > 0.0 {
            v.rounded(
                x - 2.0,
                PADDLE_Y - 2.0,
                w + 4.0,
                18.0,
                9.0,
                opacity(CYAN, self.paddle_flash * 2.0),
            );
        }
    }
    fn balls(&self, v: &Scene, game: &Game, alpha: f32) {
        for (i, ball) in game.balls().iter().enumerate() {
            if !ball.active {
                continue;
            }
            if ball.held {
                let mut point = ball.pos;
                let mut direction = ball.velocity.normalized();
                for n in 1..=10 {
                    point += direction * 12.0;
                    if point.x < LEFT + RADIUS {
                        point.x = 2.0 * (LEFT + RADIUS) - point.x;
                        direction.x = -direction.x;
                    }
                    if point.x > RIGHT - RADIUS {
                        point.x = 2.0 * (RIGHT - RADIUS) - point.x;
                        direction.x = -direction.x;
                    }
                    v.circle(point, 1.5, opacity(CYAN, 0.6 - n as f32 * 0.04));
                }
            }
            let color = if ball.phase_charges > 0 {
                PALETTE[5]
            } else {
                CYAN
            };
            for n in (0..if ball.held { 0 } else { self.trail_len[i] }).rev() {
                let index = (self.cursor + 12 - 1 - n) % 12;
                let c = opacity(color, 0.22 * (1.0 - n as f32 / 12.0));
                v.circle(self.trails[i][index], RADIUS * (1.0 - n as f32 / 15.0), c);
            }
            let pos = if ball.held {
                ball.pos
            } else {
                ball.previous.lerp(ball.pos, alpha)
            };
            for n in 0..ball.phase_charges {
                v.circle(
                    V2::new(
                        pos.x - f32::from(ball.phase_charges - 1) * 3.0 + f32::from(n) * 6.0,
                        pos.y + 14.0,
                    ),
                    1.5,
                    color,
                );
            }
            v.circle(pos, RADIUS, INK);
        }
    }
    fn hud(&mut self, v: &Scene, game: &Game, profile: &Profile) {
        v.text("SCORE", LEFT, 72.0, 11.0, DIM);
        self.scratch.clear();
        grouped(&mut self.scratch, game.score());
        v.text(&self.scratch, LEFT, 108.0, 22.0, INK);

        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{} {:02}",
            if game.mode() == Mode::Practice {
                "PRACTICE"
            } else {
                "SECTOR"
            },
            game.sector().index() + 1
        );
        v.centered(&self.scratch, 72.0, 11.0, DIM);
        v.centered(text(TextId::SectorName(game.sector())), 104.0, 13.0, INK);
        for id in SectorId::all() {
            v.rect(
                WIDTH / 2.0 - 94.0 + id.index() as f32 * 16.0,
                116.0,
                12.0,
                2.0,
                if id == game.sector() {
                    CYAN
                } else if profile.progress.record(id).medals != Medals::NONE {
                    DIM
                } else {
                    MUTED
                },
            );
        }

        v.right("LIVES", RIGHT, 72.0, 11.0, DIM);
        let shown = game.lives().max(3);
        for i in 0..shown {
            v.circle(
                V2::new(RIGHT - 5.0 - f32::from(shown - 1 - i) * 16.0, 97.0),
                5.0,
                if i < game.lives() { INK } else { MUTED },
            );
        }
    }
    fn ready(&mut self, v: &Scene, game: &Game) {
        let id = game.sector();
        let chapter = id.sector().chapter;
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{}   SECTOR {:02}",
            text(TextId::ChapterName(chapter)),
            id.index() + 1
        );
        v.centered(&self.scratch, 548.0, 11.0, sector_color(0, chapter));
        v.centered(text(TextId::SectorName(id)), 584.0, 22.0, INK);
        v.centered(text(TextId::SectorTip(id)), 614.0, 11.0, DIM);
        v.hint(
            "CLICK OR SPACE TO SERVE",
            &[Pad(Glyph::A), Text(" TO SERVE")],
            664.0,
            13.0,
            CYAN,
        );
        let start = game.balls()[0].pos;
        let direction = game.launch_velocity().normalized();
        for i in 1..=5 {
            v.circle(
                start + direction * (14.0 * i as f32),
                1.5,
                opacity(CYAN, 0.45 - i as f32 * 0.06),
            );
        }
    }
    fn options(&mut self, v: &Scene, profile: &Profile, y: f32) {
        self.scratch.clear();
        let sound = if profile.settings.muted { "OFF" } else { "ON" };
        // The shortcuts are keyboard-only; a pad player still sees the levels.
        let _ = match v.device {
            Device::KeyboardMouse => write!(
                self.scratch,
                "M SOUND {sound}   [ ] VOLUME {}   F FULLSCREEN",
                profile.settings.volume
            ),
            Device::Gamepad => write!(
                self.scratch,
                "SOUND {sound}   VOLUME {}",
                profile.settings.volume
            ),
        };
        v.centered(&self.scratch, y, 11.0, MUTED);
    }
    fn menu(&self, v: &Scene, selected: usize, labels: [&str; 3], disabled: Option<usize>) {
        for (i, label) in labels.iter().enumerate() {
            v.button(ui::menu_rect(i), label, i == selected, disabled != Some(i));
        }
    }
    fn attract(&mut self, v: &Scene, ui: &Ui, profile: &Profile) {
        v.logo(270.0, 170.0, 12.0);
        v.centered("BREAK THE COSMOS", 300.0, 11.0, DIM);
        self.menu(
            v,
            ui.choice,
            ["CONTINUE JOURNEY", "NEW JOURNEY", "SECTOR SELECT"],
            profile.progress.checkpoint().is_none().then_some(0),
        );
        self.scratch.clear();
        if let Some(c) = profile.progress.checkpoint() {
            let _ = write!(
                self.scratch,
                "SAVED AT SECTOR {:02} / {}",
                c.sector.index() + 1,
                text(TextId::SectorName(c.sector))
            );
        } else {
            self.scratch
                .push_str("TWELVE SECTORS ACROSS THREE CHAPTERS");
        }
        v.centered(&self.scratch, 590.0, 11.0, DIM);

        for (i, label) in ["SECTORS", "MEDALS", "BEST"].iter().enumerate() {
            let x = WIDTH / 2.0 + (i as f32 - 1.0) * 140.0;
            self.scratch.clear();
            match i {
                0 => {
                    let _ = write!(
                        self.scratch,
                        "{} / {SECTOR_COUNT}",
                        profile.progress.unlocked_count()
                    );
                }
                1 => {
                    let _ = write!(
                        self.scratch,
                        "{} / {}",
                        profile.progress.medal_count(),
                        SECTOR_COUNT * 3
                    );
                }
                _ => grouped(&mut self.scratch, profile.progress.best_score()),
            }
            v.center_at(label, x, 676.0, 11.0, DIM);
            v.center_at(&self.scratch, x, 702.0, 13.0, INK);
        }

        v.hint(
            "MOUSE OR ARROWS TO MOVE   SPACE OR CLICK TO SERVE   ESC TO PAUSE",
            &[
                Text("STICK OR D-PAD TO MOVE   "),
                Pad(Glyph::A),
                Text(" SERVE   "),
                Pad(Glyph::Start),
                Text(" PAUSE"),
            ],
            832.0,
            11.0,
            DIM,
        );
        self.options(v, profile, 858.0);
    }
    fn sectors(&mut self, v: &Scene, ui: &Ui, profile: &Profile) {
        let back = ui::back_rect();
        v.rounded(back.x, back.y, back.w, back.h, 8.0, RAISED);
        let (center, baseline) = (back.x + back.w / 2.0, back.y + back.h / 2.0 + 4.0);
        match v.device {
            Device::KeyboardMouse => v.center_at("< BACK", center, baseline, 11.0, DIM),
            Device::Gamepad => {
                v.prompt(&[Pad(Glyph::B), Text(" BACK")], center, baseline, 11.0, DIM)
            }
        }
        v.centered("SECTORS", 74.0, 22.0, INK);
        v.centered("PRACTICE RUNS NEVER CHANGE YOUR JOURNEY", 104.0, 11.0, DIM);
        for chapter in Chapter::ALL {
            let r = ui::sector_rect(chapter.first_sector().index());
            let title = text(TextId::ChapterName(chapter));
            v.text(title, r.x + 2.0, r.y - 16.0, 11.0, sector_color(0, chapter));
        }
        for id in SectorId::all() {
            let (i, level) = (id.index(), id.sector());
            let r = ui::sector_rect(i);
            let unlocked = i < profile.progress.unlocked_count();
            let selected = id == ui.sector;
            if selected {
                v.rounded(
                    r.x - 1.5,
                    r.y - 1.5,
                    r.w + 3.0,
                    r.h + 3.0,
                    9.5,
                    if unlocked { CYAN } else { MUTED },
                );
            }
            v.rounded(
                r.x,
                r.y,
                r.w,
                r.h,
                8.0,
                if selected { RAISED } else { SURFACE },
            );
            self.scratch.clear();
            let _ = write!(self.scratch, "{:02}", i + 1);
            v.text(&self.scratch, r.x + 14.0, r.y + 24.0, 11.0, DIM);
            v.text(
                text(TextId::SectorName(id)),
                r.x + 34.0,
                r.y + 24.0,
                11.0,
                if unlocked { INK } else { MUTED },
            );
            for cell in Cell::all() {
                if level.layout.hp[cell.index()] > 0 {
                    v.rect(
                        r.x + 14.0 + cell.col() as f32 * 9.0,
                        r.y + 38.0 + cell.row() as f32 * 7.0,
                        7.0,
                        4.0,
                        if !unlocked {
                            opacity(MUTED, 0.45)
                        } else if level.layout.cores.contains(cell) {
                            AMBER
                        } else {
                            shade(sector_color(cell.row(), level.chapter), 0.8)
                        },
                    );
                }
            }
            if unlocked {
                let record = profile.progress.record(id);
                for (j, medal) in MEDAL_ORDER.into_iter().enumerate() {
                    v.circle(
                        V2::new(r.x + 160.0 + j as f32 * 16.0, r.y + 52.0),
                        4.0,
                        if record.medals.contains(medal) {
                            AMBER
                        } else {
                            MUTED
                        },
                    );
                }
                self.scratch.clear();
                if record.best_ticks > 0 {
                    clock(&mut self.scratch, record.best_ticks);
                } else {
                    self.scratch.push_str("--:--");
                }
                v.text(
                    &self.scratch,
                    r.x + 156.0,
                    r.y + 82.0,
                    11.0,
                    if record.best_ticks > 0 { INK } else { MUTED },
                );
            } else {
                v.text("LOCKED", r.x + 156.0, r.y + 70.0, 11.0, MUTED);
            }
        }

        let level = ui.sector.sector();
        let record = profile.progress.record(ui.sector);
        self.scratch.clear();
        let _ = write!(
            self.scratch,
            "{:02} {}   SWIFT UNDER {} SECONDS",
            ui.sector.index() + 1,
            text(TextId::SectorName(ui.sector)),
            level.par_seconds
        );
        if record.best_ticks > 0 {
            self.scratch.push_str("   BEST ");
            clock(&mut self.scratch, record.best_ticks);
        }
        v.centered(&self.scratch, 702.0, 11.0, INK);
        v.centered(
            "MEDALS   CLEAR: FINISH   CLEAN: NO LIVES LOST   SWIFT: BEAT THE TARGET",
            730.0,
            11.0,
            DIM,
        );
        let open = ui.sector.index() < profile.progress.unlocked_count();
        self.scratch.clear();
        if open {
            let _ = write!(self.scratch, "PLAY SECTOR {:02}", ui.sector.index() + 1);
        } else {
            self.scratch.push_str("CLEAR THE PREVIOUS SECTOR");
        }
        v.button(ui::play_rect(), &self.scratch, true, open);
        v.hint(
            "ARROWS TO BROWSE   ENTER TO PLAY   ESC TO GO BACK",
            &[
                Text("D-PAD TO BROWSE   "),
                Pad(Glyph::A),
                Text(" PLAY   "),
                Pad(Glyph::B),
                Text(" BACK"),
            ],
            858.0,
            11.0,
            MUTED,
        );
    }
    fn cleared(&mut self, v: &Scene, game: &Game, summary: SectorSummary) {
        v.scrim();
        v.panel(280.0, 296.0, 400.0, 340.0);
        v.centered("SECTOR CLEAR", 346.0, 22.0, INK);
        for (i, label) in ["TIME", "BONUS", "BEST CHAIN"].iter().enumerate() {
            let x = WIDTH / 2.0 + (i as f32 - 1.0) * 110.0;
            self.scratch.clear();
            match i {
                0 => clock(&mut self.scratch, summary.ticks),
                1 => {
                    self.scratch.push('+');
                    grouped(&mut self.scratch, summary.bonus);
                }
                _ => {
                    let _ = write!(self.scratch, "{}", summary.best_combo);
                }
            }
            v.center_at(label, x, 388.0, 11.0, DIM);
            v.center_at(&self.scratch, x, 412.0, 13.0, INK);
        }
        for (i, label) in ["CLEAR", "CLEAN", "SWIFT"].iter().enumerate() {
            let x = WIDTH / 2.0 + (i as f32 - 1.0) * 110.0;
            let earned = summary.medals.contains(MEDAL_ORDER[i]);
            v.rounded(
                x - 46.0,
                438.0,
                92.0,
                26.0,
                13.0,
                opacity(if earned { AMBER } else { MUTED }, 0.16),
            );
            v.center_at(label, x, 455.0, 11.0, if earned { AMBER } else { MUTED });
        }
        if summary.life_earned {
            v.centered("CHAPTER COMPLETE / EXTRA LIFE", 498.0, 11.0, CYAN);
        }
        v.button(
            ui::next_rect(),
            if game.mode() == Mode::Practice {
                "BACK TO SECTORS"
            } else {
                "NEXT SECTOR"
            },
            true,
            true,
        );
        v.hint(
            "ENTER OR CLICK",
            &[Text("PRESS "), Pad(Glyph::A)],
            600.0,
            11.0,
            MUTED,
        );
    }
}

/// Writes alpha 1 everywhere and leaves color untouched.
fn opaque_material(metal: bool) -> Option<Material> {
    use miniquad::{BlendFactor, BlendState, Equation, PipelineParams};
    load_material(
        if metal {
            ShaderSource::Msl {
                program: METAL_OPAQUE,
            }
        } else {
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            }
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Zero,
                    BlendFactor::One,
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::One,
                    BlendFactor::Zero,
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .inspect_err(|e| {
        crate::diagnostics::error(format_args!(
            "Opaque frame shader rejected; skipping the alpha pass: {e:?}"
        ))
    })
    .ok()
}
const VERTEX: &str = r#"#version 100
attribute vec3 position;
uniform mat4 Model;
uniform mat4 Projection;
void main() { gl_Position = Projection * Model * vec4(position, 1.0); }
"#;
const FRAGMENT: &str = r#"#version 100
void main() { gl_FragColor = vec4(1.0); }
"#;
// Uniform order follows Macroquad's buffer layout (Projection, then Model).
const METAL_OPAQUE: &str = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms { float4x4 Projection; float4x4 Model; };
struct Vertex { float3 position [[attribute(0)]]; };
struct Raster { float4 position [[position]]; };
vertex Raster vertexShader(Vertex v [[stage_in]], constant Uniforms& u [[buffer(0)]]) {
    Raster out; out.position = u.Projection * u.Model * float4(v.position, 1.0); return out;
}
fragment float4 fragmentShader(Raster in [[stage_in]]) { return float4(1.0); }
"#;

/// The three medals in the order cards show them.
const MEDAL_ORDER: [Medals; 3] = [Medals::CLEAR, Medals::CLEAN, Medals::SWIFT];

/// Hit points per cell, to compare across ticks.
fn hp_grid(game: &Game) -> [u8; CELLS] {
    let mut hp = [0; CELLS];
    for cell in Cell::all() {
        hp[cell.index()] = game.board().hp(cell);
    }
    hp
}

fn sector_color(row: usize, chapter: Chapter) -> Color {
    const BLUE: [usize; 7] = [4, 4, 5, 5, 6, 5, 4];
    const DUSK: [usize; 7] = [6, 0, 1, 2, 1, 0, 6];
    PALETTE[match chapter {
        Chapter::Daybreak => row % 7,
        Chapter::BlueHour => BLUE[row % 7],
        Chapter::Afterlight => DUSK[row % 7],
    }]
}
/// The letter on a capsule: fixed iconography, like a Tetris piece's
/// letter, so string tables never replace it.
fn capsule_letter(power: Power) -> &'static str {
    match power {
        Power::Wide => "W",
        Power::Slow => "S",
        Power::Multi => "M",
        Power::Anchor => "A",
        Power::Phase => "P",
    }
}
fn power_color(power: Power) -> Color {
    match power {
        Power::Wide => PALETTE[3],
        Power::Slow => AMBER,
        Power::Multi => PALETTE[6],
        Power::Anchor => CYAN,
        Power::Phase => PALETTE[5],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_or_invalid_windows_have_no_view() {
        for (w, h, dpi) in [
            (0.0, 0.0, 2.0),
            (1280.0, 0.0, 1.0),
            (f32::NAN, 800.0, 1.0),
            (1280.0, 800.0, 0.0),
            (f32::INFINITY, f32::INFINITY, 1.0),
        ] {
            assert_eq!(View::fit(w, h, dpi), None, "{w}x{h} @{dpi}");
        }
    }
    #[test]
    fn scene_fits_every_target_display_and_maps_the_pointer_back() {
        // Steam Deck, 1080p, 1440p, ultrawide, 4:3, and the minimum window.
        for (w, h) in [
            (1280.0, 800.0),
            (1920.0, 1080.0),
            (2560.0, 1440.0),
            (3440.0, 1440.0),
            (1024.0, 768.0),
            (480.0, 450.0),
        ] {
            for dpi in [1.0, 2.0] {
                let (w, h) = (w / dpi, h / dpi);
                let v = View::fit(w, h, dpi).unwrap();
                let (right, bottom) = (v.x + WIDTH * v.scale, v.y + HEIGHT * v.scale);
                let pixel = 1.0 / dpi;
                assert!(v.x >= 0.0 && v.y >= 0.0, "{w}x{h}: scene clipped");
                assert!(right <= w + pixel && bottom <= h + pixel, "{w}x{h}");
                // One axis fills the window; letterbox bars are even.
                assert!(v.x.min(v.y) <= pixel / 2.0);
                assert!((v.x - (w - right)).abs() <= pixel && (v.y - (h - bottom)).abs() <= pixel);
                let top_left = v.to_scene(v.x, v.y);
                let far = v.to_scene(right, bottom);
                assert!(top_left.x.abs() < 1e-3 && top_left.y.abs() < 1e-3);
                assert!((far.x - WIDTH).abs() < 1e-2 && (far.y - HEIGHT).abs() < 1e-2);
            }
        }
    }
    #[test]
    fn scores_group_thousands() {
        for (value, text) in [
            (0, "0"),
            (999, "999"),
            (1000, "1,000"),
            (1234567, "1,234,567"),
        ] {
            let mut out = String::from("+");
            grouped(&mut out, value);
            assert_eq!(out, format!("+{text}"));
        }
    }
}

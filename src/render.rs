use crate::{
    atlas::{Atlas, Cell},
    input::Device,
    perf::Perf,
    pixel_font,
    storage::Profile,
    ui::{self, Action, Menu, Screen, Ui},
};
use ark::{
    Events, Game, Medals, Mode, Power, SectorSummary, Stage,
    clock::{DT, TICK_HZ},
    field::{
        BALL_RADIUS as RADIUS, BOTTOM, CELL_H, CELL_W, CELLS, Cell as FieldCell, LEFT, PADDLE_Y,
        RIGHT, TOP, cell_rect,
    },
    geom::V2,
    sectors::{Chapter, SECTOR_COUNT, SectorId},
    tuning::{ANCHOR_CHARGES, MAX_BALLS, PADDLE_HEIGHT, SLOW_SECONDS, WIDE_SECONDS},
};
use ark_glyphs::{Fonts, ICON_EM, spec};
use ark_text::{Arg, Form, Locale, Role, TextId, capsule, icon_power};
use macroquad::models::Vertex;
use macroquad::prelude::*;
use std::{
    cell::RefCell,
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

/// The spacing scale, in scene units. Layouts step by these and nothing in
/// between, so related things always sit visibly closer than unrelated ones.
const S8: f32 = 8.0;
const S12: f32 = 12.0;
const S16: f32 = 16.0;
const S24: f32 = 24.0;
const S32: f32 = 32.0;
const S48: f32 = 48.0;
/// A label and its value, or a name and its detail.
const PAIR: f32 = S8;
/// Between the groups of one section, such as items of a hint row.
const GROUP: f32 = S24;
/// Between the sections of a screen.
const SECTION: f32 = S48;
/// Inside a panel's edge.
const PAD: f32 = S32;

/// The height of a role's capitals in scene units, as laid out. Gaps on
/// the spacing scale run from one line's baseline to the next line's
/// capitals, so a line's baseline is the one above plus gap plus this.
const fn cap_height(role: Role) -> f32 {
    // Noto Sans capitals are 714 units of its 1000-unit em.
    spec::style(role).0 * 0.714
}

/// Side margin for full-width text, and the widest a centred line may be.
const MARGIN: f32 = 64.0;
const FULL: f32 = WIDTH - 2.0 * MARGIN;
/// Baseline-to-baseline for body text.
const LINE: f32 = 26.0;
/// Baseline-to-baseline for wrapped lines of `role`: 1.3 em, so body text
/// steps by `LINE`.
fn leading(role: Role) -> f32 {
    spec::style(role).0 * 1.3
}
/// The footer's last baseline; rows stack upward from it.
const FOOTER: f32 = 872.0;
/// Baseline-to-baseline distance between footer rows: captions are smaller
/// than body text, so the rows need more air to read as separate lines.
const FOOTER_LINE: f32 = 30.0;
/// Text width inside the pause and results panels.
const PANEL: f32 = 432.0;

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
        // A letterbox offset on whole physical pixels keeps glyphs on the pixel grid.
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

/// How a line sits in its slot.
#[derive(Clone, Copy, PartialEq)]
enum Align {
    Left,
    Center,
    Right,
}

/// The room one line of text may take: from `x`, `w` wide, on baseline `y`.
#[derive(Clone, Copy)]
struct Slot {
    x: f32,
    w: f32,
    y: f32,
    align: Align,
}
impl Slot {
    fn centered(cx: f32, w: f32, y: f32) -> Self {
        Self {
            x: cx - w / 2.0,
            w,
            y,
            align: Align::Center,
        }
    }
    /// A full-width line, centred on the scene.
    fn line(y: f32) -> Self {
        Self::centered(WIDTH / 2.0, FULL, y)
    }
    fn left(x: f32, w: f32, y: f32) -> Self {
        Self {
            x,
            w,
            y,
            align: Align::Left,
        }
    }
    fn right(right: f32, w: f32, y: f32) -> Self {
        Self {
            x: right - w,
            w,
            y,
            align: Align::Right,
        }
    }
}

/// Text that does not fit where the layout puts it, or has no glyph. The
/// layout tests collect and read these; a normal frame records nothing.
#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub struct Misfit {
    pub text: String,
    pub need: f32,
    pub room: f32,
    pub missing: Option<char>,
}

/// How one role is set at the current density.
#[derive(Clone, Copy)]
enum Face {
    /// A baked Noto strike, drawn 1:1.
    Noto(u8),
    /// The 5×7 font with cells this many physical pixels square.
    Pixel(f32),
}

/// One frame's drawing context: geometry and text in scene units, batched
/// into the shared atlas texture. Meshes are built in fixed arrays and text
/// is formatted into a reused buffer, so drawing allocates nothing.
struct Scene<'a> {
    /// `None` when recording layout for tests: nothing reaches a GPU.
    texture: Option<&'a Texture2D>,
    atlas: &'a Atlas,
    fonts: &'a Fonts,
    locale: Locale,
    /// Physical pixels per scene unit.
    density: f32,
    device: Device,
    buffer: RefCell<String>,
    misfits: Option<&'a RefCell<Vec<Misfit>>>,
}

impl Scene<'_> {
    fn mesh(&self, vertices: &[Vertex], indices: &[u16]) {
        let Some(texture) = self.texture else { return };
        // SAFETY: main-thread draw recording between frames, as Macroquad's own
        // shape functions do; no other borrow of the context is live.
        let gl = unsafe { get_internal_gl() }.quad_gl;
        gl.texture(Some(texture));
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(vertices, indices);
    }
    fn uv(&self, x: f32, y: f32) -> Vec2 {
        vec2(x / self.atlas.width as f32, y / self.atlas.height as f32)
    }
    fn vertex(&self, p: Vec2, color: Color) -> Vertex {
        let uv = self.uv(Atlas::WHITE.0, Atlas::WHITE.1);
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
    /// A texture region drawn at physical pixel position `(px, py)`, 1:1.
    fn sprite(
        &self,
        (px, py): (f32, f32),
        (sx, sy, sw, sh): (f32, f32, f32, f32),
        scale: f32,
        color: Color,
    ) {
        let d = self.density;
        let (x, y, w, h) = (px / d, py / d, sw * scale / d, sh * scale / d);
        let (u0, u1) = (self.uv(sx, sy), self.uv(sx + sw, sy + sh));
        let corners = [
            (vec2(x, y), vec2(u0.x, u0.y)),
            (vec2(x + w, y), vec2(u1.x, u0.y)),
            (vec2(x + w, y + h), vec2(u1.x, u1.y)),
            (vec2(x, y + h), vec2(u0.x, u1.y)),
        ];
        let vertices = corners.map(|(p, uv)| Vertex::new(p.x, p.y, 0.0, uv.x, uv.y, color));
        self.mesh(&vertices, &[0, 1, 2, 0, 2, 3]);
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
    /// The edge of `bounds` rounded by `r`, `t` thick, drawn inside it.
    fn outline(&self, bounds: Rect, r: f32, t: f32, color: Color) {
        const STEPS: usize = 6;
        let Rect { x, y, w, h } = bounds;
        let r = r.min(w / 2.0).min(h / 2.0).max(t);
        self.rect(x + r, y, w - 2.0 * r, t, color);
        self.rect(x + r, y + h - t, w - 2.0 * r, t, color);
        self.rect(x, y + r, t, h - 2.0 * r, color);
        self.rect(x + w - t, y + r, t, h - 2.0 * r, color);
        let zero = self.vertex(Vec2::ZERO, color);
        let mut vertices = [zero; 4 * (STEPS + 1) * 2];
        let mut indices = [0_u16; 4 * STEPS * 6];
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
            let first = corner * (STEPS + 1) * 2;
            for i in 0..=STEPS {
                let direction = Vec2::from_angle((start + i as f32 / STEPS as f32) * FRAC_PI_2);
                vertices[first + i * 2] = self.vertex(center + direction * r, color);
                vertices[first + i * 2 + 1] = self.vertex(center + direction * (r - t), color);
                if i < STEPS {
                    let k = (first + i * 2) as u16;
                    let at = (corner * STEPS + i) * 6;
                    indices[at..at + 6].copy_from_slice(&[k, k + 1, k + 2, k + 2, k + 1, k + 3]);
                }
            }
        }
        self.mesh(&vertices, &indices);
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

    // Text.

    /// How `role` is set here. Below the smallest legible Noto strike the
    /// 5×7 font takes over, for text it can spell; the Small and Compact
    /// layouts will decide the rest.
    fn face(&self, role: Role, text: &str) -> Face {
        match spec::ppem(role, self.density) {
            Some(ppem) => Face::Noto(ppem),
            None if text.chars().all(pixel_font::has) => {
                Face::Pixel(if role == Role::Display { 2.0 } else { 1.0 })
            }
            None => Face::Noto(spec::rungs(role).min().unwrap_or(spec::LADDER[0])),
        }
    }
    /// The advance width of `text`, in scene units.
    fn measure(&self, text: &str, role: Role) -> f32 {
        match self.face(role, text) {
            Face::Noto(ppem) => {
                let (_, weight, tracking) = spec::style(role);
                let tracking = self.fonts.tracking(tracking, ppem);
                self.fonts.measure(text, weight, ppem, tracking) / self.density
            }
            Face::Pixel(cell) => {
                let n = text.chars().count() as f32;
                (n * 6.0 - 1.0).max(0.0) * cell / self.density
            }
        }
    }
    /// The height of capitals, in scene units, for centring a line.
    fn cap(&self, role: Role) -> f32 {
        let (size, weight, _) = spec::style(role);
        match (self.face(role, "A"), self.fonts.latin.face(weight)) {
            (Face::Noto(ppem), Some(f)) => {
                f32::from(f.cap_height) * f32::from(ppem) / f32::from(f.units_per_em) / self.density
            }
            _ => size * 0.7,
        }
    }
    fn missing(&self, text: &str, c: char) {
        if let Some(log) = self.misfits {
            log.borrow_mut().push(Misfit {
                text: text.into(),
                need: 0.0,
                room: 0.0,
                missing: Some(c),
            });
        }
    }
    /// Draws `text` with its left end at `x` and its baseline at `y`.
    fn draw(&self, text: &str, role: Role, x: f32, y: f32, color: Color) {
        let d = self.density;
        // Whole physical pixels: the view's offset is snapped too, so every
        // glyph lands exactly on the pixel grid.
        let (ox, oy) = ((x * d).round(), (y * d).round());
        match self.face(role, text) {
            Face::Noto(ppem) => {
                let (_, weight, tracking) = spec::style(role);
                let tracking = self.fonts.tracking(tracking, ppem);
                self.fonts.layout(text, weight, ppem, tracking, |p| {
                    if let Some(power) = icon_power(p.c) {
                        let x = (ox + p.x.round()) / d;
                        self.chip(power, x, y, f32::from(ppem) / d, role);
                        return;
                    }
                    let cell = p
                        .glyph
                        .and_then(|g| self.atlas.glyph((p.source, weight, ppem, g)));
                    match cell {
                        Some(Cell { w: 0, .. }) => {}
                        Some(c) => self.sprite(
                            (ox + p.x.round() + f32::from(c.left), oy - f32::from(c.top)),
                            (
                                f32::from(c.x),
                                f32::from(c.y),
                                f32::from(c.w),
                                f32::from(c.h),
                            ),
                            1.0,
                            color,
                        ),
                        None => self.missing(text, p.c),
                    }
                });
            }
            Face::Pixel(cell) => {
                for (i, c) in text.chars().enumerate() {
                    let (sx, sy) = pixel_font::cell(c.to_ascii_uppercase());
                    self.sprite(
                        (ox + i as f32 * 6.0 * cell, oy - 7.0 * cell),
                        (sx as f32, sy as f32, 5.0, 7.0),
                        cell,
                        color,
                    );
                }
            }
        }
    }
    /// A capsule as the player sees it falling, `size` tall, sitting on the
    /// capitals of `beside` text whose baseline is `baseline`.
    fn chip(&self, power: Power, x: f32, baseline: f32, size: f32, beside: Role) {
        let (w, cy) = (ICON_EM * size, baseline - self.cap(beside) / 2.0);
        self.rounded(x, cy - size / 2.0, w, size, size / 2.0, power_color(power));
        let mut letter = [0; 4];
        let letter = capsule(power).encode_utf8(&mut letter);
        let y = cy + self.cap(Role::Label) / 2.0;
        self.put(letter, Role::Label, Slot::centered(x + w / 2.0, w, y), BG);
    }
    /// Draws `text` aligned in `slot`; text wider than the slot is drawn
    /// anyway and reported to the layout tests.
    fn put(&self, text: &str, role: Role, slot: Slot, color: Color) -> f32 {
        let width = self.measure(text, role);
        if width > slot.w + 0.5
            && let Some(log) = self.misfits
        {
            log.borrow_mut().push(Misfit {
                text: text.into(),
                need: width,
                room: slot.w,
                missing: None,
            });
        }
        let x = match slot.align {
            Align::Left => slot.x,
            Align::Center => slot.x + (slot.w - width) / 2.0,
            Align::Right => slot.x + slot.w - width,
        };
        self.draw(text, role, x, slot.y, color);
        width
    }
    /// Formats `id` into the scene's buffer and hands it to `with`.
    fn format<R>(&self, id: TextId, args: &[Arg], form: Form, with: impl FnOnce(&str) -> R) -> R {
        let mut buffer = self.buffer.borrow_mut();
        buffer.clear();
        // Capsules come out as icon marks, which `draw` sets as chips.
        let _ = ark_text::write_icons(&mut *buffer, self.locale, form, id, args);
        with(&buffer)
    }
    /// Sets `id` in `slot`, switching to its short wording if the full one
    /// does not fit.
    fn say(&self, id: TextId, args: &[Arg], role: Role, slot: Slot, color: Color) {
        let fits = self.format(id, args, Form::Full, |t| self.measure(t, role) <= slot.w);
        let form = if fits { Form::Full } else { Form::Short };
        self.format(id, args, form, |t| self.put(t, role, slot, color));
    }
    fn width_of(&self, id: TextId, args: &[Arg], role: Role) -> f32 {
        self.format(id, args, Form::Full, |t| self.measure(t, role))
    }
    /// Sets `id` across up to `max` lines from baseline `slot.y`, wrapping
    /// at word (or, in Chinese and Japanese, character) boundaries.
    /// Returns the lines used; more than `max` is reported.
    fn paragraph(
        &self,
        (id, args): (TextId, &[Arg]),
        role: Role,
        slot: Slot,
        max: usize,
        color: Color,
    ) -> usize {
        self.format(id, args, Form::Full, |text| {
            let mut lines = 0;
            let mut line = |line: &str| {
                let y = slot.y + lines as f32 * leading(role);
                if lines < max {
                    self.put(line, role, Slot { y, ..slot }, color);
                } else if let Some(log) = self.misfits {
                    log.borrow_mut().push(Misfit {
                        text: text.into(),
                        need: (lines + 1) as f32,
                        room: max as f32,
                        missing: None,
                    });
                }
                lines += 1;
            };
            match self.face(role, text) {
                Face::Noto(ppem) => {
                    let (_, weight, tracking) = spec::style(role);
                    let tracking = self.fonts.tracking(tracking, ppem);
                    let room = slot.w * self.density;
                    self.fonts.wrap(text, weight, ppem, tracking, room, line);
                }
                // The pixel font spells only Latin, so spaces are the breaks.
                Face::Pixel(_) => {
                    let mut rest = text.trim();
                    while !rest.is_empty() {
                        let fits = |end: usize| self.measure(&rest[..end], role) <= slot.w;
                        let end = if fits(rest.len()) {
                            rest.len()
                        } else {
                            rest.match_indices(' ')
                                .map(|(i, _)| i)
                                .take_while(|&i| fits(i))
                                .last()
                                .unwrap_or(rest.len())
                        };
                        line(&rest[..end]);
                        rest = rest[end..].trim_start();
                    }
                }
            }
            lines.min(max)
        })
    }

    // Controls.

    fn cap_width(&self, cap: Cap) -> f32 {
        match cap {
            Cap::Key(label) => (self.measure(label, Role::Label) + 12.0).max(24.0),
            Cap::Pad(Glyph::Start) => 30.0,
            Cap::Pad(_) => 22.0,
        }
    }
    /// A key or button cap, centred on the capitals of body text at `baseline`.
    fn cap_glyph(&self, cap: Cap, x: f32, baseline: f32, beside: Role) {
        let w = self.cap_width(cap);
        let cy = baseline - self.cap(beside) / 2.0;
        match cap {
            Cap::Key(label) => {
                self.rounded(x, cy - 12.0, w, 24.0, 6.0, BORDER);
                self.rounded(x + 1.0, cy - 11.0, w - 2.0, 22.0, 5.0, RAISED);
                let y = cy + self.cap(Role::Label) / 2.0;
                self.put(label, Role::Label, Slot::centered(x + w / 2.0, w, y), INK);
            }
            Cap::Pad(Glyph::Start) => {
                self.rounded(x, cy - 10.0, w, 20.0, 10.0, DIM);
                for dy in [-4.0, 0.0, 4.0] {
                    self.rect(x + 9.0, cy + dy - 0.75, 12.0, 1.5, BG);
                }
            }
            Cap::Pad(glyph) => {
                let (label, fill) = match glyph {
                    Glyph::A => ("A", PALETTE[3]),
                    Glyph::B => ("B", RED),
                    _ => ("X", Color::new(0.30, 0.56, 1.0, 1.0)),
                };
                self.circle(V2::new(x + w / 2.0, cy), w / 2.0, fill);
                let y = cy + self.cap(Role::Label) / 2.0;
                self.put(label, Role::Label, Slot::centered(x + w / 2.0, w, y), BG);
            }
        }
    }
    fn item_width(&self, item: &Item, role: Role) -> f32 {
        let cap = item.cap.map_or(0.0, |c| self.cap_width(c) + PAIR);
        cap + self.width_of(item.id, item.arg.as_slice(), role)
    }
    /// Lays `items` into centred lines of at most `FULL` width; calls
    /// `line` with each line's items and width. Returns the line count.
    fn pack(&self, items: &[Item], role: Role, mut line: impl FnMut(&[Item], f32)) -> usize {
        let mut start = 0;
        let mut lines = 0;
        while start < items.len() {
            let mut width = self.item_width(&items[start], role);
            let mut end = start + 1;
            while end < items.len() {
                let next = width + GROUP + self.item_width(&items[end], role);
                if next > FULL {
                    break;
                }
                width = next;
                end += 1;
            }
            line(&items[start..end], width);
            lines += 1;
            start = end;
        }
        lines
    }
    /// Draws one packed line of hint items, centred, on `baseline`.
    fn hint_line(&self, items: &[Item], width: f32, baseline: f32, color: Color, role: Role) {
        let mut x = WIDTH / 2.0 - width.min(FULL) / 2.0;
        for item in items {
            if let Some(cap) = item.cap {
                self.cap_glyph(cap, x, baseline, role);
                x += self.cap_width(cap) + PAIR;
            }
            let w = self.width_of(item.id, item.arg.as_slice(), role);
            self.say(
                item.id,
                item.arg.as_slice(),
                role,
                Slot::left(x, w.min(FULL), baseline),
                color,
            );
            x += w + GROUP;
        }
    }
    /// Rows of hints stacked up from the bottom of the screen, in the
    /// caption tier. A save failure, when there is one, sits on top in ink:
    /// it is the one footer line that is news.
    fn footer(&self, rows: &[&[Item]], save_error: bool) {
        let count: usize = rows
            .iter()
            .map(|items| self.pack(items, Role::Caption, |_, _| {}))
            .sum::<usize>()
            + usize::from(save_error);
        if count > 4
            && let Some(log) = self.misfits
        {
            log.borrow_mut().push(Misfit {
                text: "footer".into(),
                need: count as f32,
                room: 4.0,
                missing: None,
            });
        }
        let mut y = FOOTER - (count.saturating_sub(1)) as f32 * FOOTER_LINE;
        if save_error {
            self.say(TextId::SaveFailed, &[], Role::Caption, Slot::line(y), INK);
            y += FOOTER_LINE;
        }
        for &items in rows {
            self.pack(items, Role::Caption, |line, width| {
                self.hint_line(line, width, y, DIM, Role::Caption);
                y += FOOTER_LINE;
            });
        }
    }
    /// A row of label–value pairs, centred: `POINTS 12,400   MEDALS 7`.
    fn pairs(&self, pairs: &[(TextId, u32)], baseline: f32, room: f32) {
        let pair_width = |&(label, n): &(TextId, u32)| {
            let value = Figures::count(self.locale, n);
            self.width_of(label, &[], Role::Label) + PAIR + self.measure(value.as_str(), Role::Body)
        };
        let width = pairs.iter().map(pair_width).sum::<f32>() + GROUP * (pairs.len() as f32 - 1.0);
        if width > room
            && let Some(log) = self.misfits
        {
            log.borrow_mut().push(Misfit {
                text: "label and value pairs".into(),
                need: width,
                room,
                missing: None,
            });
        }
        let mut x = WIDTH / 2.0 - width / 2.0;
        for &(label, n) in pairs {
            let w = self.width_of(label, &[], Role::Label);
            self.say(label, &[], Role::Label, Slot::left(x, w, baseline), DIM);
            x += w + PAIR;
            let value = Figures::count(self.locale, n);
            x += self.put(
                value.as_str(),
                Role::Body,
                Slot::left(x, room, baseline),
                INK,
            ) + GROUP;
        }
    }
    /// The screen's primary action is filled; every other action is plain
    /// text. The focused one, primary or not, gets a cyan edge and label.
    /// A `detail` caption sits under the label, inside the button.
    fn button(
        &self,
        r: Rect,
        id: TextId,
        args: &[Arg],
        (primary, focused): (bool, bool),
        detail: Option<(TextId, &[Arg])>,
    ) {
        if primary {
            self.rounded(r.x, r.y, r.w, r.h, 10.0, opacity(CYAN, 0.13));
        }
        if focused {
            self.outline(r, 10.0, 1.5, opacity(CYAN, 0.7));
        }
        let color = if focused { CYAN } else { INK };
        let line = |y| Slot::centered(r.x + r.w / 2.0, r.w - 2.0 * S16, y);
        match detail {
            None => {
                let baseline = r.y + r.h / 2.0 + self.cap(Role::Body) / 2.0;
                self.say(id, args, Role::Body, line(baseline), color);
            }
            Some((detail, detail_args)) => {
                let block = cap_height(Role::Body) + PAIR + cap_height(Role::Caption);
                let name = r.y + (r.h - block) / 2.0 + cap_height(Role::Body);
                self.say(id, args, Role::Body, line(name), color);
                let below = name + PAIR + cap_height(Role::Caption);
                self.say(detail, detail_args, Role::Caption, line(below), DIM);
            }
        }
    }
    fn logo(&self, x: f32, y: f32, cell: f32) {
        for (letter, character) in "ARKONK".chars().enumerate() {
            let color = if character == 'O' { CYAN } else { INK };
            for (row, &bits) in pixel_font::glyph(character).iter().enumerate() {
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
    /// A small padlock for sectors not yet open.
    fn padlock(&self, cx: f32, cy: f32, color: Color) {
        self.ring(V2::new(cx, cy - 4.0), 5.0, 2.0, color);
        self.rounded(cx - 8.0, cy - 3.0, 16.0, 12.0, 2.5, color);
    }
}

/// Xbox face-button names; Steam Deck and Steam Input present this layout.
#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    A,
    B,
    X,
    Start,
}
/// What sits before a hint: a keyboard key or a gamepad button.
#[derive(Clone, Copy, PartialEq)]
enum Cap {
    Key(&'static str),
    Pad(Glyph),
}
/// One hint: an optional cap, then text.
#[derive(Clone, Copy)]
struct Item {
    cap: Option<Cap>,
    id: TextId,
    arg: Option<Arg>,
}
const fn hint(id: TextId) -> Item {
    Item {
        cap: None,
        id,
        arg: None,
    }
}
const fn pad(glyph: Glyph, id: TextId) -> Item {
    Item {
        cap: Some(Cap::Pad(glyph)),
        id,
        arg: None,
    }
}

#[derive(Clone, Copy, Default)]
struct Popup {
    pos: V2,
    life: f32,
    value: u32,
}

/// What the renderer remembers between ticks: trails, flashes, popups.
pub struct Fx {
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
impl Default for Fx {
    fn default() -> Self {
        Self {
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
}

/// The type for one locale: its fonts and the atlas built from them.
struct Type {
    locale: Locale,
    fonts: Fonts,
    atlas: Atlas,
}
impl Type {
    fn new(locale: Locale) -> Self {
        // A locale this build cannot draw falls back to English. The linked
        // atlases are unpacked by the ark-glyphs tests, so English failing
        // too would mean a corrupt binary, with nothing left to draw with.
        let (locale, fonts) = match ark_glyphs::fonts(locale) {
            Ok(fonts) => (locale, fonts),
            Err(e) => {
                crate::diagnostics::error(format_args!(
                    "No glyphs for {locale:?} ({e:?}); using English"
                ));
                (
                    Locale::En,
                    ark_glyphs::fonts(Locale::En).expect("the English atlas is linked and tested"),
                )
            }
        };
        let atlas =
            Atlas::build(&fonts).expect("linked atlases inflate; the tests unpack every strike");
        Self {
            locale,
            fonts,
            atlas,
        }
    }
}

pub struct Renderer {
    kind: Type,
    texture: Option<Texture2D>,
    /// `None` if the driver rejected the shader; frames then keep blended alpha.
    opaque: Option<Material>,
    metal: bool,
    text: String,
    fx: Fx,
}
impl Renderer {
    pub fn new(locale: Locale) -> Self {
        let metal =
            unsafe { get_internal_gl().quad_context.info().backend == miniquad::Backend::Metal };
        // Rounded shapes use about 2.5 indices per vertex; the default 5,000
        // indices would split a dense frame long before its 10,000 vertices.
        macroquad::window::gl_set_drawcall_buffer_capacity(10_000, 25_000);
        let mut renderer = Self {
            kind: Type::new(locale),
            texture: None,
            opaque: opaque_material(metal),
            metal,
            text: String::with_capacity(256),
            fx: Fx::default(),
        };
        renderer.upload();
        renderer
    }
    fn upload(&mut self) {
        let atlas = &self.kind.atlas;
        let texture = Texture2D::from_rgba8(atlas.width as u16, atlas.height as u16, &atlas.rgba());
        // Glyphs are drawn 1:1 on whole pixels, and the pixel font at whole
        // multiples, so nearest sampling is exact.
        texture.set_filter(FilterMode::Nearest);
        self.texture = Some(texture);
    }
    /// Switches language, rebuilding the atlas for its scripts.
    pub fn set_locale(&mut self, locale: Locale) {
        if locale != self.kind.locale {
            self.kind = Type::new(locale);
            self.upload();
        }
    }
    pub fn locale(&self) -> Locale {
        self.kind.locale
    }
    pub fn atlas_size(&self) -> (usize, usize) {
        (self.kind.atlas.width, self.kind.atlas.height)
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
        self.fx = Fx::default();
    }
    /// Updates trails and flashes after one tick that raised `events`.
    pub fn record(&mut self, game: &Game, events: Events) {
        let fx = &mut self.fx;
        if fx.previous_sector != Some(game.sector()) {
            *fx = Fx::default();
            fx.previous_sector = Some(game.sector());
            fx.previous_bricks = hp_grid(game);
            fx.previous_score = game.score();
        }
        for (i, ball) in game.balls().iter().enumerate() {
            if !ball.active || ball.held || game.stage() != Stage::Playing {
                fx.trail_len[i] = 0;
                continue;
            }
            fx.trails[i][fx.cursor] = ball.pos;
            fx.trail_len[i] = (fx.trail_len[i] + 1).min(12);
        }
        fx.cursor = (fx.cursor + 1) % 12;
        fx.paddle_flash = (fx.paddle_flash - DT).max(0.0);
        fx.wall_flash = (fx.wall_flash - DT).max(0.0);
        fx.pickup_flash = (fx.pickup_flash - DT).max(0.0);
        if events.paddle {
            fx.paddle_flash = 0.16;
        }
        if events.wall {
            fx.wall_flash = 0.12;
        }
        if events.pickup {
            fx.pickup_flash = 0.65;
        }
        for popup in &mut fx.popups {
            popup.life = (popup.life - DT).max(0.0);
            popup.pos.y -= 22.0 * DT;
        }
        let mut popup_spawned = false;
        for (cell, flash) in FieldCell::all().zip(&mut fx.brick_flash) {
            let i = cell.index();
            *flash = (*flash - DT).max(0.0);
            if events.brick && game.board().hp(cell) < fx.previous_bricks[i] {
                *flash = 0.18;
                if !popup_spawned && game.score() > fx.previous_score {
                    let r = cell_rect(cell);
                    fx.popups[fx.popup_cursor] = Popup {
                        pos: V2::new(r.x + r.w / 2.0, r.y),
                        life: 0.65,
                        value: (game.score() - fx.previous_score).saturating_sub(if events.clear {
                            game.summary().bonus
                        } else {
                            0
                        }),
                    };
                    fx.popup_cursor = (fx.popup_cursor + 1) % fx.popups.len();
                    popup_spawned = true;
                }
            }
        }
        fx.previous_bricks = hp_grid(game);
        fx.previous_score = game.score();
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
        self.frame(
            view.scale * screen_dpi_scale(),
            game,
            ui,
            profile,
            alpha,
            perf,
        );
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
        self.frame(view.scale, game, ui, profile, 1.0, None);
        // SAFETY: main thread, between draw calls; executes the batched frame.
        unsafe { get_internal_gl() }.flush();
        target.texture.get_texture_data().export_png(path);
        set_default_camera();
    }
    fn frame(
        &mut self,
        density: f32,
        game: &Game,
        ui: &Ui,
        profile: &Profile,
        alpha: f32,
        perf: Option<&Perf>,
    ) {
        let v = Scene {
            texture: self.texture.as_ref(),
            atlas: &self.kind.atlas,
            fonts: &self.kind.fonts,
            locale: self.kind.locale,
            density,
            device: ui.device,
            buffer: RefCell::new(std::mem::take(&mut self.text)),
            misfits: None,
        };
        // Painting the background into the scene batch, rather than with
        // `clear_background`, saves a full-framebuffer pass: Macroquad has
        // already cleared once this frame.
        v.cover(BG);
        scene(&v, &self.fx, game, ui, profile, alpha, perf);
        // Translucent shapes also blend into framebuffer alpha. Restore an
        // opaque frame so the compositor never shows anything through it.
        if let Some(opaque) = &self.opaque {
            gl_use_material(opaque);
            v.cover(WHITE);
            gl_use_default_material();
        }
        self.text = v.buffer.into_inner();
    }
}

fn scene(
    v: &Scene,
    fx: &Fx,
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
            attract(v, ui, profile);
        } else {
            sectors(v, ui, profile);
        }
        return;
    }
    playfield(v, fx);
    bricks(v, fx, game);
    effects(v, fx, game);
    paddle(v, fx, game);
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
    balls(v, fx, game, alpha);
    hud(v, game, profile);
    let mut letter = [0; 4];
    for drop in game.capsules() {
        if drop.active {
            let letter = capsule(drop.power).encode_utf8(&mut letter);
            let y = drop.pos.y + v.cap(Role::Label) / 2.0;
            v.put(letter, Role::Label, Slot::centered(drop.pos.x, 30.0, y), BG);
        }
    }
    for popup in &fx.popups {
        if popup.life > 0.0 {
            let color = opacity(INK, (popup.life * 3.0).min(1.0));
            v.format(TextId::Plus, &[Arg::Count(popup.value)], Form::Full, |t| {
                v.put(
                    t,
                    Role::Label,
                    Slot::centered(popup.pos.x, 120.0, popup.pos.y),
                    color,
                )
            });
        }
    }
    if !ui.paused && stage == Stage::Playing {
        if game.balls().iter().any(|b| b.active && b.held) {
            match v.device {
                Device::KeyboardMouse => v.say(
                    TextId::KeysRelease,
                    &[],
                    Role::Body,
                    Slot::line(720.0),
                    CYAN,
                ),
                Device::Gamepad => {
                    let items = [pad(Glyph::A, TextId::ActionRelease)];
                    v.pack(&items, Role::Body, |line, w| {
                        v.hint_line(line, w, 720.0, CYAN, Role::Body)
                    });
                }
            }
        }
        if game.effects().notice_ticks > 0
            && let Some(power) = game.effects().notice
        {
            let fade = (game.effects().notice_ticks as f32 / 60.0).min(1.0);
            let color = opacity(power_color(power), fade);
            v.say(
                TextId::PowerName(power),
                &[],
                Role::Body,
                Slot::line(687.0),
                color,
            );
        }
    }
    let footer_error = ui.save_error && (ui.paused || stage != Stage::Playing);
    if ui.paused {
        v.scrim();
        v.panel(240.0, 290.0, 480.0, 350.0);
        v.say(
            TextId::Paused,
            &[],
            Role::Display,
            Slot::centered(WIDTH / 2.0, PANEL, 342.0),
            INK,
        );
        menu(v, &ui::pause_menu(), ui.choice, None);
        let note = Slot::centered(WIDTH / 2.0, PANEL, 576.0);
        v.paragraph((TextId::RetryNote, &[]), Role::Body, note, 2, DIM);
        match v.device {
            Device::Gamepad => {
                let items = [
                    pad(Glyph::A, TextId::ActionSelect),
                    pad(Glyph::B, TextId::ActionResume),
                    pad(Glyph::X, TextId::ActionRetry),
                ];
                v.footer(&[&items], footer_error);
            }
            Device::KeyboardMouse => v.footer(&[&options(profile, v.device)], footer_error),
        }
    } else {
        match stage {
            Stage::Ready => ready(v, game),
            Stage::Cleared => cleared(v, game, summary),
            Stage::GameOver | Stage::Victory => {
                v.scrim();
                v.panel(240.0, 290.0, 480.0, 350.0);
                let heading = if stage == Stage::Victory {
                    TextId::JourneyComplete
                } else {
                    TextId::OneMoreOrbit
                };
                v.say(
                    heading,
                    &[],
                    Role::Display,
                    Slot::centered(WIDTH / 2.0, PANEL, 338.0),
                    INK,
                );
                v.pairs(
                    &[
                        (TextId::StatPoints, game.score()),
                        (TextId::StatMedals, profile.progress.medal_count()),
                    ],
                    368.0,
                    PANEL,
                );
                menu(
                    v,
                    &ui::result_menu(stage == Stage::Victory),
                    ui.choice,
                    None,
                );
                v.say(
                    TextId::ProgressSaved,
                    &[],
                    Role::Body,
                    Slot::centered(WIDTH / 2.0, PANEL, 590.0),
                    DIM,
                );
                v.footer(&[], footer_error);
            }
            Stage::Playing => {}
        }
    }
    if let Some(perf) = perf {
        v.panel(80.0, 430.0, 560.0, 210.0);
        v.say(
            TextId::PerfTitle,
            &[],
            Role::Body,
            Slot::left(100.0, 520.0, 462.0),
            CYAN,
        );
        // Diagnostic figures for developers, kept in English.
        for (i, line) in perf.lines.iter().enumerate() {
            v.draw(line, Role::Body, 100.0, 492.0 + i as f32 * 24.0, INK);
        }
    }
}

fn playfield(v: &Scene, fx: &Fx) {
    v.rect(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP, SURFACE);
    // Three walls; the open bottom edge is where a ball drains.
    let edge = mix(BORDER, CYAN, (fx.wall_flash * 4.0).min(0.6));
    v.rect(LEFT, TOP, RIGHT - LEFT, 1.0, edge);
    v.rect(LEFT, TOP, 1.0, BOTTOM - TOP, edge);
    v.rect(RIGHT - 1.0, TOP, 1.0, BOTTOM - TOP, edge);
}
fn bricks(v: &Scene, fx: &Fx, game: &Game) {
    // Quiet connections make the actual orthogonal blast routes readable.
    for cell in FieldCell::all() {
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
    for cell in FieldCell::all() {
        let i = cell.index();
        let hp = game.board().hp(cell);
        let r = cell_rect(cell);
        let c = if game.board().is_core(cell) {
            AMBER
        } else {
            sector_color(cell.row(), game.sector().sector().chapter)
        };
        let flash = fx.brick_flash[i] / 0.18;
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
fn effects(v: &Scene, fx: &Fx, game: &Game) {
    for cell in FieldCell::all() {
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
    if fx.pickup_flash > 0.0 {
        let progress = 1.0 - fx.pickup_flash / 0.65;
        v.ring(
            V2::new(game.paddle().x, PADDLE_Y),
            20.0 + progress * 90.0,
            2.0,
            opacity(CYAN, (1.0 - progress) * 0.6),
        );
    }
}
fn paddle(v: &Scene, fx: &Fx, game: &Game) {
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
    if fx.paddle_flash > 0.0 {
        v.rounded(
            x - 2.0,
            PADDLE_Y - 2.0,
            w + 4.0,
            18.0,
            9.0,
            opacity(CYAN, fx.paddle_flash * 2.0),
        );
    }
}
fn balls(v: &Scene, fx: &Fx, game: &Game, alpha: f32) {
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
        for n in (0..if ball.held { 0 } else { fx.trail_len[i] }).rev() {
            let index = (fx.cursor + 12 - 1 - n) % 12;
            let c = opacity(color, 0.22 * (1.0 - n as f32 / 12.0));
            v.circle(fx.trails[i][index], RADIUS * (1.0 - n as f32 / 15.0), c);
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
/// The HUD's two baselines: labels, then values a pair's gap under the
/// score's capitals. Score, sector name and lives all sit on the second.
const HUD_LABEL: f32 = 70.0;
const HUD_VALUE: f32 = HUD_LABEL + PAIR + cap_height(Role::Display);
/// Score left, sector centre, lives right. Labels are small tracked
/// capitals; figures are tabular, so the score never shifts as it grows.
fn hud(v: &Scene, game: &Game, profile: &Profile) {
    let side = WIDTH / 2.0 - 160.0 - LEFT;
    let label = |slot: Slot| Slot {
        y: HUD_LABEL,
        ..slot
    };
    let value = |slot: Slot| Slot {
        y: HUD_VALUE,
        ..slot
    };
    let (left, centre, right) = (
        Slot::left(LEFT, side, 0.0),
        Slot::centered(WIDTH / 2.0, 300.0, 0.0),
        Slot::right(RIGHT, side, 0.0),
    );
    v.say(TextId::Score, &[], Role::Label, label(left), DIM);
    let score = Figures::count(v.locale, game.score());
    v.put(score.as_str(), Role::Display, value(left), INK);

    let eyebrow = if game.mode() == Mode::Practice {
        TextId::PracticeNumber
    } else {
        TextId::SectorNumber
    };
    let sector = [Arg::Sector(game.sector())];
    v.say(eyebrow, &sector, Role::Label, label(centre), DIM);
    let name = TextId::SectorName(game.sector());
    v.say(name, &[], Role::Body, value(centre), INK);
    // The journey at a glance: here in the focus colour, cleared sectors
    // as supporting text, the rest muted.
    for id in SectorId::all() {
        v.rect(
            WIDTH / 2.0 - 94.0 + id.index() as f32 * 16.0,
            HUD_VALUE + S12,
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

    v.say(TextId::Lives, &[], Role::Label, label(right), DIM);
    // The dots rest on the value baseline, like the figures beside them.
    let shown = game.lives().max(3);
    for i in 0..shown {
        v.circle(
            V2::new(
                RIGHT - 5.0 - f32::from(shown - 1 - i) * 16.0,
                HUD_VALUE - 5.0,
            ),
            5.0,
            if i < game.lives() { INK } else { MUTED },
        );
    }
}
/// The eyebrow's baseline on the ready card; the rest stacks under it.
const READY_TOP: f32 = 540.0;
fn ready(v: &Scene, game: &Game) {
    let id = game.sector();
    let chapter = id.sector().chapter;
    let eyebrow = [Arg::Text(TextId::ChapterName(chapter)), Arg::Sector(id)];
    let hue = sector_color(0, chapter);
    let line = Slot::line(READY_TOP);
    v.say(TextId::ReadyEyebrow, &eyebrow, Role::Label, line, hue);
    let name = READY_TOP + S12 + cap_height(Role::Display);
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Display,
        Slot::line(name),
        INK,
    );
    // Display descenders and a tip's capsule chips both reach into the gap.
    let tip = name + S16 + cap_height(Role::Body);
    let slot = Slot::centered(WIDTH / 2.0, 720.0, tip);
    let lines = v.paragraph((TextId::SectorTip(id), &[]), Role::Body, slot, 2, DIM);
    let last = tip + lines.saturating_sub(1) as f32 * LINE;
    // The one inline hint: what to do next, in the interactive colour.
    let y = last + GROUP + cap_height(Role::Body);
    match v.device {
        Device::KeyboardMouse => v.say(TextId::KeysServe, &[], Role::Body, Slot::line(y), CYAN),
        Device::Gamepad => {
            let items = [pad(Glyph::A, TextId::ActionServe)];
            v.pack(&items, Role::Body, |line, w| {
                v.hint_line(line, w, y, CYAN, Role::Body)
            });
        }
    }
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
/// The sound, volume and display shortcuts. They are keyboard keys; a pad
/// player still sees the levels.
fn options(profile: &Profile, device: Device) -> [Item; 3] {
    let sound = if profile.settings.muted {
        TextId::SoundOff
    } else {
        TextId::SoundOn
    };
    let volume = Some(Arg::Count(u32::from(profile.settings.volume)));
    let key = |k| match device {
        Device::KeyboardMouse => Some(Cap::Key(k)),
        Device::Gamepad => None,
    };
    [
        Item {
            cap: key("M"),
            id: sound,
            arg: None,
        },
        Item {
            cap: key("[ ]"),
            id: TextId::Volume,
            arg: volume,
        },
        Item {
            cap: key("F"),
            id: TextId::Fullscreen,
            arg: None,
        },
    ]
}
/// The label of a menu action.
fn action_label(action: Action) -> TextId {
    match action {
        Action::Continue => TextId::ContinueJourney,
        Action::NewJourney => TextId::NewJourney,
        Action::Sectors => TextId::SectorSelect,
        Action::Resume => TextId::ActionResume,
        Action::Retry => TextId::RetrySector,
        Action::MainMenu => TextId::MainMenu,
    }
}
/// Draws `menu` with `focus` on one row. `detail` is the Continue row's
/// second line.
fn menu(v: &Scene, menu: &Menu, focus: usize, detail: Option<(TextId, &[Arg])>) {
    for (i, &action) in menu.actions.iter().enumerate() {
        let detail = detail.filter(|_| action == Action::Continue);
        v.button(
            menu.rect(i),
            action_label(action),
            &[],
            (i == 0, i == focus),
            detail,
        );
    }
}

/// The logo's top edge and cell size; the title stacks down from it.
const LOGO_TOP: f32 = 184.0;
const LOGO_CELL: f32 = 12.0;
fn attract(v: &Scene, ui: &Ui, profile: &Profile) {
    v.logo(270.0, LOGO_TOP, LOGO_CELL);
    let tagline = LOGO_TOP + 7.0 * LOGO_CELL + GROUP + cap_height(Role::Body);
    v.say(TextId::Tagline, &[], Role::Body, Slot::line(tagline), DIM);
    let saved = profile.progress.checkpoint();
    let title = ui::title_menu(saved.is_some());
    // Where the journey stands belongs on the button that resumes it.
    let detail = saved.map(|c| {
        [
            Arg::Sector(c.sector),
            Arg::Text(TextId::SectorName(c.sector)),
            Arg::Count(c.score),
        ]
    });
    let detail = detail
        .as_ref()
        .map(|args| (TextId::ContinueDetail, &args[..]));
    menu(v, &title, ui.choice, detail);

    // Progress: three label-value pairs spanning the menu's width. The
    // outer columns are wider: the best score is the longest figure.
    let label = title.bottom() + SECTION + cap_height(Role::Label);
    let value = label + PAIR + cap_height(Role::Body);
    let span = title.rect(0);
    let (outer, middle) = (span.w * 0.36, span.w * 0.28);
    for (i, name) in [TextId::StatSectors, TextId::StatMedals, TextId::StatBest]
        .into_iter()
        .enumerate()
    {
        let slot = |y| match i {
            0 => Slot::left(span.x, outer, y),
            1 => Slot::centered(WIDTH / 2.0, middle, y),
            _ => Slot::right(span.x + span.w, outer, y),
        };
        v.say(name, &[], Role::Label, slot(label), DIM);
        let total = |n: u32| [Arg::Count(n), Arg::Count(SECTOR_COUNT as u32 * n)];
        match i {
            0 => {
                let mut args = total(1);
                args[0] = Arg::Count(profile.progress.unlocked_count() as u32);
                v.say(TextId::Fraction, &args, Role::Body, slot(value), INK);
            }
            1 => {
                let mut args = total(3);
                args[0] = Arg::Count(profile.progress.medal_count());
                v.say(TextId::Fraction, &args, Role::Body, slot(value), INK);
            }
            _ => {
                let best = Figures::count(v.locale, profile.progress.best_score());
                v.put(best.as_str(), Role::Body, slot(value), INK);
            }
        }
    }
    let hints = match v.device {
        Device::KeyboardMouse => [
            hint(TextId::KeysMove),
            hint(TextId::KeysServe),
            hint(TextId::KeysPause),
        ],
        Device::Gamepad => [
            hint(TextId::PadMove),
            pad(Glyph::A, TextId::ActionServe),
            Item {
                cap: Some(Cap::Pad(Glyph::Start)),
                id: TextId::ActionPause,
                arg: None,
            },
        ],
    };
    let options = options(profile, v.device);
    v.footer(&[&hints, &options], ui.save_error);
}
fn sectors(v: &Scene, ui: &Ui, profile: &Profile) {
    let back = ui::back_rect();
    v.rounded(back.x, back.y, back.w, back.h, 8.0, RAISED);
    let baseline = back.y + back.h / 2.0 + v.cap(Role::Body) / 2.0;
    match v.device {
        Device::KeyboardMouse => {
            let cy = back.y + back.h / 2.0;
            v.line(
                V2::new(back.x + 20.0, cy - 5.0),
                V2::new(back.x + 15.0, cy),
                1.5,
                DIM,
            );
            v.line(
                V2::new(back.x + 15.0, cy),
                V2::new(back.x + 20.0, cy + 5.0),
                1.5,
                DIM,
            );
            let slot = Slot::left(back.x + 28.0, back.w - 38.0, baseline);
            v.say(TextId::ActionBack, &[], Role::Body, slot, DIM);
        }
        Device::Gamepad => {
            v.cap_glyph(Cap::Pad(Glyph::B), back.x + 10.0, baseline, Role::Body);
            let slot = Slot::left(back.x + 40.0, back.w - 50.0, baseline);
            v.say(TextId::ActionBack, &[], Role::Body, slot, DIM);
        }
    }
    v.say(
        TextId::SectorsHeading,
        &[],
        Role::Display,
        Slot::centered(WIDTH / 2.0, 480.0, 80.0),
        INK,
    );
    for chapter in Chapter::ALL {
        let r = ui::sector_rect(chapter.first_sector().index());
        let slot = Slot::left(r.x, r.w, r.y - S12);
        v.say(
            TextId::ChapterName(chapter),
            &[],
            Role::Label,
            slot,
            sector_color(0, chapter),
        );
    }
    for id in SectorId::all() {
        sector_card(v, id, id == ui.sector, profile);
    }
    sector_detail(v, ui.sector, profile);
    let hints = match v.device {
        Device::KeyboardMouse => [
            hint(TextId::KeysBrowse),
            hint(TextId::KeysPlay),
            hint(TextId::KeysBack),
        ],
        Device::Gamepad => [
            hint(TextId::PadBrowse),
            pad(Glyph::A, TextId::ActionPlay),
            pad(Glyph::B, TextId::ActionBack),
        ],
    };
    v.footer(&[&hints], ui.save_error);
}
/// A card in the sector grid: the name, the layout in miniature, and a pip
/// per medal; a padlock instead of pips while locked. Times live in the
/// detail panel, next to the medal they decide.
fn sector_card(v: &Scene, id: SectorId, selected: bool, profile: &Profile) {
    let (r, level) = (ui::sector_rect(id.index()), id.sector());
    let unlocked = profile.progress.is_unlocked(id);
    v.rounded(
        r.x,
        r.y,
        r.w,
        r.h,
        8.0,
        if selected { RAISED } else { SURFACE },
    );
    if selected {
        let edge = if unlocked { CYAN } else { MUTED };
        v.outline(r, 8.0, 1.5, edge);
    }
    let inner = r.x + S16;
    let name = r.y + S12 + cap_height(Role::Body);
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Body,
        Slot::left(inner, r.w - 2.0 * S16, name),
        if unlocked { INK } else { DIM },
    );
    // Bricks at their field proportions, on a 9 by 5 pitch, bottom-aligned
    // with the card's padding.
    let top = r.y + r.h - S12 - 33.0;
    for cell in FieldCell::all() {
        if level.layout.hp[cell.index()] > 0 {
            v.rect(
                inner + cell.col() as f32 * 9.0,
                top + cell.row() as f32 * 5.0,
                7.0,
                3.0,
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
    let cy = top + 16.5;
    let right = r.x + r.w - S16;
    if unlocked {
        let record = profile.progress.record(id);
        for (j, medal) in MEDAL_ORDER.into_iter().enumerate() {
            let earned = record.medals.contains(medal);
            let x = right - 4.0 - (2 - j) as f32 * 16.0;
            v.circle(V2::new(x, cy), 4.0, if earned { AMBER } else { MUTED });
        }
    } else {
        v.padlock(right - 8.0, cy, MUTED);
    }
}
/// The selected sector: where it sits, its name and tip, the medals and
/// how to earn them, and Play, or what unlocks it.
fn sector_detail(v: &Scene, id: SectorId, profile: &Profile) {
    let panel = ui::detail_rect();
    v.panel(panel.x, panel.y, panel.w, panel.h);
    let (level, record) = (id.sector(), profile.progress.record(id));
    let x = panel.x + PAD;
    let w = panel.w - 2.0 * PAD;
    let eyebrow = panel.y + PAD + cap_height(Role::Label);
    let args = [
        Arg::Text(TextId::ChapterName(level.chapter)),
        Arg::Sector(id),
    ];
    let hue = sector_color(0, level.chapter);
    v.say(
        TextId::ReadyEyebrow,
        &args,
        Role::Label,
        Slot::left(x, w, eyebrow),
        hue,
    );
    let name = eyebrow + PAIR + cap_height(Role::Body);
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Body,
        Slot::left(x, w, name),
        INK,
    );
    // A tip may open with a capsule chip, which stands taller than capitals.
    let tip = name + S12 + cap_height(Role::Body);
    let tip_slot = Slot::left(x, w, tip);
    v.paragraph((TextId::SectorTip(id), &[]), Role::Body, tip_slot, 2, DIM);

    // The medals as a checklist: chip, then what earns it. Rows start a
    // group below the tip's second line, whether or not it wraps.
    let table = w - ui::DETAIL_ACTION - S32;
    let medals = [
        (TextId::MedalClear, TextId::MedalClearHow),
        (TextId::MedalClean, TextId::MedalCleanHow),
        (TextId::MedalSwift, TextId::SwiftWithin),
    ];
    let chip = medals
        .iter()
        .map(|&(medal, _)| v.width_of(medal, &[], Role::Label) + 2.0 * S12)
        .fold(0.0, f32::max);
    let text = Slot::left(x + chip + S16, table - chip - S16, 0.0);
    let par = [Arg::Clock(level.par_seconds)];
    let rows = tip + LINE + GROUP;
    for (i, (medal, how)) in medals.into_iter().enumerate() {
        let earned = record.medals.contains(MEDAL_ORDER[i]);
        let top = rows + i as f32 * S32;
        let tone = if earned { AMBER } else { MUTED };
        v.rounded(x, top, chip, S24, S12, opacity(tone, 0.16));
        let cy = top + S12;
        let label = Slot::centered(x + chip / 2.0, chip - S8, cy + v.cap(Role::Label) / 2.0);
        v.say(medal, &[], Role::Label, label, tone);
        let args: &[Arg] = if how == TextId::SwiftWithin {
            &par
        } else {
            &[]
        };
        let baseline = cy + v.cap(Role::Body) / 2.0;
        v.say(
            how,
            args,
            Role::Body,
            Slot {
                y: baseline,
                ..text
            },
            DIM,
        );
    }
    // The best time sits under the target it is measured against.
    if record.best_ticks > 0 {
        let cy = rows + 3.0 * S32 + S12;
        let label = Slot::centered(x + chip / 2.0, chip, cy + v.cap(Role::Label) / 2.0);
        v.say(TextId::StatBest, &[], Role::Label, label, DIM);
        let t = Figures::of(|f| write!(f, "{}", Clock(record.best_ticks)));
        let baseline = cy + v.cap(Role::Body) / 2.0;
        v.put(
            t.as_str(),
            Role::Body,
            Slot {
                y: baseline,
                ..text
            },
            INK,
        );
    }

    let play = ui::play_rect();
    let column = |y| Slot::centered(play.x + play.w / 2.0, play.w, y);
    if profile.progress.is_unlocked(id) {
        v.button(
            play,
            TextId::PlaySector,
            &[Arg::Sector(id)],
            (true, true),
            None,
        );
        // The footnote qualifies Play, so it sits right under it.
        let note = play.y + play.h + PAIR + cap_height(Role::Caption);
        v.paragraph(
            (TextId::PracticeNote, &[]),
            Role::Caption,
            column(note),
            2,
            DIM,
        );
    } else {
        let before = [Arg::Sector(SectorId::clamped(id.index().saturating_sub(1)))];
        v.padlock(play.x + play.w / 2.0, play.y + 4.0, MUTED);
        let hint = play.y + S24 + cap_height(Role::Body);
        v.paragraph(
            (TextId::UnlockHint, &before),
            Role::Body,
            column(hint),
            2,
            DIM,
        );
    }
}
/// Distance between the sector-clear columns, and the widest a medal chip
/// grows: three chips at most 136 wide leave at least 14 between them.
const CLEAR_COLUMN: f32 = 150.0;
const CHIP_MAX: f32 = 136.0;
fn cleared(v: &Scene, game: &Game, summary: SectorSummary) {
    v.scrim();
    v.panel(240.0, 290.0, 480.0, 350.0);
    v.say(
        TextId::SectorClear,
        &[],
        Role::Display,
        Slot::centered(WIDTH / 2.0, PANEL, 342.0),
        INK,
    );
    // Without the extra-life line the stats and medals drop into its space,
    // so the button never sits under an empty gap.
    let shift = if summary.life_earned { 0.0 } else { 14.0 };
    for (i, label) in [TextId::StatTime, TextId::StatBonus, TextId::StatBestChain]
        .into_iter()
        .enumerate()
    {
        let x = WIDTH / 2.0 + (i as f32 - 1.0) * CLEAR_COLUMN;
        v.say(
            label,
            &[],
            Role::Label,
            Slot::centered(x, CHIP_MAX, 386.0 + shift),
            DIM,
        );
        let value = Slot::centered(x, CHIP_MAX, 414.0 + shift);
        match i {
            0 => {
                let t = Figures::of(|f| write!(f, "{}", Clock(summary.ticks)));
                v.put(t.as_str(), Role::Body, value, INK);
            }
            1 => v.say(
                TextId::Plus,
                &[Arg::Count(summary.bonus)],
                Role::Body,
                value,
                INK,
            ),
            _ => {
                let combo = Figures::count(v.locale, summary.best_combo);
                v.put(combo.as_str(), Role::Body, value, INK);
            }
        }
    }
    for (i, label) in [TextId::MedalClear, TextId::MedalClean, TextId::MedalSwift]
        .into_iter()
        .enumerate()
    {
        let x = WIDTH / 2.0 + (i as f32 - 1.0) * CLEAR_COLUMN;
        let earned = summary.medals.contains(MEDAL_ORDER[i]);
        let w = (v.width_of(label, &[], Role::Label) + 32.0).min(CHIP_MAX);
        v.rounded(
            x - w / 2.0,
            436.0 + shift,
            w,
            28.0,
            14.0,
            opacity(if earned { AMBER } else { MUTED }, 0.16),
        );
        let y = 450.0 + shift + v.cap(Role::Label) / 2.0;
        v.say(
            label,
            &[],
            Role::Label,
            Slot::centered(x, CHIP_MAX - 12.0, y),
            if earned { AMBER } else { MUTED },
        );
    }
    if summary.life_earned {
        v.say(
            TextId::ExtraLife,
            &[],
            Role::Body,
            Slot::centered(WIDTH / 2.0, PANEL, 494.0),
            CYAN,
        );
    }
    let next = if game.mode() == Mode::Practice {
        TextId::BackToSectors
    } else {
        TextId::NextSector
    };
    v.button(ui::next_rect(), next, &[], (true, true), None);
    match v.device {
        Device::KeyboardMouse => v.say(
            TextId::KeysContinue,
            &[],
            Role::Caption,
            Slot::centered(WIDTH / 2.0, PANEL, 602.0),
            MUTED,
        ),
        Device::Gamepad => {
            let items = [pad(Glyph::A, TextId::ActionContinue)];
            v.pack(&items, Role::Caption, |line, w| {
                v.hint_line(line, w, 602.0, MUTED, Role::Caption)
            });
        }
    }
}

/// A short run of figures formatted on the stack, so drawing scores and
/// times allocates nothing.
struct Figures {
    bytes: [u8; 48],
    len: usize,
}
impl Figures {
    fn of(write: impl FnOnce(&mut Self) -> std::fmt::Result) -> Self {
        let mut out = Self {
            bytes: [0; 48],
            len: 0,
        };
        // Overflow only truncates; 48 bytes hold any u32 in any locale.
        let _ = write(&mut out);
        out
    }
    fn count(locale: Locale, n: u32) -> Self {
        Self::of(|f| ark_text::grouped(f, locale, n))
    }
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes[..self.len]).unwrap_or_default()
    }
}
impl Write for Figures {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let end = self.len + s.len();
        let room = self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?;
        room.copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}

/// A duration in ticks as `mm:ss`; figures read the same in every locale.
struct Clock(u32);
impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = self.0 / TICK_HZ;
        write!(f, "{:02}:{:02}", seconds / 60, seconds % 60)
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
    for cell in FieldCell::all() {
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
mod tests;

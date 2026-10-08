use crate::{
    atlas::{Atlas, Cell},
    input::Device,
    perf::Perf,
    pixel_font,
    storage::Profile,
    ui::{self, Action, Hits, Menu, Pressed, Prompt, Screen, Ui},
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
use ark_glyphs::{Fonts, ICON_EM, spec, spec::Weight};
use ark_text::{Arg, Form, Locale, Role, TextId, capsule, icon_power};
use macroquad::models::Vertex;
use macroquad::prelude::*;
use std::{
    cell::{Cell as Shared, RefCell},
    f32::consts::{FRAC_PI_2, TAU},
    fmt::Write,
};

/// The fixed scene, in scene units; the window shows it scaled and centered.
pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 900.0;

/// The night around the instrument; everything outside the arch.
const NIGHT: Color = hex(0x07080e);
const PEARL_RIM: Color = hex(0xcdd8e8);
const SURFACE: Color = Color::new(0.050, 0.060, 0.092, 1.0);
const BORDER: Color = Color::new(0.135, 0.155, 0.210, 1.0);
const INK: Color = hex(0xedf2fa);
const DIM: Color = hex(0x8794ab);
/// Only for what is unavailable.
const MUTED: Color = hex(0x4a5469);
const CYAN: Color = hex(0x54def5);
/// Achievements only.
const AMBER: Color = hex(0xffc24d);
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

/// The spacing scale, in scene units (4, 8, 12, 16, 24, 32, 48, 64).
/// Layouts step by these and nothing in between, so related things always
/// sit visibly closer than unrelated ones: 8 between action rows, 12 from
/// a chip to its text, 16 band padding and actions to their help line, 24
/// between groups and from a title to its content, 32 inside a sheet.
const S8: f32 = 8.0;
const S12: f32 = 12.0;
const S16: f32 = 16.0;
const S24: f32 = 24.0;
const S32: f32 = 32.0;

/// How a piece of text is set: its role, whether it is emphasised (body
/// text on a primary or focused action), and its size in scene units,
/// the role's own unless a place sets it otherwise.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Style {
    role: Role,
    strong: bool,
    size: f32,
    /// Letter spacing in em; the role's own unless set otherwise.
    tracking: f32,
}
impl From<Role> for Style {
    fn from(role: Role) -> Self {
        Self {
            role,
            strong: false,
            size: spec::style(role).0,
            tracking: spec::style(role).2,
        }
    }
}
impl Style {
    fn strong(self) -> Self {
        Self {
            strong: true,
            ..self
        }
    }
    fn sized(self, size: f32) -> Self {
        Self { size, ..self }
    }
    /// Set solid, as a key's label is.
    fn untracked(self) -> Self {
        Self {
            tracking: 0.0,
            ..self
        }
    }
    fn weight(self) -> Weight {
        if self.strong {
            spec::strong(self.role)
        } else {
            spec::style(self.role).1
        }
    }
}

/// Side margin for full-width text, and the widest a centred line may be.
const MARGIN: f32 = 64.0;
const FULL: f32 = WIDTH - 2.0 * MARGIN;
/// Baseline-to-baseline for wrapped lines of `style`.
fn leading(style: impl Into<Style>) -> f32 {
    let style = style.into();
    style.size * spec::line(style.role)
}
/// The footer's last baseline; rows stack upward from it.
const FOOTER: f32 = 872.0;

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
        a.a + (b.a - a.a) * t,
    )
}
/// A colour from its `0xRRGGBB` code, as the design tokens are written.
const fn hex(rgb: u32) -> Color {
    Color::new(
        ((rgb >> 16) & 0xFF) as f32 / 255.0,
        ((rgb >> 8) & 0xFF) as f32 / 255.0,
        (rgb & 0xFF) as f32 / 255.0,
        1.0,
    )
}

/// A vertical colour ramp: `top` at the top edge to `bottom` at the
/// bottom, through `mid` at a fraction of the height when there is one.
#[derive(Clone, Copy)]
struct Fill {
    top: Color,
    mid: Option<(f32, Color)>,
    bottom: Color,
}
impl Fill {
    const fn flat(c: Color) -> Self {
        Self::ramp(c, c)
    }
    const fn ramp(top: Color, bottom: Color) -> Self {
        Self {
            top,
            mid: None,
            bottom,
        }
    }
    /// A lit edge: light at the top, the colour itself at `at`, dark below.
    const fn lit(top: Color, at: f32, mid: Color, bottom: Color) -> Self {
        Self {
            top,
            mid: Some((at, mid)),
            bottom,
        }
    }
    fn at(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        match self.mid {
            Some((m, c)) if t < m => mix(self.top, c, t / m),
            Some((m, c)) => mix(c, self.bottom, (t - m) / (1.0 - m)),
            None => mix(self.top, self.bottom, t),
        }
    }
}

/// The edge of a rectangle with rounded corners, clockwise from the
/// top-left corner. Every path has the same number of points, so two of
/// them pair up into an outline or a halo.
#[derive(Clone, Copy)]
struct Path {
    rect: Rect,
    /// Top-left, top-right, bottom-right, bottom-left.
    radii: [f32; 4],
}
impl Path {
    const STEPS: usize = 6;
    const LEN: usize = 4 * (Self::STEPS + 1);
    fn new(rect: Rect, radii: [f32; 4]) -> Self {
        let most = (rect.w.min(rect.h) / 2.0).max(0.0);
        Self {
            rect,
            radii: radii.map(|r| r.clamp(0.0, most)),
        }
    }
    fn points(&self) -> impl Iterator<Item = Vec2> {
        let Rect { x, y, w, h } = self.rect;
        let [a, b, c, d] = self.radii;
        [
            (x + a, y + a, a, 2.0),
            (x + w - b, y + b, b, 3.0),
            (x + w - c, y + h - c, c, 0.0),
            (x + d, y + h - d, d, 1.0),
        ]
        .into_iter()
        .flat_map(|(cx, cy, r, start)| {
            (0..=Self::STEPS).map(move |i| {
                let a = (start + i as f32 / Self::STEPS as f32) * FRAC_PI_2;
                vec2(cx, cy) + Vec2::from_angle(a) * r
            })
        })
    }
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
    /// The mouse, not the keyboard, has been driving the paddle.
    mouse: bool,
    /// Glyphs whose input just fired.
    pressed: Pressed,
    buffer: RefCell<String>,
    misfits: Option<&'a RefCell<Vec<Misfit>>>,
    /// What this frame drew that the pointer can hit.
    hits: RefCell<Hits>,
    /// Opacity and downward offset for what is being drawn: a sheet fades
    /// and rises into place. Alpha and translation only.
    motion: Shared<(f32, f32)>,
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
        let (alpha, lift) = self.motion.get();
        let color = Color {
            a: color.a * alpha,
            ..color
        };
        Vertex::new(p.x, p.y + lift, 0.0, uv.x, uv.y, color)
    }
    fn set_motion(&self, alpha: f32, lift: f32) {
        self.motion.set((alpha, lift));
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
        let (alpha, lift) = self.motion.get();
        let color = Color {
            a: color.a * alpha,
            ..color
        };
        let vertices = corners.map(|(p, uv)| Vertex::new(p.x, p.y + lift, 0.0, uv.x, uv.y, color));
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

    // Pixel snapping. Interface shapes land on whole physical pixels, and
    // nothing thin ever rounds away.

    /// `v` scene units, moved to the nearest physical pixel.
    fn snap(&self, v: f32) -> f32 {
        (v * self.density).round() / self.density
    }
    /// A thickness of `units`, in whole physical pixels and at least one.
    fn thick(&self, units: f32) -> f32 {
        (units * self.density).round().max(1.0) / self.density
    }
    fn snap_rect(&self, r: Rect) -> Rect {
        let (x, y) = (self.snap(r.x), self.snap(r.y));
        Rect::new(x, y, self.snap(r.x + r.w) - x, self.snap(r.y + r.h) - y)
    }

    /// A vertical colour ramp across a quad.
    fn ramp(&self, r: Rect, top: Color, bottom: Color) {
        let vertices = [
            self.vertex(vec2(r.x, r.y), top),
            self.vertex(vec2(r.x + r.w, r.y), top),
            self.vertex(vec2(r.x + r.w, r.y + r.h), bottom),
            self.vertex(vec2(r.x, r.y + r.h), bottom),
        ];
        self.mesh(&vertices, &[0, 1, 2, 0, 2, 3]);
    }
    /// `r` with each corner rounded by its own radius (top-left, top-right,
    /// bottom-right, bottom-left), filled with `fill`. One fan from the
    /// centre: a rounded rectangle is convex, so no triangle overlaps
    /// another and translucent fills stay even.
    fn shape(&self, r: Rect, radii: [f32; 4], fill: Fill) {
        let path = Path::new(r, radii);
        let colour = |p: Vec2| fill.at((p.y - r.y) / r.h);
        let centre = r.center();
        let zero = self.vertex(centre, colour(centre));
        let mut vertices = [zero; 1 + Path::LEN];
        let mut indices = [0_u16; 3 * Path::LEN];
        for (i, p) in path.points().enumerate() {
            vertices[i + 1] = self.vertex(p, colour(p));
            let next = (i + 1) % Path::LEN + 1;
            indices[i * 3..i * 3 + 3].copy_from_slice(&[0, i as u16 + 1, next as u16]);
        }
        self.mesh(&vertices, &indices);
    }
    /// The area between two rounded paths of the same corners, with a
    /// colour for each: an outline when both are opaque, a soft halo when
    /// the outer one is transparent.
    fn between(&self, outer: Path, inner: Path, outer_fill: Fill, inner_fill: Fill) {
        let at = |path: &Path, fill: Fill, p: Vec2| fill.at((p.y - path.rect.y) / path.rect.h);
        let zero = self.vertex(Vec2::ZERO, outer_fill.top);
        let mut vertices = [zero; 2 * Path::LEN];
        let mut indices = [0_u16; 6 * Path::LEN];
        for (i, (a, b)) in outer.points().zip(inner.points()).enumerate() {
            vertices[i * 2] = self.vertex(a, at(&outer, outer_fill, a));
            vertices[i * 2 + 1] = self.vertex(b, at(&inner, inner_fill, b));
            let (k, next) = ((i * 2) as u16, ((i + 1) % Path::LEN * 2) as u16);
            indices[i * 6..i * 6 + 6].copy_from_slice(&[k, k + 1, next, next, k + 1, next + 1]);
        }
        self.mesh(&vertices, &indices);
    }
    /// The edge of `bounds` rounded by `r`, `t` thick, drawn inside it.
    fn outline(&self, bounds: Rect, r: f32, t: f32, color: Color) {
        let inner = Rect::new(
            bounds.x + t,
            bounds.y + t,
            bounds.w - 2.0 * t,
            bounds.h - 2.0 * t,
        );
        self.between(
            Path::new(bounds, [r; 4]),
            Path::new(inner, [(r - t).max(0.0); 4]),
            Fill::flat(color),
            Fill::flat(color),
        );
    }
    /// A soft light or shadow around `bounds`: `color` at its edge, fading
    /// to nothing `spread` further out. Vertex colours, so no blur pass.
    fn halo(&self, bounds: Rect, r: f32, spread: f32, color: Color) {
        let outer = Rect::new(
            bounds.x - spread,
            bounds.y - spread,
            bounds.w + 2.0 * spread,
            bounds.h + 2.0 * spread,
        );
        self.between(
            Path::new(outer, [r + spread; 4]),
            Path::new(bounds, [r; 4]),
            Fill::flat(opacity(color, 0.0)),
            Fill::flat(color),
        );
    }
    /// A pearl lit from above: white at a hub above its centre, cooler at
    /// its rim. The lives in the band are pearls.
    fn pearl(&self, p: V2, r: f32, alpha: f32) {
        const SIDES: usize = 20;
        let hub = vec2(p.x, p.y - 0.4 * r);
        let mut vertices = [self.vertex(hub, opacity(WHITE, alpha)); SIDES + 2];
        let mut indices = [0_u16; SIDES * 3];
        let rim = opacity(PEARL_RIM, alpha);
        for i in 0..=SIDES {
            let a = i as f32 / SIDES as f32 * TAU;
            vertices[i + 1] = self.vertex(vec2(p.x, p.y) + vec2(a.cos(), a.sin()) * r, rim);
            if i < SIDES {
                indices[i * 3..i * 3 + 3].copy_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
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
    fn face(&self, style: Style, text: &str) -> Face {
        let role = style.role;
        match spec::ppem_px(role, style.size * self.density) {
            Some(ppem) => Face::Noto(ppem),
            None if text.chars().all(pixel_font::has) => {
                Face::Pixel(if matches!(role, Role::Display | Role::Title) {
                    2.0
                } else {
                    1.0
                })
            }
            None => Face::Noto(spec::rungs(role).min().unwrap_or(spec::LADDER[0])),
        }
    }
    /// The advance width of `text`, in scene units.
    fn measure(&self, text: &str, style: impl Into<Style>) -> f32 {
        let style = style.into();
        match self.face(style, text) {
            Face::Noto(ppem) => {
                let tracking = self.fonts.tracking(style.tracking, ppem);
                self.fonts.measure(text, style.weight(), ppem, tracking) / self.density
            }
            Face::Pixel(cell) => {
                let n = text.chars().count() as f32;
                (n * 6.0 - 1.0).max(0.0) * cell / self.density
            }
        }
    }
    /// The height of capitals, in scene units, for centring a line.
    fn cap(&self, style: impl Into<Style>) -> f32 {
        let style = style.into();
        match (self.face(style, "A"), self.fonts.latin.face(style.weight())) {
            (Face::Noto(ppem), Some(f)) => {
                f32::from(f.cap_height) * f32::from(ppem) / f32::from(f.units_per_em) / self.density
            }
            _ => style.size * 0.7,
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
    fn draw(&self, text: &str, style: impl Into<Style>, x: f32, y: f32, color: Color) {
        let style = style.into();
        let d = self.density;
        // Whole physical pixels: the view's offset is snapped too, so every
        // glyph lands exactly on the pixel grid.
        let (ox, oy) = ((x * d).round(), (y * d).round());
        match self.face(style, text) {
            Face::Noto(ppem) => {
                let weight = style.weight();
                let tracking = self.fonts.tracking(style.tracking, ppem);
                self.fonts.layout(text, weight, ppem, tracking, |p| {
                    if let Some(power) = icon_power(p.c) {
                        let x = (ox + p.x.round()) / d;
                        self.chip(power, x, y, f32::from(ppem) / d, style);
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
    fn chip(&self, power: Power, x: f32, baseline: f32, size: f32, beside: Style) {
        let (w, cy) = (ICON_EM * size, baseline - self.cap(beside) / 2.0);
        self.rounded(x, cy - size / 2.0, w, size, size / 2.0, power_color(power));
        let mut letter = [0; 4];
        let letter = capsule(power).encode_utf8(&mut letter);
        let y = cy + self.cap(Role::Label) / 2.0;
        self.put(
            letter,
            Role::Label,
            Slot::centered(x + w / 2.0, w, y),
            NIGHT,
        );
    }
    /// Draws `text` aligned in `slot`; text wider than the slot is drawn
    /// anyway and reported to the layout tests.
    fn put(&self, text: &str, style: impl Into<Style>, slot: Slot, color: Color) -> f32 {
        let style = style.into();
        let width = self.measure(text, style);
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
        self.draw(text, style, x, slot.y, color);
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
    fn say(&self, id: TextId, args: &[Arg], style: impl Into<Style>, slot: Slot, color: Color) {
        let style = style.into();
        let fits = self.format(id, args, Form::Full, |t| self.measure(t, style) <= slot.w);
        let form = if fits { Form::Full } else { Form::Short };
        self.format(id, args, form, |t| self.put(t, style, slot, color));
    }
    fn width_of(&self, id: TextId, args: &[Arg], style: impl Into<Style>) -> f32 {
        let style = style.into();
        self.format(id, args, Form::Full, |t| self.measure(t, style))
    }
    /// How many lines `paragraph` would set `id` in, `width` wide.
    fn lines(&self, (id, args): (TextId, &[Arg]), style: impl Into<Style>, width: f32) -> usize {
        let style = style.into();
        self.format(id, args, Form::Full, |text| match self.face(style, text) {
            Face::Noto(ppem) => {
                let tracking = self.fonts.tracking(style.tracking, ppem);
                let room = width * self.density;
                self.fonts
                    .wrap(text, style.weight(), ppem, tracking, room, |_| {})
            }
            Face::Pixel(_) => pixel_lines(text, width, |t| self.measure(t, style), |_| {}),
        })
    }
    /// Sets `id` across up to `max` lines from baseline `slot.y`, wrapping
    /// at word (or, in Chinese and Japanese, character) boundaries.
    /// Returns the lines used; more than `max` is reported.
    fn paragraph(
        &self,
        (id, args): (TextId, &[Arg]),
        style: impl Into<Style>,
        slot: Slot,
        max: usize,
        color: Color,
    ) -> usize {
        let style = style.into();
        self.format(id, args, Form::Full, |text| {
            let mut lines = 0;
            let line = |line: &str| {
                let y = slot.y + lines as f32 * leading(style);
                if lines < max {
                    self.put(line, style, Slot { y, ..slot }, color);
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
            match self.face(style, text) {
                Face::Noto(ppem) => {
                    let tracking = self.fonts.tracking(style.tracking, ppem);
                    let room = slot.w * self.density;
                    self.fonts
                        .wrap(text, style.weight(), ppem, tracking, room, line);
                }
                Face::Pixel(_) => {
                    pixel_lines(text, slot.w, |t| self.measure(t, style), line);
                }
            }
            lines.min(max)
        })
    }

    /// The save warning, until the band reports it.
    fn footer(&self, save_error: bool) {
        if save_error {
            self.say(
                TextId::SaveFailed,
                &[],
                Role::Caption,
                Slot::line(FOOTER),
                INK,
            );
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

/// Wraps pixel-font text into lines no wider than `room`. The pixel font
/// spells only Latin, so spaces are the only breaks. Returns the count.
fn pixel_lines<'t>(
    text: &'t str,
    room: f32,
    measure: impl Fn(&str) -> f32,
    mut line: impl FnMut(&'t str),
) -> usize {
    let mut count = 0;
    let mut rest = text.trim();
    while !rest.is_empty() {
        let fits = |end: usize| measure(&rest[..end]) <= room;
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
        count += 1;
        rest = rest[end..].trim_start();
    }
    count
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
    /// Lives when this sector began; the band rings each one lost since.
    entry_lives: u8,
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
            entry_lives: 0,
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
    hits: Hits,
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
            hits: Hits::default(),
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

    /// What the last frame drew that the pointer can hit.
    pub fn hits(&self) -> &Hits {
        &self.hits
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
            fx.entry_lives = game.lives();
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
        let hits = self.hits;
        self.frame(view.scale, game, ui, profile, 1.0, None);
        // An offscreen capture is not what the player sees.
        self.hits = hits;
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
            mouse: ui.mouse,
            pressed: ui.pressed,
            buffer: RefCell::new(std::mem::take(&mut self.text)),
            misfits: None,
            hits: RefCell::default(),
            motion: Shared::new((1.0, 0.0)),
        };
        // Painting the background into the scene batch, rather than with
        // `clear_background`, saves a full-framebuffer pass: Macroquad has
        // already cleared once this frame.
        v.cover(NIGHT);
        scene(&v, &self.fx, game, ui, profile, alpha, perf);
        // Translucent shapes also blend into framebuffer alpha. Restore an
        // opaque frame so the compositor never shows anything through it.
        if let Some(opaque) = &self.opaque {
            gl_use_material(opaque);
            v.cover(WHITE);
            gl_use_default_material();
        }
        self.hits = v.hits.into_inner();
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
    frame::arch(v, fx.wall_flash);
    if ui.screen != Screen::Play {
        if ui.screen == Screen::Title {
            screens::title(v, ui, profile);
        } else {
            screens::sectors(v, ui, profile);
        }
        if let Some(row) = ui.settings {
            sheet::dim(v, ui.sheet_open);
            sheet::settings(v, ui, profile, row);
        }
        return;
    }
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
    frame::band_play(v, fx, game, profile);
    let mut letter = [0; 4];
    for drop in game.capsules() {
        if drop.active {
            let letter = capsule(drop.power).encode_utf8(&mut letter);
            let y = drop.pos.y + v.cap(Role::Label) / 2.0;
            v.put(
                letter,
                Role::Label,
                Slot::centered(drop.pos.x, 30.0, y),
                NIGHT,
            );
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
            let words = (TextId::ActionRelease, Role::Body.into());
            let w = chips::prompt_width(v, Prompt::Serve, words, sheet::CHIP);
            let at = (v.snap(WIDTH / 2.0 - w / 2.0), 720.0);
            chips::prompt(v, Prompt::Serve, words, at, sheet::CHIP, CYAN);
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
    if let Some(row) = ui.settings {
        sheet::dim(v, ui.sheet_open);
        sheet::settings(v, ui, profile, row);
    } else if ui.paused {
        sheet::dim(v, ui.sheet_open);
        sheet::pause(v, ui, game);
        v.footer(footer_error);
    } else {
        match stage {
            Stage::Ready => screens::ready(v, game),
            Stage::Cleared => {
                sheet::dim(v, ui.sheet_open);
                sheet::cleared(v, ui, game, summary);
            }
            Stage::GameOver | Stage::Victory => {
                sheet::dim(v, ui.sheet_open);
                sheet::results(v, ui, game, stage == Stage::Victory);
                v.footer(footer_error);
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

mod chips;
mod frame;
mod screens;
mod sheet;
#[cfg(test)]
mod tests;

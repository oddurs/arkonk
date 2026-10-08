//! Where the fixed scene sits on the screen, and which of the three layout
//! classes that screen gets.
use super::{HEIGHT, WIDTH};
use ark::{
    field::{BOTTOM, LEFT, RIGHT, TOP},
    geom::V2,
};
use macroquad::prelude::*;
use std::sync::OnceLock;

/// How much interface a screen holds, by the frame's width in physical
/// pixels: the class is chosen from the frame, never from the window or
/// the platform, so a small window and a handheld behave alike.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// 720 px and wider: everything as designed.
    Regular,
    /// 400 to 719 px: a shorter band, narrow rails, wide sheets, one
    /// chapter of sectors at a time.
    Small,
    /// Under 400 px: the 5×7 pixel font, a one-line band, and lists.
    Compact,
}
impl Class {
    /// The class for a frame `width` physical pixels wide.
    pub fn of(width: f32) -> Self {
        if width >= 720.0 {
            Class::Regular
        } else if width >= 400.0 {
            Class::Small
        } else {
            Class::Compact
        }
    }
}

/// The compact band's strip and the gap under it, in physical pixels.
pub const STRIP: f32 = 8.0;
const STRIP_GAP: f32 = 1.0;

/// Where the fixed scene sits in the window: uniform scale, centred.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    /// Window points per scene unit, and the scene's origin in points.
    pub scale: f32,
    pub x: f32,
    pub y: f32,
    /// Physical pixels per scene unit: what text and hairlines snap to.
    pub density: f32,
    pub class: Class,
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
        let class = Class::of(WIDTH * scale * dpi);
        // An offset on whole physical pixels keeps glyphs on the pixel grid.
        let snap = |value: f32| (value * dpi).round() / dpi;
        if class == Class::Compact {
            // A tiny screen gives the field all it has: the scene is
            // fitted to the field, its one-pixel rails and the band's strip,
            // and everything else is night.
            let (rails, strip) = (2.0 / dpi, (STRIP + STRIP_GAP) / dpi);
            let field = (RIGHT - LEFT, BOTTOM - TOP);
            let scale = ((width - rails) / field.0).min((height - strip) / field.1);
            let top = (height - strip - field.1 * scale) / 2.0 + strip;
            return Some(Self {
                scale,
                x: snap((width - field.0 * scale) / 2.0 - LEFT * scale),
                y: snap(top - TOP * scale),
                density: scale * dpi,
                class,
            });
        }
        Some(Self {
            scale,
            x: snap((width - WIDTH * scale) / 2.0),
            y: snap((height - HEIGHT * scale) / 2.0),
            density: scale * dpi,
            class,
        })
    }
    /// The view of the window, or of the frame preview inside it.
    pub fn current() -> Option<Self> {
        let (w, h, dpi) = (screen_width(), screen_height(), screen_dpi_scale());
        let Some(&(pw, ph)) = PREVIEW.get() else {
            return Self::fit(w, h, dpi);
        };
        // Laid out for a pw × ph screen at one pixel per pixel, then shown
        // a whole number of times larger, so every pixel stays square.
        let inner = Self::fit(pw, ph, 1.0)?;
        let (ww, wh) = (w * dpi, h * dpi);
        let k = (ww / pw).min(wh / ph).floor().max(1.0);
        let (ox, oy) = (((ww - pw * k) / 2.0).floor(), ((wh - ph * k) / 2.0).floor());
        Some(Self {
            scale: inner.scale * k / dpi,
            x: (ox + inner.x * k) / dpi,
            y: (oy + inner.y * k) / dpi,
            density: inner.density,
            class: inner.class,
        })
    }
    pub fn to_scene(self, x: f32, y: f32) -> V2 {
        V2::new((x - self.x) / self.scale, (y - self.y) / self.scale)
    }
    /// Maps the whole window onto scene units, centering the fixed scene.
    pub(super) fn camera(&self, width: f32, height: f32) -> Camera2D {
        let w = width / self.scale;
        let h = height / self.scale;
        Camera2D {
            target: vec2(w / 2.0 - self.x / self.scale, h / 2.0 - self.y / self.scale),
            zoom: vec2(2.0 / w, 2.0 / h),
            ..Default::default()
        }
    }
}

/// A development aid: lay out and draw as if the screen were this many
/// physical pixels, shown at a whole multiple in the window, so Small and
/// Compact layouts can be seen on a desktop. Set once, at launch.
static PREVIEW: OnceLock<(f32, f32)> = OnceLock::new();

/// Turns on the frame preview for a `width` × `height` screen.
pub fn preview(width: u32, height: u32) {
    PREVIEW.get_or_init(|| (width as f32, height as f32));
}

/// The pointer in scene units; `None` when the window cannot map it, so a
/// minimized window can never feed a non-finite position to the paddle.
pub fn mouse() -> Option<V2> {
    let (x, y) = mouse_position();
    let p = View::current()?.to_scene(x, y);
    (p.x.is_finite() && p.y.is_finite()).then_some(p)
}

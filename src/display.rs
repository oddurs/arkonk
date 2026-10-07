//! Window mode, minimum size, and the simulation hold around display changes.
use crate::diagnostics;
use arkonk::timing::Settle;
use macroquad::prelude::*;

/// Half the scene in physical pixels. Below it, one-pixel glyph cells stretch
/// text past its layout, so the window is not allowed to shrink further.
pub const MIN_PHYSICAL: (f32, f32) = (480.0, 450.0);

pub struct Display {
    fullscreen: bool,
    size: (f32, f32),
    dpi: f32,
    settle: Settle,
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    windowed: (f32, f32),
}
impl Display {
    pub fn new(fullscreen: bool, windowed: (f32, f32)) -> Self {
        let size = (screen_width(), screen_height());
        Self {
            fullscreen,
            size,
            dpi: 0.0,
            settle: Settle::new(size),
            windowed,
        }
    }
    /// Applies the requested mode, keeps a window above its minimum size, and
    /// returns true while the simulation must hold for a display transition.
    pub fn update(&mut self, fullscreen: bool, frame_seconds: f64) -> bool {
        let switched = fullscreen != self.fullscreen;
        if switched {
            self.fullscreen = fullscreen;
            set_fullscreen(fullscreen);
            // macOS restores the previous frame when leaving its fullscreen
            // Space; other backends would keep the monitor-sized window.
            #[cfg(not(target_os = "macos"))]
            if !fullscreen {
                request_new_screen_size(self.windowed.0, self.windowed.1);
            }
            diagnostics::info(if fullscreen {
                "Display: fullscreen"
            } else {
                "Display: windowed"
            });
        }
        let dpi = screen_dpi_scale();
        let min = (MIN_PHYSICAL.0 / dpi, MIN_PHYSICAL.1 / dpi);
        // AppKit enforces a minimum natively; the scale changes between displays.
        #[cfg(target_os = "macos")]
        if dpi != self.dpi {
            miniquad::native::macos::set_content_min_size(f64::from(min.0), f64::from(min.1));
        }
        self.dpi = dpi;
        let size = (screen_width(), screen_height());
        if size != self.size {
            self.size = size;
            // Zero means minimized; that is not a size to correct. AppKit
            // already refuses smaller windows, so this corrects other backends.
            if !self.fullscreen && size.0 > 0.0 && size.1 > 0.0 {
                if size.0 < min.0 || size.1 < min.1 {
                    request_new_screen_size(size.0.max(min.0), size.1.max(min.1));
                } else {
                    self.windowed = size;
                }
            }
        }
        self.settle.hold(size, switched, frame_seconds)
    }
}

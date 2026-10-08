//! Window mode, minimum size, and the simulation hold around display changes.
use crate::diagnostics;
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

/// Holds the simulation while the window changes size or display mode, so a
/// resize or fullscreen transition can neither drain a ball nor read as a stall.
pub struct Settle {
    size: (f32, f32),
    remaining: f64,
}
impl Settle {
    /// The window must keep one size this long before play resumes.
    pub const QUIET: f64 = 0.5;
    /// A mode switch animates (a macOS Space transition takes about 0.7 s).
    pub const MODE_SWITCH: f64 = 1.0;
    pub fn new(size: (f32, f32)) -> Self {
        Self {
            size,
            remaining: 0.0,
        }
    }
    /// True while the simulation must hold. A frame that arrives inside the
    /// window is held even if it is long: it belongs to the transition.
    pub fn hold(&mut self, size: (f32, f32), mode_switched: bool, seconds: f64) -> bool {
        if size != self.size {
            self.size = size;
            self.remaining = self.remaining.max(Self::QUIET);
        }
        if mode_switched {
            self.remaining = Self::MODE_SWITCH;
        }
        let hold = self.remaining > 0.0;
        self.remaining -= seconds.max(0.0);
        hold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn steady_window_never_holds() {
        let mut settle = Settle::new((960.0, 900.0));
        for _ in 0..1000 {
            assert!(!settle.hold((960.0, 900.0), false, 1.0 / 120.0));
        }
    }
    #[test]
    fn resize_holds_until_size_is_quiet() {
        let mut settle = Settle::new((960.0, 900.0));
        let frame = 1.0 / 60.0;
        // A live drag: the size changes every frame for half a second.
        for i in 1..=30 {
            assert!(settle.hold((960.0 + i as f32, 900.0), false, frame));
        }
        let mut held = 0;
        while settle.hold((990.0, 900.0), false, frame) {
            held += 1;
        }
        let quiet = held as f64 * frame;
        assert!((quiet - Settle::QUIET).abs() <= frame, "held {quiet} s");
    }
    #[test]
    fn long_transition_frames_are_held_not_stalls() {
        let mut settle = Settle::new((960.0, 900.0));
        assert!(settle.hold((960.0, 900.0), true, 1.0 / 120.0));
        // The window server blocks presentation during the switch.
        assert!(settle.hold((960.0, 900.0), false, 0.4));
        assert!(settle.hold((1728.0, 1117.0), false, 0.45));
        assert!(settle.hold((1728.0, 1117.0), false, 0.3));
        assert!(!settle.hold((1728.0, 1117.0), false, 1.0 / 120.0));
    }
}

//! Folds gamepads into the same `Controls` and paddle axis the keyboard and mouse
//! produce, and remembers which kind of device the player touched last so the
//! cursor and on-screen prompts can follow them.
use crate::ui::Controls;
use gilrs::{Axis, Button, EventType, Gilrs};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Device {
    #[default]
    KeyboardMouse,
    Gamepad(Pad),
}
/// Which face-button glyphs a pad shows. Steam Deck and Steam Input
/// present the Xbox layout, which is also the default for unknown pads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Pad {
    #[default]
    Xbox,
    PlayStation,
}
impl Pad {
    /// Sony's USB vendor id marks a PlayStation pad.
    pub fn from_vendor(vendor: Option<u16>) -> Self {
        if vendor == Some(0x054C) {
            Pad::PlayStation
        } else {
            Pad::Xbox
        }
    }
}
impl Device {
    /// Explicit input decides; a frame with both keeps the current device so
    /// prompts never flicker. Losing the last pad hands control back to the
    /// pointer, otherwise the cursor would stay hidden with nothing to drive it.
    pub fn next(self, pointer: bool, pad: Option<Pad>, pads_connected: bool) -> Self {
        match (pointer, pad) {
            (false, Some(pad)) => Self::Gamepad(pad),
            (true, None) => Self::KeyboardMouse,
            (false, None) if !pads_connected => Self::KeyboardMouse,
            _ => self,
        }
    }
    pub fn is_pad(self) -> bool {
        matches!(self, Self::Gamepad(_))
    }
}

/// Worn sticks rarely rest exactly at zero; below this the paddle must not drift.
const INNER: f32 = 0.2;
/// Many sticks never report a full 1.0 on the diagonal or even the axis.
const OUTER: f32 = 0.95;
/// Menus need a deliberate push, well past the gameplay deadzone.
const TILT: f32 = 0.5;
const REPEAT_DELAY: f64 = 0.4;
const REPEAT_INTERVAL: f64 = 0.12;

/// A radial deadzone keeps diagonals smooth, and rescaling from its edge lets a
/// light push still move the paddle slowly instead of jumping to 20 % speed.
pub fn deadzone(x: f32, y: f32) -> (f32, f32) {
    let magnitude = x.hypot(y);
    if magnitude <= INNER {
        return (0.0, 0.0);
    }
    let scaled = ((magnitude - INNER) / (OUTER - INNER)).min(1.0);
    (x / magnitude * scaled, y / magnitude * scaled)
}

pub fn merge_axis(a: f32, b: f32) -> f32 {
    (a + b).clamp(-1.0, 1.0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
/// Expects deadzoned values with positive y pointing up, as gilrs reports it.
pub fn stick_dir(x: f32, y: f32) -> Option<Dir> {
    if x.hypot(y) < TILT {
        None
    } else if x.abs() > y.abs() {
        Some(if x > 0.0 { Dir::Right } else { Dir::Left })
    } else {
        Some(if y > 0.0 { Dir::Up } else { Dir::Down })
    }
}

/// Turns a held direction into discrete menu steps: one immediately, then a
/// steady repeat after a pause, like a keyboard's auto-repeat.
#[derive(Default)]
pub struct Repeat {
    held: Option<Dir>,
    wait: f64,
}
impl Repeat {
    pub fn update(&mut self, dir: Option<Dir>, dt: f64) -> Option<Dir> {
        if dir != self.held {
            self.held = dir;
            self.wait = REPEAT_DELAY;
            return dir;
        }
        self.wait -= dt;
        if dir.is_some() && self.wait <= 0.0 {
            // Reset rather than accumulate, so a long stall yields one step, not a burst.
            self.wait = REPEAT_INTERVAL;
            return dir;
        }
        None
    }
}

/// Buttons that went down this frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct Presses {
    pub south: bool,
    pub east: bool,
    pub west: bool,
    pub start: bool,
    /// The shoulder buttons turn sector select's chapters.
    pub lb: bool,
    pub rb: bool,
}

/// Xbox layout: A confirms and serves, B backs out, X retries, Start pauses.
/// B also closes the pause and results menus, where it means "back".
pub fn pad_controls(pressed: Presses, step: Option<Dir>, menu_open: bool) -> Controls {
    Controls {
        confirm: pressed.south,
        escape: pressed.east,
        pause: pressed.start || (pressed.east && menu_open),
        restart: pressed.west,
        up: step == Some(Dir::Up),
        down: step == Some(Dir::Down),
        left: step == Some(Dir::Left),
        right: step == Some(Dir::Right),
        page: i8::from(pressed.rb) - i8::from(pressed.lb),
        ..Controls::default()
    }
}

/// Any key or mouse button counts, so even a hotkey brings keyboard prompts back.
pub fn pointer_pressed() -> bool {
    use macroquad::prelude::*;
    get_last_key_pressed().is_some()
        || is_mouse_button_pressed(MouseButton::Left)
        || is_mouse_button_pressed(MouseButton::Right)
}

pub fn merge(a: Controls, b: Controls) -> Controls {
    Controls {
        click: a.click || b.click,
        confirm: a.confirm || b.confirm,
        escape: a.escape || b.escape,
        pause: a.pause || b.pause,
        up: a.up || b.up,
        down: a.down || b.down,
        left: a.left || b.left,
        right: a.right || b.right,
        page: if a.page != 0 { a.page } else { b.page },
        restart: a.restart || b.restart,
        focus_lost: a.focus_lost || b.focus_lost,
    }
}

#[derive(Default)]
pub struct PadFrame {
    pub controls: Controls,
    pub axis: f32,
    /// The controller in use went away; play should stop until the player is back.
    pub lost: bool,
}

pub struct Gamepads {
    gilrs: Option<Gilrs>,
    repeat: Repeat,
    pub device: Device,
}
impl Gamepads {
    /// Test runs pass `false` so a controller on the desk cannot steer them.
    pub fn new(enabled: bool) -> Self {
        let gilrs = if enabled {
            match Gilrs::new() {
                Ok(gilrs) => Some(gilrs),
                Err(e) => {
                    crate::diagnostics::error(format_args!("Gamepads unavailable: {e}"));
                    None
                }
            }
        } else {
            None
        };
        Self {
            gilrs,
            repeat: Repeat::default(),
            device: Device::default(),
        }
    }

    /// Call once per frame. Input is ignored while the window is in the
    /// background, because gamepads keep reporting regardless of focus.
    pub fn poll(&mut self, dt: f64, focused: bool, menu_open: bool, pointer: bool) -> PadFrame {
        let Some(gilrs) = &mut self.gilrs else {
            self.device = self.device.next(pointer, None, false);
            return PadFrame::default();
        };
        let mut pressed = Presses::default();
        let mut tapped = None;
        // The pad touched this frame, by its family.
        let mut activity = None;
        let mut disconnected = false;
        // Every event must be drained, even unfocused, or gilrs' state goes stale.
        while let Some(event) = gilrs.next_event() {
            match event.event {
                EventType::ButtonPressed(button, _) if focused => {
                    activity = Some(Pad::from_vendor(gilrs.gamepad(event.id).vendor_id()));
                    match button {
                        Button::South => pressed.south = true,
                        Button::East => pressed.east = true,
                        Button::West => pressed.west = true,
                        Button::Start => pressed.start = true,
                        Button::LeftTrigger => pressed.lb = true,
                        Button::RightTrigger => pressed.rb = true,
                        // A tap shorter than a frame is gone from the held state.
                        Button::DPadUp => tapped = Some(Dir::Up),
                        Button::DPadDown => tapped = Some(Dir::Down),
                        Button::DPadLeft => tapped = Some(Dir::Left),
                        Button::DPadRight => tapped = Some(Dir::Right),
                        _ => {}
                    }
                }
                EventType::Disconnected => disconnected = true,
                _ => {}
            }
        }
        let mut axis = 0.0f32;
        let mut held = None;
        let mut connected = false;
        for (_, pad) in gilrs.gamepads() {
            connected = true;
            if !focused {
                continue;
            }
            let (x, y) = deadzone(pad.value(Axis::LeftStickX), pad.value(Axis::LeftStickY));
            let dpad = f32::from(u8::from(pad.is_pressed(Button::DPadRight)))
                - f32::from(u8::from(pad.is_pressed(Button::DPadLeft)));
            let pad_axis = merge_axis(dpad, x);
            if pad_axis.abs() > axis.abs() {
                axis = pad_axis;
            }
            let stick = stick_dir(x, y);
            if stick.is_some() {
                activity = Some(Pad::from_vendor(pad.vendor_id()));
            }
            held = held.or_else(|| {
                [
                    (Button::DPadUp, Dir::Up),
                    (Button::DPadDown, Dir::Down),
                    (Button::DPadLeft, Dir::Left),
                    (Button::DPadRight, Dir::Right),
                ]
                .into_iter()
                .find(|&(button, _)| pad.is_pressed(button))
                .map(|(_, dir)| dir)
                .or(stick)
            });
        }
        let lost = disconnected && self.device.is_pad();
        self.device = self.device.next(pointer, activity, connected);
        let step = self.repeat.update(held.or(tapped), dt);
        PadFrame {
            controls: pad_controls(pressed, step, menu_open),
            axis,
            lost,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: (f32, f32), b: (f32, f32)) -> bool {
        (a.0 - b.0).abs() < 1e-5 && (a.1 - b.1).abs() < 1e-5
    }

    #[test]
    fn deadzone_is_radial_and_rescaled() {
        assert_eq!(deadzone(0.0, 0.0), (0.0, 0.0));
        assert_eq!(deadzone(0.19, 0.0), (0.0, 0.0));
        // Each axis alone is under the threshold; together they are not.
        assert_eq!(deadzone(0.14, 0.14), (0.0, 0.0));
        let (x, y) = deadzone(0.18, 0.18);
        assert!(x > 0.0 && (x - y).abs() < 1e-6);
        // Just past the edge starts near zero rather than jumping.
        assert!(deadzone(0.21, 0.0).0 < 0.02);
        assert!(close(deadzone(0.575, 0.0), (0.5, 0.0)));
        assert!(close(deadzone(-0.95, 0.0), (-1.0, 0.0)));
        assert!(close(deadzone(0.0, 1.0), (0.0, 1.0)));
        let diagonal = deadzone(1.0, 1.0);
        assert!((diagonal.0.hypot(diagonal.1) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn axes_merge_and_saturate() {
        assert_eq!(merge_axis(0.0, 0.4), 0.4);
        assert_eq!(merge_axis(1.0, 0.4), 1.0);
        assert_eq!(merge_axis(-1.0, -0.7), -1.0);
        assert_eq!(merge_axis(1.0, -1.0), 0.0);
        assert_eq!(merge_axis(-1.0, 0.25), -0.75);
    }

    #[test]
    fn stick_needs_a_deliberate_push_for_menus() {
        assert_eq!(stick_dir(0.3, 0.2), None);
        assert_eq!(stick_dir(0.6, 0.1), Some(Dir::Right));
        assert_eq!(stick_dir(-0.6, 0.1), Some(Dir::Left));
        assert_eq!(stick_dir(0.1, 0.6), Some(Dir::Up));
        assert_eq!(stick_dir(0.1, -0.6), Some(Dir::Down));
    }

    #[test]
    fn held_direction_steps_then_repeats() {
        let mut r = Repeat::default();
        // A frame time that never lands exactly on a repeat boundary.
        let frame = 0.035;
        assert_eq!(r.update(Some(Dir::Down), frame), Some(Dir::Down));
        let mut steps = 0;
        // Through 0.385 s: still inside the initial delay.
        for _ in 1..=11 {
            steps += usize::from(r.update(Some(Dir::Down), frame).is_some());
        }
        assert_eq!(steps, 0);
        // Through 0.98 s: one repeat at 0.42 s, then every fourth frame.
        for _ in 12..=28 {
            steps += usize::from(r.update(Some(Dir::Down), frame).is_some());
        }
        assert_eq!(steps, 5);
        assert_eq!(r.update(None, frame), None);
        assert_eq!(r.update(Some(Dir::Down), frame), Some(Dir::Down));
        // Changing direction steps at once, without waiting out the delay.
        assert_eq!(r.update(Some(Dir::Up), frame), Some(Dir::Up));
    }

    #[test]
    fn a_stall_repeats_once_not_in_a_burst() {
        let mut r = Repeat::default();
        r.update(Some(Dir::Left), 0.0);
        assert_eq!(r.update(Some(Dir::Left), 2.0), Some(Dir::Left));
        assert_eq!(r.update(Some(Dir::Left), 1.0 / 60.0), None);
    }

    #[test]
    fn device_follows_the_last_input() {
        use Device::*;
        let (xbox, ps) = (Gamepad(Pad::Xbox), Gamepad(Pad::PlayStation));
        assert_eq!(KeyboardMouse.next(false, Some(Pad::Xbox), true), xbox);
        assert_eq!(xbox.next(true, None, true), KeyboardMouse);
        assert_eq!(xbox.next(false, None, true), xbox);
        assert_eq!(KeyboardMouse.next(false, None, true), KeyboardMouse);
        assert_eq!(xbox.next(true, Some(Pad::Xbox), true), xbox);
        assert_eq!(
            KeyboardMouse.next(true, Some(Pad::Xbox), true),
            KeyboardMouse
        );
        assert_eq!(xbox.next(false, None, false), KeyboardMouse);
        // Touching another pad switches every glyph to its family.
        assert_eq!(xbox.next(false, Some(Pad::PlayStation), true), ps);
    }

    #[test]
    fn sony_pads_show_playstation_glyphs() {
        assert_eq!(Pad::from_vendor(Some(0x054C)), Pad::PlayStation);
        assert_eq!(Pad::from_vendor(Some(0x045E)), Pad::Xbox);
        assert_eq!(Pad::from_vendor(Some(0x28DE)), Pad::Xbox);
        assert_eq!(Pad::from_vendor(None), Pad::Xbox);
    }

    #[test]
    fn pad_buttons_map_to_controls() {
        let none = Presses::default();
        let c = pad_controls(
            Presses {
                south: true,
                ..none
            },
            None,
            false,
        );
        assert!(c.confirm && !c.pause && !c.escape);
        let c = pad_controls(
            Presses {
                start: true,
                ..none
            },
            None,
            false,
        );
        assert!(c.pause && !c.escape);
        // B only pauses (toggles) when it closes an open menu.
        let c = pad_controls(Presses { east: true, ..none }, None, false);
        assert!(c.escape && !c.pause);
        let c = pad_controls(Presses { east: true, ..none }, None, true);
        assert!(c.escape && c.pause);
        let c = pad_controls(Presses { west: true, ..none }, None, true);
        assert!(c.restart && !c.confirm);
        let c = pad_controls(none, Some(Dir::Left), false);
        assert!(c.left && !c.right && !c.up && !c.down);
        assert!(!c.click && !c.focus_lost);
        // The shoulders turn chapters, apart from the d-pad's steps.
        let c = pad_controls(Presses { lb: true, ..none }, None, false);
        assert!(c.page == -1 && !c.left && !c.right);
        let c = pad_controls(Presses { rb: true, ..none }, None, false);
        assert!(c.page == 1 && !c.left && !c.right);
    }

    #[test]
    fn merged_controls_keep_either_source() {
        let keys = Controls {
            click: true,
            up: true,
            ..Controls::default()
        };
        let pad = pad_controls(
            Presses {
                south: true,
                ..Presses::default()
            },
            Some(Dir::Right),
            false,
        );
        let c = merge(keys, pad);
        assert!(c.click && c.up && c.confirm && c.right);
        assert!(!c.down && !c.pause && !c.restart);
    }
}

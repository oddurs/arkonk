//! Input glyphs: neutral glass chips lit from above, in the family of the
//! device used last. Keys are rounded keycaps, face buttons are circles,
//! and shapes and icons are drawn from primitives, never brand colours.
use super::*;
use crate::{input::Pad, ui::Prompt};

/// How a chip is lit.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Lit {
    Neutral,
    /// On the primary action: cyan glass.
    Primary,
    /// Its input just fired.
    Pressed,
    Unavailable,
}

/// What a chip shows.
#[derive(Clone, Copy, PartialEq)]
enum Face {
    Key(TextId),
    Enter,
    Arrow(bool),
    Mouse,
    /// A pad face button: its letter on the Xbox layout, or the
    /// PlayStation shape.
    Button(Pad, u8),
    /// The d-pad with one arm lit.
    DPad(bool),
}
/// The face buttons in use, by position: south confirms, east goes back.
const CROSS: u8 = 0;
const CIRCLE: u8 = 1;

/// What `prompt` looks like on `device`; `mouse` when the mouse, rather
/// than the keyboard, has been driving.
fn face(prompt: Prompt, device: Device, mouse: bool) -> Face {
    match (device, prompt) {
        (Device::KeyboardMouse, Prompt::Serve) if mouse => Face::Mouse,
        (Device::KeyboardMouse, Prompt::Serve) => Face::Key(TextId::KeySpace),
        (Device::KeyboardMouse, Prompt::Confirm) => Face::Enter,
        (Device::KeyboardMouse, Prompt::Back) => Face::Key(TextId::KeyEsc),
        (Device::KeyboardMouse, Prompt::Left) => Face::Arrow(true),
        (Device::KeyboardMouse, Prompt::Right) => Face::Arrow(false),
        (Device::Gamepad(pad), Prompt::Confirm | Prompt::Serve) => Face::Button(pad, CROSS),
        (Device::Gamepad(pad), Prompt::Back) => Face::Button(pad, CIRCLE),
        (Device::Gamepad(_), Prompt::Left) => Face::DPad(true),
        (Device::Gamepad(_), Prompt::Right) => Face::DPad(false),
    }
}

/// The chip's label size for a chip `size` tall.
fn label_size(size: f32) -> f32 {
    size * 13.0 / 28.0
}

/// How wide `prompt`'s chip is at `size`: at least square, keys grow
/// with their label.
pub(super) fn width(v: &Scene, prompt: Prompt, size: f32) -> f32 {
    match face(prompt, v.device, v.mouse) {
        Face::Key(id) => {
            let label = v.width_of(id, &[], label_style(size));
            (label + 2.0 * size * 8.0 / 28.0).max(size)
        }
        _ => size,
    }
}

fn label_style(size: f32) -> Style {
    Style::from(Role::Label).sized(label_size(size)).untracked()
}

/// Draws `prompt`'s chip, `size` tall, from `x` with its middle at `mid`,
/// and returns its width.
pub(super) fn chip(v: &Scene, prompt: Prompt, (x, mid): (f32, f32), size: f32, lit: Lit) -> f32 {
    let lit = if lit != Lit::Unavailable && v.pressed.is(prompt) {
        Lit::Pressed
    } else {
        lit
    };
    let face = face(prompt, v.device, v.mouse);
    let w = width(v, prompt, size);
    // Pressed chips sink by a unit.
    let drop = if lit == Lit::Pressed { 1.0 } else { 0.0 };
    let r = v.snap_rect(Rect::new(x, mid - size / 2.0 + drop, w, size));
    let round = matches!(face, Face::Button(..) | Face::DPad(_) | Face::Mouse);
    let radius = if round { size / 2.0 } else { 8.0 * size / 28.0 };
    let (fill, rim, ink) = match lit {
        Lit::Neutral => (
            Fill::ramp(hex(0x1a1e2b), hex(0x131620)),
            Fill::ramp(hex(0x525c73), hex(0x22283a)),
            hex(0xcfd6e3),
        ),
        Lit::Primary => (
            Fill::ramp(
                mix(hex(0x10131c), CYAN, 0.24),
                mix(hex(0x10131c), CYAN, 0.10),
            ),
            Fill::lit(hex(0xb8f3fc), 0.45, CYAN, hex(0x2b8fa3)),
            hex(0xdffaff),
        ),
        Lit::Pressed => (
            Fill::ramp(hex(0x2a3142), hex(0x1d2230)),
            Fill::ramp(WHITE, hex(0x9aa6bd)),
            WHITE,
        ),
        Lit::Unavailable => (Fill::flat(hex(0x10131c)), Fill::flat(hex(0x1d2230)), MUTED),
    };
    let t = v.thick(1.5);
    v.shape(r, [radius; 4], rim);
    let inner = Rect::new(r.x + t, r.y + t, r.w - 2.0 * t, r.h - 2.0 * t);
    v.shape(inner, [(radius - t).max(0.0); 4], fill);
    let c = r.center();
    let u = size / 28.0;
    let stroke = v.thick(1.5 * u);
    let line = |a: (f32, f32), b: (f32, f32)| {
        v.line(
            V2::new(c.x + a.0 * u, c.y + a.1 * u),
            V2::new(c.x + b.0 * u, c.y + b.1 * u),
            stroke,
            ink,
        )
    };
    match face {
        Face::Key(id) => {
            let style = label_style(size);
            let baseline = v.snap(c.y + v.cap(style) / 2.0);
            v.say(id, &[], style, Slot::centered(c.x, w, baseline), ink);
        }
        Face::Enter => {
            line((5.0, -5.0), (5.0, 1.0));
            line((5.0, 1.0), (-5.0, 1.0));
            line((-5.0, 1.0), (-2.0, -2.0));
            line((-5.0, 1.0), (-2.0, 4.0));
        }
        Face::Arrow(left) => {
            let d = if left { -1.0 } else { 1.0 };
            line((-5.0 * d, 0.0), (5.0 * d, 0.0));
            line((5.0 * d, 0.0), (1.5 * d, -3.5));
            line((5.0 * d, 0.0), (1.5 * d, 3.5));
        }
        Face::Mouse => {
            // An outline with its left button lit.
            let body = Rect::new(c.x - 5.0 * u, c.y - 7.5 * u, 10.0 * u, 15.0 * u);
            v.outline(body, 5.0 * u, stroke, ink);
            let button = Rect::new(
                body.x + stroke,
                body.y + stroke,
                body.w / 2.0 - stroke,
                5.0 * u,
            );
            v.shape(button, [4.0 * u, 0.0, 0.0, 0.0], Fill::flat(CYAN));
            line((-5.0, -2.0), (5.0, -2.0));
            line((0.0, -7.5), (0.0, -2.0));
        }
        Face::Button(Pad::Xbox, n) => {
            let style = label_style(size);
            let baseline = v.snap(c.y + v.cap(style) / 2.0);
            let letter = if n == CROSS { "A" } else { "B" };
            v.put(letter, style, Slot::centered(c.x, w, baseline), ink);
        }
        Face::Button(Pad::PlayStation, CROSS) => {
            line((-4.0, -4.0), (4.0, 4.0));
            line((4.0, -4.0), (-4.0, 4.0));
        }
        Face::Button(Pad::PlayStation, _) => {
            v.ring(V2::new(c.x, c.y), 4.5 * u - stroke, stroke, ink)
        }
        Face::DPad(left) => {
            // A cross, its pressed arm lit cyan.
            let arm = 2.5 * u;
            let reach = 7.0 * u;
            v.rect(
                c.x - arm,
                c.y - reach,
                2.0 * arm,
                2.0 * reach,
                opacity(ink, 0.5),
            );
            v.rect(
                c.x - reach,
                c.y - arm,
                2.0 * reach,
                2.0 * arm,
                opacity(ink, 0.5),
            );
            let x = if left { c.x - reach } else { c.x + arm };
            v.rect(x, c.y - arm, reach - arm, 2.0 * arm, CYAN);
        }
    }
    w
}

/// A prompt in the field: a chip and its word, `size`-unit chip beside
/// `role` text, centred on `cx` or starting at `x`; returns its width.
pub(super) fn prompt_width(
    v: &Scene,
    prompt: Prompt,
    (id, style): (TextId, Style),
    size: f32,
) -> f32 {
    width(v, prompt, size) + S12 + v.width_of(id, &[], style)
}
pub(super) fn prompt(
    v: &Scene,
    prompt: Prompt,
    (id, style): (TextId, Style),
    (x, baseline): (f32, f32),
    size: f32,
    color: Color,
) {
    let mid = baseline - v.cap(style) / 2.0;
    let w = chip(v, prompt, (x, mid), size, Lit::Neutral);
    let text = x + w + S12;
    let room = v.width_of(id, &[], style);
    v.say(id, &[], style, Slot::left(text, room, baseline), color);
}

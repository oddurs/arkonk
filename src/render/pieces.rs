//! The pieces, "neon glass": bricks and capsules are tinted glass with a
//! lit neon rim; the ball and the paddle are the only solid things, pearl
//! lit from above. One light, drawn as a colour per vertex on shapes
//! already drawn; the glow each piece casts is the atlas sprite, so a frame
//! stays one batch. State is worn by the pieces in one grammar: counts are
//! pips, durations drain toward their centre, events flash and are gone.
//!
//! A Compact screen builds the same pieces flat, in whole pixels.
use super::{
    status::{self, Flash},
    *,
};
use crate::pictogram;
use ark::{
    Ball, Capsule,
    field::{CellSet, ROWS},
    tuning::PHASE_CONTACTS,
};

/// The play field's glass, which the pieces' tints mix into.
pub(super) const FIELD: Color = hex(0x0c0e16);
/// Solid light: white above, cool beneath.
const PEARL: Fill = Fill::lit(WHITE, 0.38, hex(0xf1f5fa), hex(0xc8d2e1));
/// The pearl struck: white all through.
const PEARL_LIT: Fill = Fill::ramp(WHITE, hex(0xe6f6fb));
const SEAM: Fill = Fill::ramp(hex(0x2bb7d3), CYAN);
/// An Anchor charge used.
const SEAM_OUT: Color = hex(0xc8d2e1);
const AMBER_LIGHT: Color = hex(0xffe2a3);
/// The ball's halo: white light, no hue.
const BALL_LIGHT: Color = hex(0xdcf0ff);
const TRAIL: Color = Color::new(200.0 / 255.0, 236.0 / 255.0, 1.0, 1.0);
const PHASE_LIT: Fill = Fill::ramp(hex(0xc9cbff), INDIGO);
/// Where a held ball rests on the pearl.
const CONTACT_SHADOW: Color = Color::new(40.0 / 255.0, 52.0 / 255.0, 74.0 / 255.0, 0.55);
/// A spent relay core's empty socket.
const SOCKET: Color = hex(0x0e0d0b);
/// The Compact paddle: pearl, flat.
const FLAT_PEARL: Color = hex(0xe9eef5);

/// Brick corners; capsules and the paddle are full pills.
const BRICK_RADIUS: f32 = 4.0;
/// The glow's reach beyond a piece, and how far below it lands.
const GLOW_GROW: f32 = 6.0;
/// A ball's light fades over the field's last stretch.
const DRAIN_FADE: f32 = 30.0;
/// The red hairline when the last ball is lost.
const DRAIN_LINE: f32 = 0.3;
/// A spent core's socket shows this long.
const SOCKET_SECONDS: f32 = 0.25;
/// Ticks of a relay blast's flash, from the simulation.
const IGNITION_TICKS: f32 = ark::RELAY_FLASH_TICKS as f32;

/// High contrast lifts the dim hues to stay 3:1 or better on the field.
pub(super) fn lifted(hue: Color) -> Color {
    if hue == INDIGO {
        hex(0xa3a7ff)
    } else if hue == RED {
        hex(0xff5c6e)
    } else {
        hue
    }
}

/// A hue as glass: every layer's colour, mixed once per sector.
#[derive(Clone, Copy, Debug)]
pub(super) struct Glass {
    hue: Color,
    /// Lifted toward white along the top, sunk toward the field below.
    rim: Fill,
    body: Fill,
    /// What a hit lifts the body to.
    hit: Fill,
    /// Armour: the panes between rims, and the second and third rims.
    pane: Color,
    rim2: Color,
    rim3: Color,
    /// The hue lit: shards and the ring a break leaves.
    lit: Color,
    /// The one bright thing inside: a pictogram or a diamond.
    mark: Color,
}
impl Glass {
    /// `hue` as glass whose body is `top` to `bottom` percent of it over
    /// the field.
    fn new(hue: Color, (top, bottom): (f32, f32), high: bool) -> Self {
        let hue = if high { lifted(hue) } else { hue };
        let over = |percent: f32| mix(FIELD, hue, percent / 100.0);
        if high {
            // No tint: the field shows through full-hue rims.
            return Self {
                hue,
                rim: Fill::flat(hue),
                body: Fill::flat(FIELD),
                hit: Fill::ramp(over(55.0), over(35.0)),
                pane: FIELD,
                rim2: hue,
                rim3: hue,
                lit: mix(hue, WHITE, 0.2),
                mark: WHITE,
            };
        }
        Self {
            hue,
            rim: Fill::lit(mix(hue, WHITE, 0.48), 0.45, hue, over(58.0)),
            body: Fill::ramp(over(top), over(bottom)),
            hit: Fill::ramp(over(55.0), over(35.0)),
            pane: over(18.0),
            rim2: over(62.0),
            rim3: over(44.0),
            lit: mix(hue, WHITE, 0.2),
            mark: mix(hue, WHITE, 0.72),
        }
    }
}

/// How much of a brick hue its glass body takes, top and bottom. Measured
/// on OpenGL captures, red, indigo and orchid glass at the shared 26 %
/// came out 6 to 9 L* darker than the other hues and read muddy, so they
/// take the most the design allows at the top.
fn brick_body(hue: Color) -> (f32, f32) {
    if hue == INDIGO || hue == RED || hue == ORCHID {
        (BODY_DIM_TOP, 11.0)
    } else {
        (26.0, 11.0)
    }
}
/// The dim hues' body top, in percent.
const BODY_DIM_TOP: f32 = 32.0;

/// Every glass of a sector: its brick rows, the relay cores, the powers.
#[derive(Clone, Copy, Debug)]
pub(super) struct Palette {
    key: (Chapter, bool),
    rows: [Glass; ROWS],
    core: Glass,
    powers: [Glass; 5],
}
impl Default for Palette {
    fn default() -> Self {
        Self::new(Chapter::Daybreak, false)
    }
}
impl Palette {
    fn new(chapter: Chapter, high: bool) -> Self {
        let row = |r: usize| {
            let hue = sector_color(r, chapter);
            Glass::new(hue, brick_body(hue), high)
        };
        Self {
            key: (chapter, high),
            rows: std::array::from_fn(row),
            core: Glass::new(AMBER, (22.0, 9.0), high),
            powers: Power::ALL.map(|p| Glass::new(power_color(p), (30.0, 12.0), high)),
        }
    }
    /// This palette, or a fresh one if it was mixed for another chapter
    /// or contrast.
    pub(super) fn refresh(self, chapter: Chapter, high: bool) -> Self {
        if self.key == (chapter, high) {
            self
        } else {
            Self::new(chapter, high)
        }
    }
    fn power(&self, power: Power) -> &Glass {
        &self.powers[power_index(power)]
    }
}
fn power_index(power: Power) -> usize {
    Power::ALL.iter().position(|&p| p == power).unwrap_or(0)
}

fn grow(r: Rect, x: f32, y: f32) -> Rect {
    Rect::new(r.x - x, r.y - y, r.w + 2.0 * x, r.h + 2.0 * y)
}
fn inset(r: Rect, t: f32) -> Rect {
    grow(r, -t, -t)
}
fn cell_box(cell: FieldCell) -> Rect {
    let r = cell_rect(cell);
    Rect::new(r.x, r.y, r.w, r.h)
}
/// `fill` brightened toward `to` by `k`, stop by stop.
fn toward(fill: Fill, to: Color, k: f32) -> Fill {
    Fill {
        top: mix(fill.top, to, k),
        mid: fill.mid.map(|(at, c)| (at, mix(c, to, k))),
        bottom: mix(fill.bottom, to, k),
    }
}
/// Between two fills, `k` of the way to `b`.
fn blend(a: Fill, b: Fill, k: f32) -> Fill {
    Fill {
        top: mix(a.top, b.at(0.0), k),
        mid: a.mid.map(|(at, c)| (at, mix(c, b.at(at), k))),
        bottom: mix(a.bottom, b.at(1.0), k),
    }
}
/// The part of `fill`, laid over `outer`, that falls on `inner`.
fn part(fill: Fill, outer: Rect, inner: Rect) -> Fill {
    let at = |y: f32| fill.at((y - outer.y) / outer.h);
    Fill::ramp(at(inner.y), at(inner.y + inner.h))
}

/// The light a piece casts: the glow grown past its edges and landing a
/// little below it, the light being above.
fn cast(v: &Scene, r: Rect, color: Color) {
    v.glow(
        Rect::new(
            r.x - GLOW_GROW,
            r.y,
            r.w + 2.0 * GLOW_GROW,
            r.h + 2.0 * GLOW_GROW,
        ),
        color,
    );
}

/// A line whose colour runs from `from` at `a` to `to` at `b`.
fn streak(v: &Scene, a: V2, b: V2, thickness: f32, (from, to): (Color, Color)) {
    let (a, b) = (vec2(a.x, a.y), vec2(b.x, b.y));
    let along = b - a;
    let length = along.length();
    if length < f32::EPSILON {
        return;
    }
    let t = along.perp() * (thickness * 0.5 / length);
    let vertices = [
        v.vertex(a + t, from),
        v.vertex(a - t, from),
        v.vertex(b - t, to),
        v.vertex(b + t, to),
    ];
    v.mesh(&vertices, &[0, 1, 2, 0, 2, 3]);
}

/// Everything on the field in play, bottom to top.
pub(super) fn draw(v: &Scene, fx: &Fx, game: &Game, alpha: f32) {
    let look = v.look.get();
    let palette = fx
        .palette
        .refresh(game.sector().sector().chapter, look.high);
    filaments(v, game);
    bricks(v, fx, game, &palette);
    ignitions(v, game);
    shards(v, fx, game, &palette);
    paddle(v, fx, game, &palette);
    for drop in game.capsules().iter().filter(|d| d.active) {
        capsule(v, drop, palette.power(drop.power));
    }
    balls(v, fx, game, alpha);
    drain_line(v, fx);
}

// Bricks.

/// Linked live cores share a filament along their blast route, under the
/// glass, so it shows in the gap between them.
fn filaments(v: &Scene, game: &Game) {
    let board = game.board();
    let live = |c: FieldCell| board.is_core(c) && board.hp(c) > 0;
    let flat = v.style() == PieceStyle::Flat;
    for cell in FieldCell::all().filter(|&c| live(c)) {
        let from = cell_box(cell).center();
        let [_, right, _, down] = cell.neighbors();
        for other in [right, down].into_iter().flatten().filter(|&c| live(c)) {
            let to = cell_box(other).center();
            let (a, b) = (V2::new(from.x, from.y), V2::new(to.x, to.y));
            if !flat {
                v.line(a, b, 3.0, opacity(AMBER, 0.25));
            }
            v.line(a, b, v.thick(1.0), AMBER);
        }
    }
}

fn bricks(v: &Scene, fx: &Fx, game: &Game, palette: &Palette) {
    let look = v.look.get();
    let board = game.board();
    // Core glow breathes over 2.6 s: the game's one idle motion.
    let seconds = game.sector_ticks() as f32 * DT;
    let breath = 0.5 + 0.15 * (seconds * TAU / 2.6).sin();
    let mut phased = CellSet::EMPTY;
    for ball in game.balls().iter().filter(|b| b.active) {
        for cell in ball.phased.iter() {
            phased.insert(cell);
        }
    }
    for cell in FieldCell::all() {
        let i = cell.index();
        let (hp, core) = (board.hp(cell), board.is_core(cell));
        let glass = if core {
            &palette.core
        } else {
            &palette.rows[cell.row()]
        };
        let r = v.snap_rect(cell_box(cell));
        let age = fx.brick_age[i];
        if hp == 0 {
            if core {
                socket(v, game, cell, r, age);
            } else {
                broken(v, r, glass, age);
            }
            continue;
        }
        let hit = Flash::HIT.at(age, look.reduced);
        // The rim a hit just took, when armour lost one.
        let lost = (hit > 0.0 && fx.brick_was[i] > hp).then_some(fx.brick_was[i]);
        let rim = if phased.contains(cell) {
            palette.power(Power::Phase).rim
        } else {
            glass.rim
        };
        let glow = if core { breath } else { 0.30 };
        let brick = Brick {
            r,
            glass,
            rim,
            hp,
            core,
            hit,
            lost,
            glow,
        };
        match v.style() {
            PieceStyle::Glass => glass_brick(v, &brick),
            PieceStyle::Flat => flat_brick(v, &brick),
        }
    }
}

/// One live brick, as the frame finds it.
struct Brick<'g> {
    r: Rect,
    glass: &'g Glass,
    rim: Fill,
    hp: u8,
    core: bool,
    /// The hit flash's strength now.
    hit: f32,
    /// The hit points before a hit that took an armour rim.
    lost: Option<u8>,
    /// The glow's opacity at rest.
    glow: f32,
}

/// Armour's nested panes, measured inward from inside the outer rim: a
/// pane, a rim, and for three hit points a second pane and rim. Inner
/// rims are flat, so the outer rim alone carries the light.
fn armour(v: &Scene, glass: &Glass, hp: u8) -> ([(f32, Color); 4], usize) {
    let (pane, rim2, rim3) = (v.thick(2.0), v.thick(1.25), v.thick(1.0));
    let layers = [
        (pane, glass.pane),
        (rim2, glass.rim2),
        (pane, glass.pane),
        (rim3, glass.rim3),
    ];
    let count = match hp {
        0 | 1 => 0,
        2 => 2,
        _ => 4,
    };
    (layers, count)
}

fn glass_brick(v: &Scene, b: &Brick) {
    let look = v.look.get();
    let (r, glass, hit) = (b.r, b.glass, b.hit);
    // Reduced effects draw no glow, so the cores' breathing stops with it.
    cast(v, r, opacity(glass.hue, b.glow + (0.75 - b.glow) * hit));
    v.shape(r, [BRICK_RADIUS; 4], toward(b.rim, WHITE, hit));
    let rim = v.thick(if look.high { 2.0 } else { 1.5 });
    let body = inset(r, rim);
    let fill = blend(glass.body, glass.hit, hit);
    let mut at = rim;
    let (layers, count) = armour(v, glass, b.hp);
    for &(t, colour) in &layers[..count] {
        pane(v, inset(r, at), BRICK_RADIUS - at, Fill::flat(colour));
        at += t;
    }
    let inner = inset(r, at);
    pane(v, inner, BRICK_RADIUS - at, part(fill, body, inner));
    if let Some(was) = b.lost {
        // The innermost rim shatters: white for the flash, then gone.
        let (layers, before) = armour(v, glass, was);
        let at = rim + layers[..before - 1].iter().map(|l| l.0).sum::<f32>();
        let t = layers[before - 1].0;
        let ring = inset(r, at);
        v.outline(ring, (BRICK_RADIUS - at).max(0.0), t, opacity(WHITE, hit));
    }
    if b.core {
        diamond(v, r.center(), glass.mark, look.high);
    }
}

/// A pane of glass inset in a brick: rounded while its corner still
/// curves, a plain quad once the inset has used the radius up.
fn pane(v: &Scene, r: Rect, radius: f32, fill: Fill) {
    if radius >= 0.75 {
        v.shape(r, [radius; 4], fill);
    } else {
        v.ramp(r, fill.at(0.0), fill.at(1.0));
    }
}

/// A relay core's inner mark: a lit diamond with a white centre.
fn diamond(v: &Scene, c: Vec2, colour: Color, high: bool) {
    // A 7.5-unit square turned 45°, drawn as four 1.25-unit strokes.
    let reach = (7.5 / 2.0 - 1.25 / 2.0) * std::f32::consts::SQRT_2;
    let corners = [
        V2::new(c.x - reach, c.y),
        V2::new(c.x, c.y - reach),
        V2::new(c.x + reach, c.y),
        V2::new(c.x, c.y + reach),
    ];
    let colour = if high { WHITE } else { colour };
    let t = v.thick(1.25);
    for k in 0..4 {
        v.line(corners[k], corners[(k + 1) % 4], t, colour);
    }
    let dot = v.thick(2.0);
    v.rect(
        v.snap(c.x - dot / 2.0),
        v.snap(c.y - dot / 2.0),
        dot,
        dot,
        WHITE,
    );
}

/// What a broken brick leaves for a moment: its rim lifting off as a
/// ring that widens and fades.
fn broken(v: &Scene, r: Rect, glass: &Glass, age: f32) {
    let reduced = v.look.get().reduced;
    let k = Flash::BREAK.at(age, reduced);
    if k <= 0.0 {
        return;
    }
    // It widens over the first two thirds and fades over all of it.
    let p = (Flash::BREAK.progress(age, reduced) * 1.5).min(1.0);
    ring(v, r, p, opacity(glass.lit, 0.85 * k));
}

/// The ring a break or an ignition leaves: the brick grown by 6 × 5,
/// widening to 13 × 11 as `p` goes from 0 to 1.
fn ring(v: &Scene, r: Rect, p: f32, colour: Color) {
    let (x, y) = (3.0 + 3.5 * p, 2.5 + 3.0 * p);
    let t = v.thick(1.0);
    if v.style() == PieceStyle::Flat {
        v.outline(v.snap_rect(grow(r, x, y)), 0.0, t, colour);
    } else {
        v.outline(grow(r, x, y), BRICK_RADIUS + y, t, colour);
    }
}

/// A relay core once broken: its blast pending and then going off, and
/// its empty socket for a beat.
fn socket(v: &Scene, game: &Game, cell: FieldCell, r: Rect, age: f32) {
    let look = v.look.get();
    let flat = v.style() == PieceStyle::Flat;
    if age < SOCKET_SECONDS {
        if flat {
            v.rect(r.x, r.y, r.w, r.h, opacity(AMBER, 0.16));
            let i = inset(r, 1.0 / v.density);
            v.rect(i.x, i.y, i.w, i.h, SOCKET);
        } else {
            v.shape(r, [BRICK_RADIUS; 4], Fill::flat(SOCKET));
            v.outline(r, BRICK_RADIUS, v.thick(1.0), opacity(AMBER, 0.16));
        }
    }
    let pending = game.board().relay_countdown(cell) > 0;
    let flash = f32::from(game.effects().relay_flash[cell.index()]);
    let elapsed = (IGNITION_TICKS - flash) * DT;
    // Lit while its blast is pending, fading as it goes off.
    let k = if pending {
        if look.reduced { 0.4 } else { 1.0 }
    } else if flash > 0.0 {
        Flash::IGNITE.at(elapsed, look.reduced)
    } else {
        0.0
    };
    if k <= 0.0 {
        return;
    }
    cast(v, r, opacity(AMBER, 0.7 * k));
    let rim = Fill::ramp(opacity(WHITE, k), opacity(AMBER_LIGHT, k));
    if flat {
        v.outline(r, 0.0, v.thick(1.0), rim.at(0.5));
    } else {
        let t = v.thick(1.5);
        v.between(
            Path::new(r, [BRICK_RADIUS; 4]),
            Path::new(inset(r, t), [BRICK_RADIUS - t; 4]),
            rim,
            rim,
        );
    }
}

/// A relay blast going off: an amber ring as a break leaves, and sparks
/// running toward each neighbour it reaches.
fn ignitions(v: &Scene, game: &Game) {
    let reduced = v.look.get().reduced;
    for cell in FieldCell::all() {
        let flash = f32::from(game.effects().relay_flash[cell.index()]);
        if flash <= 0.0 {
            continue;
        }
        let elapsed = (IGNITION_TICKS - flash) * DT;
        let k = Flash::IGNITE.at(elapsed, reduced);
        if k <= 0.0 {
            continue;
        }
        let r = v.snap_rect(cell_box(cell));
        let p = (Flash::IGNITE.progress(elapsed, reduced) * 1.25).min(1.0);
        ring(v, r, p, opacity(AMBER, 0.85 * k));
        let c = r.center();
        let t = v.thick(1.0);
        for (dx, dy, edge) in [
            (1.0, 0.0, r.w / 2.0),
            (-1.0, 0.0, r.w / 2.0),
            (0.0, 1.0, r.h / 2.0),
            (0.0, -1.0, r.h / 2.0),
        ] {
            let from = V2::new(c.x + dx * edge, c.y + dy * edge);
            let to = V2::new(from.x + dx * 18.0, from.y + dy * 18.0);
            streak(
                v,
                from,
                to,
                t,
                (opacity(AMBER_LIGHT, k), opacity(AMBER, 0.0)),
            );
        }
    }
}

/// Bricks on a Compact screen: a one-pixel rim around flat glass, armour
/// as inner lines, a core as one lit pixel.
fn flat_brick(v: &Scene, b: &Brick) {
    let look = v.look.get();
    let px = 1.0 / v.density;
    let (r, glass) = (b.r, b.glass);
    let rim = mix(b.rim.at(0.45), WHITE, b.hit);
    v.rect(r.x, r.y, r.w, r.h, rim);
    let body = inset(r, px);
    let fill = if look.high { FIELD } else { glass.pane };
    v.rect(
        body.x,
        body.y,
        body.w,
        body.h,
        mix(fill, glass.hue, 0.4 * b.hit),
    );
    let c = r.center();
    if b.core {
        let colour = if look.high { WHITE } else { AMBER_LIGHT };
        v.rect(
            v.snap(c.x - px / 2.0),
            v.snap(c.y - px / 2.0),
            px,
            px,
            colour,
        );
    } else if b.hp > 1 {
        let w = v.snap(r.w * 34.0 / 58.0);
        let x = v.snap(c.x - w / 2.0);
        let lines = b.hp - 1;
        for k in 0..lines {
            let offset = (f32::from(k) * 2.0 - f32::from(lines - 1)) * px;
            v.rect(x, v.snap(c.y - px / 2.0) + offset, w, px, glass.rim2);
        }
    }
}

/// Sparks from hits and pickups: the simulation's particles, each a lit
/// shard of its brick's hue, gone within 180 ms. Two of every five slots
/// draw, so a direct hit's burst of ten throws four; reduced effects
/// draw half that.
fn shards(v: &Scene, fx: &Fx, game: &Game, palette: &Palette) {
    let look = v.look.get();
    let life = Flash::BREAK.length(look.reduced);
    let flat = v.style() == PieceStyle::Flat;
    let px = 1.0 / v.density;
    for (i, p) in game.effects().particles.iter().enumerate() {
        let age = fx.particle_age[i];
        let drawn = if look.reduced { i % 5 == 0 } else { i % 5 < 2 };
        if p.life <= 0.0 || age >= life || !drawn {
            continue;
        }
        let colour = opacity(palette.rows[p.hue % ROWS].lit, 1.0 - age / life);
        if flat {
            v.rect(v.snap(p.pos.x), v.snap(p.pos.y), px, px, colour);
            continue;
        }
        // 3.5 × 1.5, along its flight.
        let speed = p.velocity.length();
        let along = if speed > f32::EPSILON {
            p.velocity * (1.75 / speed)
        } else {
            V2::new(1.75, 0.0)
        };
        v.line(p.pos - along, p.pos + along, 1.5, colour);
    }
}

// Capsules.

fn capsule(v: &Scene, drop: &Capsule, glass: &Glass) {
    let r = v.snap_rect(Rect::new(drop.pos.x - 17.0, drop.pos.y - 9.0, 34.0, 18.0));
    if v.style() == PieceStyle::Glass {
        // A short wake of its own light above it, falling straight.
        let wake = Rect::new(drop.pos.x - 7.0, drop.pos.y - 28.0, 14.0, 28.0);
        v.ramp(wake, opacity(glass.hue, 0.0), opacity(glass.hue, 0.16));
    }
    glass_capsule(v, drop.power, r, glass, 0.55);
}

/// A capsule filling `r`: the pill of its power's glass and its
/// pictogram. Falling, and inline in text, where it is the same piece.
pub(super) fn capsule_at(v: &Scene, power: Power, r: Rect) {
    let glass = Glass::new(power_color(power), (30.0, 12.0), v.look.get().high);
    glass_capsule(v, power, r, &glass, 0.0);
}

fn glass_capsule(v: &Scene, power: Power, r: Rect, glass: &Glass, glow: f32) {
    let look = v.look.get();
    if v.style() == PieceStyle::Flat {
        // Hue alone, with a lit centre: the one place shape gives way.
        let px = 1.0 / v.density;
        v.rect(r.x, r.y, r.w, r.h, glass.hue);
        let body = inset(r, px);
        let fill = if look.high { FIELD } else { glass.pane };
        v.rect(body.x, body.y, body.w, body.h, fill);
        let c = r.center();
        v.rect(
            v.snap(c.x - px / 2.0),
            v.snap(c.y - px / 2.0),
            px,
            px,
            WHITE,
        );
        return;
    }
    cast(v, r, opacity(glass.hue, glow));
    let radius = r.h / 2.0;
    v.shape(r, [radius; 4], glass.rim);
    let t = v.thick(if look.high { 2.0 } else { 1.5 }) * r.h / 18.0;
    let t = t.max(1.0 / v.density);
    v.shape(inset(r, t), [radius - t; 4], glass.body);
    pictogram(v, power, r, glass.mark);
}

/// `power`'s mark in the middle of capsule `r`, on whole pixels.
fn pictogram(v: &Scene, power: Power, r: Rect, colour: Color) {
    let d = v.density;
    let h = pictogram::height(power) * r.h / 18.0 * d;
    let cell = v.atlas.icon(power, h);
    let c = r.center();
    let at = (
        (c.x * d - f32::from(cell.w) / 2.0).round(),
        (c.y * d - f32::from(cell.h) / 2.0).round(),
    );
    v.sprite(
        at,
        (
            f32::from(cell.x),
            f32::from(cell.y),
            f32::from(cell.w),
            f32::from(cell.h),
        ),
        1.0,
        colour,
    );
}

// The paddle.

fn paddle(v: &Scene, fx: &Fx, game: &Game, palette: &Palette) {
    let look = v.look.get();
    let paddle = game.paddle();
    let w = paddle.width;
    let body = v.snap_rect(Rect::new(paddle.x - w / 2.0, PADDLE_Y, w, PADDLE_HEIGHT));
    let contact = Flash::CONTACT.at(fx.contact_age, look.reduced);
    let caught = Flash::CATCH.at(fx.catch.1, look.reduced);
    let full = if look.reduced { 0.4 } else { 1.0 };
    // A catch pours the capsule's hue into the keel; a contact or a catch
    // flares it toward the paddle's ends.
    let hue = mix(CYAN, palette.power(fx.catch.0).hue, caught / full);
    let flare = contact.max(caught) / full;
    let flat = v.style() == PieceStyle::Flat;
    let px = 1.0 / v.density;
    let inset_by = if flat { 0.13 } else { 0.14 - 0.08 * flare };
    let keel_t = if flat {
        px
    } else {
        v.thick(if look.high { 1.5 } else { 1.0 })
    };
    let keel_y = v.snap(body.y + body.h + if flat { 2.0 * px } else { 4.5 });
    let keel = v.snap_rect(Rect::new(
        body.x + w * inset_by,
        keel_y,
        w * (1.0 - 2.0 * inset_by),
        keel_t,
    ));
    let keel = Rect::new(keel.x, keel_y, keel.w, keel_t);
    if !flat {
        cast(v, body, opacity(hue, 0.55 + 0.35 * flare));
        v.glow(grow(keel, GLOW_GROW, 5.0), opacity(hue, 0.85));
    }
    v.rect(keel.x, keel.y, keel.w, keel.h, hue);
    drains(v, fx, game, keel, palette);
    if flat {
        let colour = mix(FLAT_PEARL, WHITE, flare);
        v.rect(body.x, body.y, body.w, body.h, colour);
    } else {
        v.shape(
            body,
            [PADDLE_HEIGHT / 2.0; 4],
            blend(PEARL, PEARL_LIT, contact / full),
        );
    }
    seam(v, game, body);
    // A held ball rests on the pearl over a soft contact shadow, so the
    // two solid pieces never merge.
    if !flat {
        for ball in game.balls().iter().filter(|b| b.active && b.held) {
            v.rect(ball.pos.x - 6.0, body.y - 0.5, 12.0, 2.0, CONTACT_SHADOW);
        }
    }
}

/// The seam at the paddle's middle, where the ball leaves straight up;
/// with Anchor, its charges as pips, used ones pearl.
fn seam(v: &Scene, game: &Game, body: Rect) {
    let flat = v.style() == PieceStyle::Flat;
    let px = 1.0 / v.density;
    let c = body.center();
    let charges = game.powers().anchor_charges;
    let anchor = charges > 0 || game.balls().iter().any(|b| b.active && b.held);
    if !anchor {
        let (w, h) = if flat { (3.0 * px, px) } else { (13.0, 2.5) };
        let r = v.snap_rect(Rect::new(c.x - w / 2.0, c.y - h / 2.0, w, h));
        if flat {
            v.rect(r.x, r.y, r.w, r.h, CYAN);
        } else {
            v.shape(r, [h / 2.0; 4], SEAM);
        }
        return;
    }
    let (w, h, gap) = if flat { (px, px, px) } else { (3.5, 2.5, 2.5) };
    let total = f32::from(ANCHOR_CHARGES) * (w + gap) - gap;
    for i in 0..ANCHOR_CHARGES {
        // Used from the left, so what is left reads as a count.
        let lit = i >= ANCHOR_CHARGES - charges;
        let x = c.x - total / 2.0 + f32::from(i) * (w + gap);
        let r = v.snap_rect(Rect::new(x, c.y - h / 2.0, w, h));
        if flat {
            v.rect(r.x, r.y, r.w, r.h, if lit { CYAN } else { SEAM_OUT });
        } else {
            let fill = if lit { SEAM } else { Fill::flat(SEAM_OUT) };
            v.shape(r, [h / 2.0; 4], fill);
        }
    }
}

/// Wide and Slow as drains under the keel: centred lines that close in
/// from both ends, oldest first, pulsing through their last two seconds.
fn drains(v: &Scene, fx: &Fx, game: &Game, keel: Rect, palette: &Palette) {
    let look = v.look.get();
    let flat = v.style() == PieceStyle::Flat;
    let px = 1.0 / v.density;
    let powers = game.powers();
    let mut timed = [
        (
            powers.wide_seconds,
            WIDE_SECONDS,
            Power::Wide,
            fx.granted[0],
        ),
        (
            powers.slow_seconds,
            SLOW_SECONDS,
            Power::Slow,
            fx.granted[1],
        ),
    ];
    if timed[1].3 < timed[0].3 {
        timed.swap(0, 1);
    }
    let full = game.paddle().width - 24.0;
    let t = if flat { px } else { v.thick(1.0) };
    let pitch = if flat {
        2.0 * px
    } else {
        (2.0_f32).max(t + px)
    };
    let mut y = v.snap(keel.y + keel.h + if flat { px } else { 2.5 });
    let centre = game.paddle().x;
    for (left, total, power, _) in timed {
        if left <= 0.0 {
            continue;
        }
        let length = status::drain(left, total, full);
        let a = status::pulse(left, look.reduced);
        let hue = if look.reduced && status::running_out(left) {
            WHITE
        } else {
            palette.power(power).hue
        };
        let r = v.snap_rect(Rect::new(centre - length / 2.0, y, length, t));
        let r = Rect::new(r.x, y, r.w.max(px), t);
        v.glow(grow(r, GLOW_GROW, 4.0), opacity(hue, 0.5 * a));
        v.rect(r.x, r.y, r.w, r.h, opacity(hue, a));
        y = v.snap(y + pitch);
    }
}

// Balls.

fn balls(v: &Scene, fx: &Fx, game: &Game, alpha: f32) {
    let look = v.look.get();
    let flat = v.style() == PieceStyle::Flat;
    for (i, ball) in game.balls().iter().enumerate() {
        if !ball.active {
            continue;
        }
        if ball.held {
            aim(v, ball);
        }
        let pos = if ball.held {
            ball.pos
        } else {
            ball.previous.lerp(ball.pos, alpha)
        };
        // Sinking into the open edge, the ball dims.
        let sink = ((pos.y - (BOTTOM - DRAIN_FADE)) / DRAIN_FADE).clamp(0.0, 1.0);
        let light = 1.0 - 0.65 * sink;
        if flat {
            let px = 1.0 / v.density;
            let (x, y) = (v.snap(pos.x - px), v.snap(pos.y - px));
            v.rect(x, y, 2.0 * px, 2.0 * px, opacity(WHITE, light));
            continue;
        }
        let phase = ball.phase_charges > 0;
        let tint = if phase { INDIGO } else { TRAIL };
        let len = if ball.held { 0 } else { fx.trail_len[i] };
        for n in (1..=len).rev() {
            let index = (fx.cursor + 12 - n) % 12;
            let k = n as f32;
            let colour = opacity(tint, 0.2 * (1.0 - k / 12.0) * light);
            v.disc(fx.trails[i][index], RADIUS * (1.0 - k / 15.0), 12, colour);
        }
        let halo = if phase { INDIGO } else { BALL_LIGHT };
        let reach = RADIUS + 8.0;
        v.glow(
            Rect::new(pos.x - reach, pos.y - reach, 2.0 * reach, 2.0 * reach),
            opacity(halo, 0.35 * light),
        );
        if look.high {
            v.ring(pos, RADIUS, v.thick(1.0), NIGHT);
        }
        v.pearl(pos, RADIUS, light);
        if phase {
            phase_pips(v, pos, ball.phase_charges);
        }
    }
}

/// Phase charges as three pips riding under the ball, one going out per
/// brick it passes through.
fn phase_pips(v: &Scene, pos: V2, charges: u8) {
    let (w, h, pitch) = (3.5, 1.5, 4.5);
    let total = f32::from(PHASE_CONTACTS - 1) * pitch + w;
    for k in 0..PHASE_CONTACTS {
        let x = pos.x - total / 2.0 + f32::from(k) * pitch;
        let r = v.snap_rect(Rect::new(x, pos.y + 12.0, w, h));
        let fill = if k < charges {
            PHASE_LIT
        } else {
            Fill::flat(mix(FIELD, INDIGO, 0.22))
        };
        v.shape(r, [h / 2.0; 4], fill);
    }
}

/// A held ball's launch line: the only dotted thing in the game.
fn aim(v: &Scene, ball: &Ball) {
    let mut point = ball.pos;
    let mut direction = ball.velocity.normalized();
    let flat = v.style() == PieceStyle::Flat;
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
        let colour = opacity(CYAN, 0.6 - n as f32 * 0.04);
        if flat {
            let px = 1.0 / v.density;
            v.rect(v.snap(point.x), v.snap(point.y), px, px, colour);
        } else {
            v.disc(point, 1.5, 8, colour);
        }
    }
}

/// When the last ball is lost, a red hairline sweeps once along the open
/// edge from where it fell: the one place red appears.
fn drain_line(v: &Scene, fx: &Fx) {
    let (x, age) = fx.drain;
    if age >= DRAIN_LINE {
        return;
    }
    let p = age / DRAIN_LINE;
    let reach = 40.0 + 260.0 * p;
    let colour = opacity(RED, 0.7 * (1.0 - p));
    let clear = opacity(RED, 0.0);
    let y = v.snap(BOTTOM) - v.thick(1.0);
    let t = v.thick(1.0);
    for end in [x - reach, x + reach] {
        // Clipped to the field, the fade keeps its slope.
        let edge = end.clamp(LEFT, RIGHT);
        let left = mix(colour, clear, (edge - x).abs() / reach);
        let (a, b) = (V2::new(x, y + t / 2.0), V2::new(edge, y + t / 2.0));
        streak(v, a, b, t, (colour, left));
    }
}

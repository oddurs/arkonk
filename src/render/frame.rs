//! The instrument: one glass arch around the field. A band across the top
//! carries the status of whatever screen is showing, the rails down the
//! sides are the walls, and the open bottom fades into the night where a
//! ball drains. Nothing is drawn outside the arch.
use super::*;

/// The arch's outer edge. Its last fifth fades to night, as the design's
/// mask does, so the one open side is the one a ball can leave by.
const ARCH: Rect = Rect {
    x: 40.0,
    y: 40.0,
    w: 880.0,
    h: 842.0,
};
const ARCH_RADIUS: f32 = 24.0;
const FADE: f32 = ARCH.y + 0.8 * ARCH.h;
/// The band: the arch above the field, between the rails.
pub(super) const BAND: Rect = Rect {
    x: LEFT,
    y: ARCH.y,
    w: RIGHT - LEFT,
    h: TOP - ARCH.y,
};
/// Inside the band's ends.
pub(super) const BAND_PAD: f32 = S16;
const FIELD_RADIUS: f32 = 10.0;
/// The last stretch of the field, fading to night.
const DRAIN: f32 = 30.0;

const ARCH_TOP: Color = hex(0x141824);
const ARCH_MID: Color = hex(0x10131c);
const ARCH_LOW: Color = hex(0x0e1119);
const ARCH_EDGE: Color = hex(0x1b2030);
pub(super) const FIELD_GLASS: Color = hex(0x0c0e16);

/// The arch, the field inside it, and the walls: everything under the
/// pieces. `flash` lights the walls after a bounce.
pub(super) fn arch(v: &Scene, flash: f32) {
    let left = v.snap(ARCH.x);
    let right = v.snap(ARCH.x + ARCH.w);
    let (top, fade, foot) = (v.snap(ARCH.y), v.snap(FADE), v.snap(ARCH.y + ARCH.h));
    let w = right - left;
    // The glass darkens from its top over 100 units, then slowly to the
    // foot, where it has faded into the night.
    let lit = top + 100.0;
    let at_fade = mix(ARCH_MID, ARCH_LOW, (fade - lit) / (foot - lit));
    let r = ARCH_RADIUS;
    v.shape(
        Rect::new(left, top, w, lit - top),
        [r, r, 0.0, 0.0],
        Fill::ramp(ARCH_TOP, ARCH_MID),
    );
    v.ramp(Rect::new(left, lit, w, fade - lit), ARCH_MID, at_fade);
    v.ramp(Rect::new(left, fade, w, foot - fade), at_fade, NIGHT);
    let hair = v.thick(1.0);
    top_edge(
        v,
        Rect::new(left, top, w, fade - top),
        (r, hair),
        ARCH_EDGE,
        ARCH_EDGE,
    );
    for x in [left, right - hair] {
        v.ramp(Rect::new(x, fade, hair, foot - fade), ARCH_EDGE, NIGHT);
    }
    // Light from above catches the top edge.
    v.rect(left + r, top, w - 2.0 * r, hair, opacity(WHITE, 0.08));

    let field = v.snap_rect(Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
    let fr = FIELD_RADIUS;
    v.shape(field, [fr, fr, 0.0, 0.0], Fill::flat(FIELD_GLASS));
    // The walls are the field's own edge: cyan hairlines, brightest on
    // the ceiling, and none at the bottom.
    let lift = (flash * 5.0).min(0.6);
    let wall = |alpha: f32| opacity(CYAN, alpha + (1.0 - alpha) * lift);
    top_edge(v, field, (fr, hair), wall(0.20), wall(0.13));
    let drain = v.snap(BOTTOM - DRAIN);
    v.ramp(
        Rect::new(field.x, drain, field.w, field.y + field.h - drain),
        opacity(NIGHT, 0.0),
        NIGHT,
    );
}

/// The top and sides of `r`, its top corners rounded, as a line `t`
/// thick drawn inside it; open at the bottom. The top takes `top`, the
/// sides `sides`, and the corners blend between them.
fn top_edge(v: &Scene, r: Rect, (radius, t): (f32, f32), top: Color, sides: Color) {
    v.rect(r.x + radius, r.y, r.w - 2.0 * radius, t, top);
    for x in [r.x, r.x + r.w - t] {
        v.rect(x, r.y + radius, t, r.h - radius, sides);
    }
    const STEPS: usize = Path::STEPS;
    for (cx, start) in [(r.x + radius, 2.0), (r.x + r.w - radius, 3.0)] {
        let centre = vec2(cx, r.y + radius);
        let zero = v.vertex(centre, top);
        let mut vertices = [zero; (STEPS + 1) * 2];
        let mut indices = [0_u16; STEPS * 6];
        for i in 0..=STEPS {
            let along = i as f32 / STEPS as f32;
            // The left corner runs from the side up to the top, the right
            // one from the top down to the side.
            let toward_top = if start == 2.0 { along } else { 1.0 - along };
            let colour = mix(sides, top, toward_top);
            let direction = Vec2::from_angle((start + along) * FRAC_PI_2);
            vertices[i * 2] = v.vertex(centre + direction * radius, colour);
            vertices[i * 2 + 1] = v.vertex(centre + direction * (radius - t), colour);
            if i < STEPS {
                let k = (i * 2) as u16;
                indices[i * 6..i * 6 + 6].copy_from_slice(&[k, k + 1, k + 2, k + 2, k + 1, k + 3]);
            }
        }
        v.mesh(&vertices, &indices);
    }
}

/// Where a line of `size`-unit text sits when set in a box `line` times
/// its size tall whose top is `top`: Noto's ascender and descender
/// (1.069 and 0.293 em) centred in the box, as a browser sets it.
pub(super) fn baseline(top: f32, size: f32, line: f32) -> f32 {
    top + size * (line / 2.0 + (1.069 - 0.293) / 2.0)
}

/// The journey as twelve pips: cleared ones pearl, this one lit cyan, the
/// rest outlined.
pub(super) fn pips(v: &Scene, centre: f32, top: f32, current: Option<SectorId>, profile: &Profile) {
    const W: f32 = 14.0;
    const GAP: f32 = 4.0;
    const H: f32 = 3.0;
    let span = SECTOR_COUNT as f32 * (W + GAP) - GAP;
    let left = centre - span / 2.0;
    for id in SectorId::all() {
        let r = v.snap_rect(Rect::new(left + id.index() as f32 * (W + GAP), top, W, H));
        let radii = [2.0; 4];
        if Some(id) == current {
            v.halo(r, 2.0, 4.0, opacity(CYAN, 0.45));
            v.shape(r, radii, Fill::ramp(hex(0xb8f3fc), CYAN));
        } else if profile.progress.record(id).medals != Medals::NONE {
            v.shape(
                r,
                radii,
                Fill::ramp(opacity(WHITE, 0.55), opacity(hex(0xa9b4c6), 0.55)),
            );
        } else {
            v.outline(r, 2.0, v.thick(1.0), hex(0x2a3142));
        }
    }
}

/// The band in play: the score, the sector and the journey, and the lives.
pub(super) fn band_play(v: &Scene, fx: &Fx, game: &Game, profile: &Profile) {
    let (left, right) = (BAND.x + BAND_PAD, BAND.x + BAND.w - BAND_PAD);
    let mid = BAND.y + BAND.h / 2.0;
    let score = Figures::count(v.locale, game.score());
    let figure = Role::Figure;
    let size = spec::style(figure).0;
    let side = 280.0;
    v.put(
        score.as_str(),
        figure,
        Slot::left(left, side, v.snap(baseline(mid - size / 2.0, size, 1.0))),
        INK,
    );
    // The sector's name over the journey: a 20-unit line, 10 apart, then
    // three-unit pips, centred as one block.
    let block = 20.0 + 10.0 + 3.0;
    let top = mid - block / 2.0;
    let name = Slot::centered(
        WIDTH / 2.0,
        right - left - 2.0 * side,
        v.snap(baseline(top, 20.0, 1.0)),
    );
    v.say(
        TextId::SectorName(game.sector()),
        &[],
        Role::Body,
        name,
        INK,
    );
    pips(v, WIDTH / 2.0, top + 30.0, Some(game.sector()), profile);
    lives(v, right, mid, game.lives(), fx.entry_lives);
}

/// Lives as pearls ending at `right`, and a dim ring for each life lost
/// since the sector began.
fn lives(v: &Scene, right: f32, mid: f32, lives: u8, entry: u8) {
    const D: f32 = 12.0;
    const GAP: f32 = 9.0;
    let lost = entry.saturating_sub(lives);
    let count = lives + lost;
    for i in 0..count {
        let x = right - D / 2.0 - f32::from(count - 1 - i) * (D + GAP);
        let p = V2::new(v.snap(x), v.snap(mid));
        if i < lives {
            v.halo(
                Rect::new(p.x - D / 2.0, p.y - D / 2.0, D, D),
                D / 2.0,
                4.0,
                opacity(hex(0xdcf0ff), 0.3),
            );
            v.pearl(p, D / 2.0, 1.0);
        } else {
            let t = v.thick(1.5);
            v.ring(p, D / 2.0 - t, t, hex(0x3a4256));
        }
    }
}

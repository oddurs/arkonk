//! The instrument: one glass arch around the field. A band across the top
//! carries the status of whatever screen is showing, the rails down the
//! sides are the walls, and the open bottom fades into the night where a
//! ball drains. Nothing is drawn outside the arch.
use super::*;
use crate::ui::Prompt;

/// The arch's outer edge on a Regular screen. Its last fifth fades to
/// night, as the design's mask does, so the one open side is the one a
/// ball can leave by.
const ARCH: Rect = Rect {
    x: 40.0,
    y: 40.0,
    w: 880.0,
    h: 842.0,
};
const ARCH_RADIUS: f32 = 24.0;
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

/// The arch's outer edge and the band, as the screen's class shapes them:
/// a Small screen has a 72-unit band and 12-unit rails, a Compact one an
/// 8-pixel strip and one-pixel rails. The field never moves. Where the
/// platform reports overscan, the band stays inside the safe area.
pub(super) fn parts(v: &Scene) -> (Rect, Rect) {
    let field = Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP);
    let (arch, band) = match v.class {
        Class::Regular => (ARCH, Rect::new(LEFT, ARCH.y, field.w, TOP - ARCH.y)),
        Class::Small => {
            let (rail, band) = (12.0, 72.0);
            let top = TOP - band;
            let arch = Rect::new(
                LEFT - rail,
                top,
                field.w + 2.0 * rail,
                ARCH.y + ARCH.h - top,
            );
            (arch, Rect::new(LEFT, top, field.w, band))
        }
        Class::Compact => {
            let px = 1.0 / v.density;
            let strip = view::STRIP * px;
            let band = Rect::new(LEFT, TOP - strip - px, field.w, strip);
            let arch = Rect::new(LEFT - px, band.y, field.w + 2.0 * px, BOTTOM - band.y);
            (arch, band)
        }
    };
    let top = band.y.max(v.safe.y);
    (arch, Rect::new(band.x, top, band.w, band.bottom() - top))
}

/// The band's ends, inside its padding and the safe area, and its middle
/// line. Band text keeps inside the band.
fn band_frame(v: &Scene) -> (f32, f32, f32) {
    let band = parts(v).1;
    v.region(0, band);
    let left = (band.x + BAND_PAD).max(v.safe.x);
    let right = (band.x + band.w - BAND_PAD).min(v.safe.x + v.safe.w);
    (left, right, band.y + band.h / 2.0)
}

/// Field text keeps inside the field.
pub(super) fn field_region(v: &Scene, layer: u8) {
    v.region(layer, Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
}

/// The arch, the field inside it, and the walls: everything under the
/// pieces. `flash` lights the walls after a bounce.
pub(super) fn arch(v: &Scene, flash: f32) {
    let field = v.snap_rect(Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
    let hair = v.thick(1.0);
    let lift = (flash * 5.0).min(0.6);
    let wall = |alpha: f32| opacity(CYAN, alpha + (1.0 - alpha) * lift);
    if v.class == Class::Compact {
        // One-pixel rails, flat glass: nothing a few pixels cannot hold.
        let (arch, _) = parts(v);
        v.rect(field.x, field.y, field.w, field.h, FIELD_GLASS);
        let side = v.snap(arch.x);
        for x in [side, v.snap(arch.x + arch.w) - hair] {
            v.rect(x, field.y, hair, field.h, hex(0x1e2a35));
        }
        v.rect(field.x, field.y, field.w, hair, wall(0.45));
        return;
    }
    let (frame, _) = parts(v);
    let left = v.snap(frame.x);
    let right = v.snap(frame.x + frame.w);
    let foot = v.snap(frame.y + frame.h);
    let top = v.snap(frame.y);
    let fade = v.snap(top + 0.8 * (foot - top));
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

    let fr = FIELD_RADIUS;
    v.shape(field, [fr, fr, 0.0, 0.0], Fill::flat(FIELD_GLASS));
    // The walls are the field's own edge: cyan hairlines, brightest on
    // the ceiling, and none at the bottom.
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

/// The journey as twelve pips centred on `centre` from `top`: cleared
/// ones pearl, this one lit cyan, the rest outlined. Returns their span.
pub(super) fn pips(
    v: &Scene,
    centre: f32,
    top: f32,
    current: Option<SectorId>,
    profile: &Profile,
) -> f32 {
    let span = pips_span(v);
    // A Compact strip sets them in whole pixels: three wide, one tall.
    let px = 1.0 / v.density;
    let (w, gap, h) = if v.class == Class::Compact {
        (3.0 * px, px, px)
    } else {
        (14.0, 4.0, 3.0)
    };
    let left = v.snap(centre - span / 2.0);
    for id in SectorId::all() {
        let r = v.snap_rect(Rect::new(left + id.index() as f32 * (w + gap), top, w, h));
        if v.class == Class::Compact {
            let lit = profile.progress.record(id).medals != Medals::NONE;
            let colour = match (Some(id) == current, lit) {
                (true, _) => CYAN,
                (false, true) => hex(0xa9b4c6),
                (false, false) => hex(0x2a3142),
            };
            v.rect(r.x, r.y, r.w, r.h, colour);
            continue;
        }
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
    span
}
fn pips_span(v: &Scene) -> f32 {
    let (w, gap) = if v.class == Class::Compact {
        (3.0 / v.density, 1.0 / v.density)
    } else {
        (14.0, 4.0)
    };
    SECTOR_COUNT as f32 * (w + gap) - gap
}
/// The pips in a band's middle, between what its ends already hold, when
/// they fit there with room to spare: a crowded Compact strip drops them
/// before anything overlaps.
fn middle_pips(
    v: &Scene,
    (left, right, mid): (f32, f32, f32),
    current: Option<SectorId>,
    profile: &Profile,
) {
    let half = pips_span(v) / 2.0 + v.at_least(S8, 2.0);
    if WIDTH / 2.0 - half >= left && WIDTH / 2.0 + half <= right {
        let h = if v.class == Class::Compact {
            1.0 / v.density
        } else {
            3.0
        };
        pips(v, WIDTH / 2.0, v.snap(mid - h / 2.0), current, profile);
    }
}

/// News takes a Compact strip whole while it lasts; there is no room
/// beside it. Returns whether it did.
/// Words the pixel font cannot spell would stand taller than the strip,
/// so those languages see the warning sign alone.
fn compact_news(v: &Scene, (left, right, mid): (f32, f32, f32), notice: bool) -> bool {
    if !(notice && v.class == Class::Compact) {
        return false;
    }
    let style = Style::from(Role::Caption);
    let fits = |t: &str| t.chars().all(pixel_font::spells) && v.measure(t, style) <= right - left;
    let form = [Form::Full, Form::Short]
        .into_iter()
        .find(|&form| v.format(TextId::SaveFailed, &[], form, fits));
    if let Some(form) = form {
        let at = v.snap(mid + v.cap(style) / 2.0);
        let slot = Slot::centered((left + right) / 2.0, right - left, at);
        v.format(TextId::SaveFailed, &[], form, |t| {
            v.put(t, style, slot, INK)
        });
    } else {
        let px = 1.0 / v.density;
        let at = (v.snap(WIDTH / 2.0 - 2.5 * px), v.snap(mid - 3.5 * px));
        v.bits(WARNING, at, px, INK);
    }
    true
}
/// A warning sign in the 5×7 grid: a triangle round an exclamation mark.
const WARNING: [u8; 7] = [4, 10, 10, 21, 17, 21, 31];

/// The widest line centred in the band that stays clear of what its ends
/// hold, from `left` to `right`, by a gap either side.
fn between(left: f32, right: f32) -> f32 {
    2.0 * ((WIDTH / 2.0 - left).min(right - WIDTH / 2.0) - S16)
}

/// The band in play: the score, the sector and the journey, and the lives.
/// A Small band drops the name and keeps the pips; a Compact strip sets
/// all three on one pixel line.
pub(super) fn band_play(v: &Scene, fx: &Fx, game: &Game, profile: &Profile, notice: bool) {
    let (left, right, mid) = band_frame(v);
    if compact_news(v, (left, right, mid), notice) {
        return;
    }
    let score = Figures::count(v.locale, game.score());
    let figure = v.snap(mid + v.cap(Role::Figure) / 2.0);
    let score_w = v.measure(score.as_str(), Role::Figure);
    v.put(
        score.as_str(),
        Role::Figure,
        Slot::left(left, score_w, figure),
        INK,
    );
    let lives_w = lives(
        v,
        right,
        mid,
        game.lives(),
        (fx.entry_lives, fx.life_gained),
    );
    if notice {
        moments::status(
            v,
            mid,
            TextId::SaveFailed,
            between(left + score_w, right - lives_w),
        );
    } else if v.class == Class::Regular {
        // The sector's name over the journey: the name's line, 10 apart,
        // then three-unit pips, centred as one block.
        let strong = Style::from(Role::Body).strong();
        let name_h = v.line_h(Role::Body) / spec::line(Role::Body);
        let top = mid - (name_h + 10.0 + 3.0) / 2.0;
        let at = v.snap(top + name_h * (0.5 + 0.388));
        let side = 280.0;
        let name = Slot::centered(WIDTH / 2.0, right - left - 2.0 * side, at);
        v.say(TextId::SectorName(game.sector()), &[], strong, name, INK);
        pips(
            v,
            WIDTH / 2.0,
            v.snap(top + name_h + 10.0),
            Some(game.sector()),
            profile,
        );
    } else {
        let ends = (left + score_w, right - lives_w, mid);
        middle_pips(v, ends, Some(game.sector()), profile);
    }
}

/// Lives as pearls ending at `right`, and a dim ring for each life lost
/// since the sector began; returns the row's width. A life just gained
/// wears an amber ring for a second, with "+1 life" under the row, or
/// beside it where a Small band has no room below. A Compact strip shows
/// 2 × 2 dots.
fn lives(v: &Scene, right: f32, mid: f32, lives: u8, (entry, gained): (u8, f32)) -> f32 {
    let lost = entry.saturating_sub(lives);
    let count = lives + lost;
    if v.class == Class::Compact {
        let px = 1.0 / v.density;
        for i in 0..count {
            let x = v.snap(right - (2.0 + f32::from(count - 1 - i) * 4.0) * px);
            let colour = if i < lives { INK } else { hex(0x3a4256) };
            v.rect(x, v.snap(mid - px), 2.0 * px, 2.0 * px, colour);
        }
        return (f32::from(count) * 4.0 - 2.0).max(0.0) * px;
    }
    const D: f32 = 12.0;
    const GAP: f32 = 9.0;
    for i in 0..count {
        let x = right - D / 2.0 - f32::from(count - 1 - i) * (D + GAP);
        let p = V2::new(v.snap(x), v.snap(mid));
        let bounds = Rect::new(p.x - D / 2.0, p.y - D / 2.0, D, D);
        if i < lives {
            if gained > 0.0 && i + 1 == lives {
                v.halo(bounds, D / 2.0, 12.0, opacity(AMBER, 0.6 * gained));
                v.ring(p, D / 2.0, 4.0, opacity(AMBER, 0.25 * gained));
            }
            v.halo(bounds, D / 2.0, 4.0, opacity(hex(0xdcf0ff), 0.3));
            v.pearl(p, D / 2.0, 1.0);
        } else {
            let t = v.thick(1.5);
            v.ring(p, D / 2.0 - t, t, hex(0x3a4256));
        }
    }
    let row = (f32::from(count) * (D + GAP) - GAP).max(0.0);
    if gained > 0.0 {
        let caption = Style::from(Role::Caption).sized(13.0);
        let colour = opacity(AMBER, gained.min(0.5) * 2.0);
        let slot = if v.class == Class::Regular {
            Slot::right(right, 160.0, v.snap(mid + D / 2.0 + 8.0 + v.cap(caption)))
        } else {
            let at = v.snap(mid + v.cap(caption) / 2.0);
            Slot::right(right - row - S12, 160.0, at)
        };
        v.say(TextId::LifeGained, &[], caption, slot, colour);
        if v.class != Class::Regular {
            let w = v.width_of(TextId::LifeGained, &[], caption).min(160.0);
            return row + S12 + w;
        }
    }
    row
}

/// A count out of a total as a figure, the total muted: `6 / 36`, ending
/// at `right` on `baseline`; returns its width.
fn out_of(
    v: &Scene,
    (count, total): (u32, u32),
    style: Style,
    (right, baseline): (f32, f32),
) -> f32 {
    let total = Figures::of(|f| {
        f.write_str(" / ")?;
        ark_text::grouped(f, v.locale, total)
    });
    let count = Figures::count(v.locale, count);
    let tail = v.measure(total.as_str(), style);
    v.put(
        total.as_str(),
        style,
        Slot::right(right, tail, baseline),
        MUTED,
    );
    let head = v.measure(count.as_str(), style);
    v.put(
        count.as_str(),
        style,
        Slot::right(right - tail, head, baseline),
        INK,
    );
    head + tail
}

/// The band on the title: the best score, the journey (the saved sector
/// lit), and the medals. A Compact strip drops the labels and the count
/// of open sectors.
pub(super) fn band_title(v: &Scene, profile: &Profile, notice: bool) {
    let (left, right, mid) = band_frame(v);
    if compact_news(v, (left, right, mid), notice) {
        return;
    }
    let medals = (profile.progress.medal_count(), 3 * SECTOR_COUNT as u32);
    let best = Figures::count(v.locale, profile.progress.best_score());
    let here = profile.progress.checkpoint().map(|c| c.sector);
    if v.class == Class::Compact {
        let style = Style::from(Role::Figure);
        let at = v.snap(mid + v.cap(style) / 2.0);
        let best_w = v.measure(best.as_str(), style);
        v.put(best.as_str(), style, Slot::left(left, best_w, at), INK);
        let medals_w = out_of(v, medals, style, (right, at));
        middle_pips(v, (left + best_w, right - medals_w, mid), here, profile);
        return;
    }
    let style = Style::from(Role::Figure).sized(28.0);
    // A Label over a figure, 8 apart, centred in the band.
    let (label_h, figure_h) = (v.line_h(Role::Label), v.line_h(style));
    let top = mid - (label_h + 8.0 + figure_h) / 2.0;
    let label = v.baseline(Role::Label, top);
    let figure = v.baseline(style, top + label_h + 8.0);
    let side = 270.0;
    v.say(
        TextId::StatBest,
        &[],
        Role::Label,
        Slot::left(left, side, label),
        DIM,
    );
    v.put(best.as_str(), style, Slot::left(left, side, figure), INK);
    v.say(
        TextId::StatMedals,
        &[],
        Role::Label,
        Slot::right(right, side, label),
        DIM,
    );
    let medals_w = out_of(v, medals, style, (right, figure));
    if notice {
        let best_w =
            v.measure(best.as_str(), style)
                .max(v.width_of(TextId::StatBest, &[], Role::Label));
        let medals_w = medals_w.max(v.width_of(TextId::StatMedals, &[], Role::Label));
        let room = between(left + best_w.min(side), right - medals_w.min(side));
        moments::status(v, mid, TextId::SaveFailed, room);
        return;
    }
    // The journey: twelve pips over how many sectors are open.
    let caption = Style::from(Role::Caption).sized(15.0);
    let top = mid - (3.0 + 10.0 + v.line_h(caption)) / 2.0;
    pips(v, WIDTH / 2.0, v.snap(top), here, profile);
    let open = profile.progress.unlocked_count() as u32;
    let args = [Arg::Count(open), Arg::Count(SECTOR_COUNT as u32)];
    let at = v.baseline(caption, top + 13.0);
    v.say(
        TextId::SectorsOf,
        &args,
        caption,
        Slot::centered(WIDTH / 2.0, 260.0, at),
        DIM,
    );
}

/// The band on sector select: the way back, where you are, the medals.
/// A Compact strip names the screen in its one line of capitals.
pub(super) fn band_sectors(v: &Scene, profile: &Profile, notice: bool) {
    let (left, right, mid) = band_frame(v);
    if compact_news(v, (left, right, mid), notice) {
        return;
    }
    let size = chips::size(v, false);
    let chip = chips::chip(v, Prompt::Back, (left, mid), size, chips::Lit::Neutral);
    let text = left + chip + S12;
    let at = v.snap(mid + v.cap(Role::Caption) / 2.0);
    let compact = v.class == Class::Compact;
    let back = if compact {
        0.0
    } else {
        let back = v
            .width_of(TextId::ActionBack, &[], Role::Caption)
            .min(200.0);
        v.say(
            TextId::ActionBack,
            &[],
            Role::Caption,
            Slot::left(text, 200.0, at),
            DIM,
        );
        S12 + back
    };
    let reach = v.at_least(40.0, 32.0);
    v.hits.borrow_mut().back = Some(Rect::new(left, mid - reach / 2.0, chip + back, reach));
    let style = if compact {
        Style::from(Role::Figure)
    } else {
        Style::from(Role::Figure).sized(24.0)
    };
    let figure = v.snap(mid + v.cap(style) / 2.0);
    let medals = (profile.progress.medal_count(), 3 * SECTOR_COUNT as u32);
    let w = out_of(v, medals, style, (right, figure));
    let pip = if compact { 0.0 } else { 16.0 + 10.0 };
    if !compact {
        sheet::medal_pip(v, Rect::new(right - w - pip, mid - 2.0, 16.0, 4.0), true);
    }
    if notice {
        let room = between(left + chip + back, right - w - pip);
        moments::status(v, mid, TextId::SaveFailed, room);
    } else if !compact {
        // The heading takes the middle, between the way back and the
        // medals; a Compact strip leaves the page to say where it is.
        let room = 2.0 * (WIDTH / 2.0 - (left + chip + back).max(WIDTH - (right - w - pip)));
        let at = v.snap(mid + v.cap(Role::Title) / 2.0);
        let slot = Slot::centered(WIDTH / 2.0, (room - 2.0 * S12).min(320.0), at);
        v.say(TextId::SectorsHeading, &[], Role::Title, slot, INK);
    }
}

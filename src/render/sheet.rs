//! Sheets: every modal is one raised glass card centred in the field with
//! a title, an optional back glyph, its actions, and one help line for the
//! focused action. The band and rails stay lit around it; the field dims.
use super::*;
use crate::{
    settings::{self, Value},
    ui::{List, Prompt},
};
use chips::Lit;

/// A sheet's width, and the most the fit chain may grow it to.
pub(super) const NARROW: f32 = 400.0;
pub(super) const WIDE: f32 = 496.0;
/// Padding on every side.
const PAD: f32 = S32;
/// Rows: their height, the taller Continue row, and the gap between them.
pub(super) const ROW: f32 = 48.0;
pub(super) const TALL: f32 = 64.0;
const ROW_GAP: f32 = S8;
/// Where a row's label starts, and the confirm glyph's inset from its end.
pub(super) const ROW_TEXT: f32 = 22.0;
const GLYPH_INSET: f32 = 10.0;

const GLASS_TOP: Color = hex(0x151925);
const GLASS_BOTTOM: Color = hex(0x10131c);
const GLASS_EDGE: Color = hex(0x1f2534);
const ROW_INK: Color = hex(0xc9d1de);
/// How long a sheet takes to rise into place, and how far it rises.
const OPEN_SECONDS: f32 = 0.14;
const RISE: f32 = 8.0;

/// What a sheet shows; its content block is drawn by the caller.
pub(super) struct Spec<'a> {
    pub title: TextId,
    pub back: bool,
    pub menu: Menu,
    pub focus: usize,
    /// The height of what sits between the title and the actions.
    pub content: f32,
    /// The content needs the wide sheet whatever the actions need.
    pub wide: bool,
    /// The content again as lines, for a Compact screen's list.
    pub info: &'a [Info<'a>],
}

/// One line of a sheet's content on a Compact screen.
#[derive(Clone, Copy)]
pub(super) enum Info<'a> {
    /// A value's name and the value, as figures.
    Figure(TextId, &'a str),
    /// The medals a sector earned.
    Medals(Medals),
    /// A chapter's extra life.
    Life,
}

/// How far a sheet that has been open `seconds` is into its entrance.
fn opening(seconds: f32) -> f32 {
    let t = (seconds / OPEN_SECONDS).clamp(0.0, 1.0);
    // Ease out: fast at first, settling into place.
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Dims the field under a sheet; the band and rails stay at full light.
pub(super) fn dim(v: &Scene, seconds: f32) {
    let field = v.snap_rect(Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
    let alpha = 0.62 * opening(seconds);
    v.shape(
        field,
        [10.0, 10.0, 0.0, 0.0],
        Fill::flat(opacity(NIGHT, alpha)),
    );
}

/// Whether `id` can be set in `room` by the fit chain short of an
/// ellipsis: full or short wording, at its size or one size smaller.
fn fits(v: &Scene, id: TextId, style: Style, room: f32) -> bool {
    let sizes = [style, style.smaller()];
    let steps = if v.can_step(style) { 2 } else { 1 };
    sizes[..steps].iter().any(|&style| {
        [Form::Full, Form::Short]
            .into_iter()
            .any(|form| v.format(id, &[], form, |t| v.measure(t, style) <= room + 0.5))
    })
}

/// A sheet's width: 400, or 496 when its text needs it, on a Regular
/// screen; the field's width less 16 on a Small one.
fn width(v: &Scene, narrow_fits: impl Fn(f32) -> bool) -> f32 {
    match v.class {
        Class::Regular if narrow_fits(NARROW) => NARROW,
        Class::Regular => WIDE,
        Class::Small | Class::Compact => RIGHT - LEFT - S16,
    }
}

/// A row's height, at least a 32-pixel pointer target.
pub(super) fn row_h(v: &Scene, action: Action) -> f32 {
    if action == Action::Continue {
        let two = v.line_h(Style::from(Role::Body).strong()) + 4.0 + v.line_h(Role::Caption) + S8;
        v.at_least(TALL.max(two), 32.0)
    } else {
        v.at_least(ROW, 32.0)
    }
}

/// Draws the sheet, calling `content` with the content block's rect, and
/// records its rows and back glyph for the pointer.
pub(super) fn draw(
    v: &Scene,
    ui: &Ui,
    spec: Spec,
    label: impl Fn(Action) -> TextId,
    content: impl FnOnce(Rect),
) {
    if v.class == Class::Compact {
        compact::sheet(v, &spec, label, help);
        return;
    }
    let actions = spec.menu.actions;
    let strong = Style::from(Role::Body).strong();
    let chip = chips::size(v, true);
    let row_room = |w: f32| w - 2.0 * PAD - ROW_TEXT - GLYPH_INSET - chip - S8;
    let title_room = |w: f32| w - 2.0 * PAD - if spec.back { chip + S16 } else { 0.0 };
    // The fit chain's fourth step: the sheet grows before anything is cut.
    let w = width(v, |w| {
        !spec.wide
            && fits(v, spec.title, Role::Title.into(), title_room(w))
            && actions
                .iter()
                .all(|&a| fits(v, label(a), strong, row_room(w)))
    });
    let inner = w - 2.0 * PAD;
    let help_lines = actions
        .iter()
        .filter_map(|&a| help(a))
        .map(|id| v.lines((id, &[]), Role::Caption, inner - ROW_TEXT))
        .max()
        .unwrap_or(0);
    let head = head(v, spec.back);
    let gap = v.at_least(ROW_GAP, 2.0);
    let list = actions.iter().map(|&a| row_h(v, a) + gap).sum::<f32>() - gap;
    let content_h = if spec.content > 0.0 {
        spec.content + S24
    } else {
        0.0
    };
    let help_h = if help_lines > 0 {
        S16 + help_lines as f32 * v.line_h(Role::Caption)
    } else {
        0.0
    };
    let h = PAD + head + S24 + content_h + list + help_h + PAD;
    let r = open(v, ui, (w, h), (spec.title, title_room(w)), spec.back);
    let x = r.x + PAD;
    let mut y = r.y + PAD;
    y += head + S24;
    if spec.content > 0.0 {
        content(Rect::new(x, y, inner, spec.content));
        y += content_h;
    }
    v.hits.borrow_mut().begin(spec.menu);
    for (i, &action) in actions.iter().enumerate() {
        let row_rect = Rect::new(x, y, inner, row_h(v, action));
        row(
            v,
            row_rect,
            (label(action), &[]),
            (i == 0, i == spec.focus),
            None,
        );
        v.hits.borrow_mut().push(row_rect);
        y += row_rect.h + gap;
    }
    if help_lines > 0
        && let Some(id) = help(spec.menu.action(spec.focus))
    {
        let top = y - gap + S16;
        let slot = Slot::left(
            x + ROW_TEXT,
            inner - ROW_TEXT,
            v.baseline(Role::Caption, top),
        );
        let slot = v.lead((x, inner), slot);
        v.paragraph((id, &[]), Role::Caption, slot, help_lines, DIM);
    }
    v.set_motion(1.0, 0.0);
}

/// The height of a sheet's title row, with or without a back glyph.
fn head(v: &Scene, back: bool) -> f32 {
    let title = v.line_h(Role::Title);
    if back {
        title.max(chips::size(v, true))
    } else {
        title
    }
}

/// Places a `w` × `h` sheet in the middle of the field, sets it moving
/// into place, and draws its glass, title and back glyph. The caller
/// draws the rest and then stops the motion.
fn open(v: &Scene, ui: &Ui, (w, h): (f32, f32), (title, room): (TextId, f32), back: bool) -> Rect {
    let field_mid = (TOP + BOTTOM) / 2.0;
    // Inside the safe area, where the platform reports one.
    let safe = v.safe;
    let y = (field_mid - h / 2.0).min(safe.y + safe.h - h).max(safe.y);
    let r = v.snap_rect(Rect::new(WIDTH / 2.0 - w / 2.0, y, w, h));
    v.region(1, r);
    let t = opening(ui.sheet_open);
    v.set_motion(t, v.snap(RISE * (1.0 - t)));
    glass(v, r, 16.0);
    let head = head(v, back);
    let top = r.y + PAD;
    let title_top = top + (head - v.line_h(Role::Title)) / 2.0;
    let baseline = v.baseline(Role::Title, title_top);
    let span = (r.x, r.w);
    v.say(
        title,
        &[],
        Role::Title,
        v.lead(span, Slot::left(r.x + PAD, room, baseline)),
        INK,
    );
    if back {
        let size = chips::size(v, true);
        let mid = top + head / 2.0;
        let cw = chips::width(v, Prompt::Back, size);
        // At the title's far end: the right, or the left in Arabic.
        let x = v.mirror(span, r.x + w - PAD - cw, cw);
        chips::chip(v, Prompt::Back, (x, mid), size, Lit::Neutral);
        let reach = v.at_least(size, 32.0);
        v.hits.borrow_mut().back = Some(Rect::new(x, mid - reach / 2.0, cw, reach));
    }
    r
}

/// The help line for an action, if it needs one.
fn help(action: Action) -> Option<TextId> {
    match action {
        Action::Resume => Some(TextId::HelpResume),
        Action::Retry => Some(TextId::HelpRetry),
        Action::MainMenu => Some(TextId::HelpMainMenu),
        Action::Sectors => Some(TextId::HelpSectors),
        Action::NewJourney => Some(TextId::HelpNewJourney),
        Action::Settings => Some(TextId::HelpSettings),
        Action::Continue | Action::Next => None,
    }
}

/// Raised glass: lit at the top edge, a hairline rim, a soft shadow below.
pub(super) fn glass(v: &Scene, r: Rect, radius: f32) {
    let shadow = Rect::new(r.x + 12.0, r.y + 24.0, r.w - 24.0, r.h - 12.0);
    let dark = opacity(BLACK, 0.45);
    v.shape(shadow, [radius; 4], Fill::flat(dark));
    v.halo(shadow, radius, 40.0, dark);
    let hair = v.thick(1.0);
    v.shape(r, [radius; 4], Fill::flat(GLASS_EDGE));
    let inner = Rect::new(r.x + hair, r.y + hair, r.w - 2.0 * hair, r.h - 2.0 * hair);
    v.shape(
        inner,
        [radius - hair; 4],
        Fill::ramp(GLASS_TOP, GLASS_BOTTOM),
    );
    v.rect(
        r.x + radius,
        inner.y,
        r.w - 2.0 * radius,
        hair,
        opacity(WHITE, 0.08),
    );
}

/// One action row. The primary is a cyan glass pill; any other row is
/// plain text until focused, when it gains a cyan outline. The focused
/// row carries the confirm glyph at its end. `detail` is a second line,
/// the Continue row's saved journey.
pub(super) fn row(
    v: &Scene,
    r: Rect,
    (id, args): (TextId, &[Arg]),
    (primary, focused): (bool, bool),
    detail: Option<(TextId, &[Arg])>,
) {
    let r = v.snap_rect(r);
    let radius = if r.h > v.at_least(ROW, 32.0) {
        22.0
    } else {
        r.h / 2.0
    };
    if primary {
        let glow = Rect::new(r.x + 16.0, r.y + 10.0, r.w - 32.0, r.h - 10.0);
        v.halo(glow, radius, 14.0, opacity(CYAN, 0.22));
        let rim = v.thick(1.5);
        v.shape(
            r,
            [radius; 4],
            Fill::lit(hex(0xb8f3fc), 0.45, CYAN, hex(0x2b8fa3)),
        );
        let body = Rect::new(r.x + rim, r.y + rim, r.w - 2.0 * rim, r.h - 2.0 * rim);
        v.shape(
            body,
            [radius - rim; 4],
            Fill::ramp(mix(GLASS_BOTTOM, CYAN, 0.22), mix(GLASS_BOTTOM, CYAN, 0.10)),
        );
    } else if focused {
        v.outline(r, radius, v.thick(1.5), opacity(CYAN, 0.55));
    }
    let style = Style::from(Role::Body);
    let (style, color) = if primary || focused {
        (style.strong(), INK)
    } else {
        (style, ROW_INK)
    };
    let chip = chips::size(v, true);
    let room = r.w - ROW_TEXT - GLYPH_INSET - chip - S8;
    let span = (r.x, r.w);
    match detail {
        None => {
            let baseline = v.snap(r.y + r.h / 2.0 + v.cap(style) / 2.0);
            v.say(
                id,
                args,
                style,
                v.lead(span, Slot::left(r.x + ROW_TEXT, room, baseline)),
                color,
            );
        }
        Some((detail, detail_args)) => {
            // The name and a 15-unit caption, 4 apart, centred.
            let caption = Style::from(Role::Caption).sized(15.0);
            let (name_h, caption_h) = (v.size_of(style), v.size_of(caption));
            let top = r.y + (r.h - (name_h + 4.0 + caption_h)) / 2.0;
            let name = v.snap(top + name_h * 0.888);
            v.say(
                id,
                args,
                style,
                v.lead(span, Slot::left(r.x + ROW_TEXT, room, name)),
                color,
            );
            let below = v.snap(top + name_h + 4.0 + caption_h * 0.888);
            let slot = v.lead(span, Slot::left(r.x + ROW_TEXT, room, below));
            v.say(detail, detail_args, caption, slot, hex(0x9fd9e6));
        }
    }
    if focused {
        let lit = if primary { Lit::Primary } else { Lit::Neutral };
        let w = chips::width(v, Prompt::Confirm, chip);
        // At the row's end: the right, or the left in Arabic.
        let at = (
            v.mirror(span, r.x + r.w - GLYPH_INSET - w, w),
            r.y + r.h / 2.0,
        );
        chips::chip(v, Prompt::Confirm, at, chip, lit);
    }
}

/// The label of an action on a sheet.
pub(super) fn label(action: Action, practice: bool) -> TextId {
    match action {
        Action::Continue => TextId::ContinueJourney,
        Action::NewJourney => TextId::NewJourney,
        Action::Sectors => TextId::SectorSelect,
        Action::Resume => TextId::ActionResume,
        Action::Retry => TextId::RetrySector,
        Action::MainMenu => TextId::MainMenu,
        Action::Next if practice => TextId::BackToSectors,
        Action::Next => TextId::NextSector,
        Action::Settings => TextId::Settings,
    }
}

/// Paused: back to play, retry, or leave.
pub(super) fn pause(v: &Scene, ui: &Ui, game: &Game) {
    let spec = Spec {
        title: TextId::Paused,
        back: true,
        menu: ui::pause_menu(),
        focus: ui.choice,
        content: 0.0,
        wide: false,
        info: &[],
    };
    let practice = game.mode() == Mode::Practice;
    draw(v, ui, spec, |a| label(a, practice), |_| {});
}

/// A Label over a Figure, 10 apart, as one block; its height.
fn figure_block(v: &Scene) -> f32 {
    v.line_h(Role::Label) + 10.0 + v.line_h(Role::Figure)
}
fn figure(v: &Scene, x: f32, top: f32, w: f32, (label, value): (TextId, &str)) {
    let name = v.baseline(Role::Label, top);
    let slot = |y| v.lead((x, w), Slot::left(x, w, y));
    v.say(label, &[], Role::Label, slot(name), DIM);
    let at = v.baseline(Role::Figure, top + v.line_h(Role::Label) + 10.0);
    v.put(value, Role::Figure, slot(at), INK);
}

/// After the last life or the last sector: the outcome, the points, and
/// the way back in first.
pub(super) fn results(v: &Scene, ui: &Ui, game: &Game, victory: bool) {
    let score = Figures::count(v.locale, game.score());
    let spec = Spec {
        title: if victory {
            TextId::JourneyComplete
        } else {
            TextId::OneMoreOrbit
        },
        back: false,
        menu: ui::result_menu(victory),
        focus: ui.choice,
        content: figure_block(v),
        wide: false,
        info: &[Info::Figure(TextId::StatPoints, score.as_str())],
    };
    draw(
        v,
        ui,
        spec,
        |a| label(a, false),
        |r| {
            figure(v, r.x, r.y, r.w, (TextId::StatPoints, score.as_str()));
        },
    );
}

/// The medal row: a pip over each medal's name, 10 apart.
fn medal_block(v: &Scene) -> f32 {
    5.0 + 10.0 + v.line_h(Role::Label)
}

/// A sector cleared: time, bonus and chain, the medals, an extra life when
/// the chapter earned one, and on.
pub(super) fn cleared(v: &Scene, ui: &Ui, game: &Game, summary: SectorSummary) {
    let time = Figures::of(|f| write!(f, "{}", Clock(summary.ticks)));
    let bonus = Figures::of(|f| {
        f.write_char('+')?;
        ark_text::grouped(f, v.locale, summary.bonus)
    });
    let chain = Figures::of(|f| {
        f.write_char('×')?;
        ark_text::grouped(f, v.locale, summary.best_combo)
    });
    let stats = [
        (TextId::StatTime, time.as_str()),
        (TextId::StatBonus, bonus.as_str()),
        (TextId::StatChain, chain.as_str()),
    ];
    let column = |w: f32| (w - 2.0 * PAD - 2.0 * S8) / 3.0;
    let medals = [TextId::MedalClear, TextId::MedalClean, TextId::MedalSwift];
    // Figures are never cut or shortened, so the sheet widens for them,
    // and for a label that does not fit even in its short form.
    let room = column(NARROW);
    let wide = stats.iter().any(|&(label, value)| {
        v.measure(value, Role::Figure) > room || !fits(v, label, Role::Label.into(), room)
    }) || medals
        .iter()
        .any(|&m| !fits(v, m, Role::Label.into(), room));
    let life_line = v.line_h(Role::Caption);
    let mut content = figure_block(v) + S24 + medal_block(v);
    if summary.life_earned {
        content += S24 + life_line;
    }
    let info = [
        Info::Figure(stats[0].0, stats[0].1),
        Info::Figure(stats[1].0, stats[1].1),
        Info::Figure(stats[2].0, stats[2].1),
        Info::Medals(summary.medals),
        Info::Life,
    ];
    let lines = if summary.life_earned { 5 } else { 4 };
    let spec = Spec {
        title: TextId::SectorClear,
        back: false,
        menu: ui::cleared_menu(),
        focus: 0,
        content,
        wide,
        info: &info[..lines],
    };
    let practice = game.mode() == Mode::Practice;
    draw(
        v,
        ui,
        spec,
        |a| label(a, practice),
        |r| {
            let w = (r.w - 2.0 * S8) / 3.0;
            let span = (r.x, r.w);
            // Columns read in the language's direction.
            let column = |i: usize| v.mirror(span, r.x + i as f32 * (w + S8), w);
            for (i, &stat) in stats.iter().enumerate() {
                figure(v, column(i), r.y, w, stat);
            }
            let top = r.y + figure_block(v) + S24;
            for (i, medal) in medals.into_iter().enumerate() {
                let x = column(i);
                let earned = summary.medals.contains(MEDAL_ORDER[i]);
                medal_pip(
                    v,
                    Rect::new(v.mirror((x, w), x, 28.0), top, 28.0, 5.0),
                    earned,
                );
                let name = v.baseline(Role::Label, top + 15.0);
                let ink = if earned { INK } else { MUTED };
                let slot = v.lead((x, w), Slot::left(x, w, name));
                v.say(medal, &[], Role::Label, slot, ink);
            }
            if summary.life_earned {
                let top = top + medal_block(v) + S24;
                let mid = top + life_line / 2.0;
                let pearl = v.mirror(span, r.x, 12.0) + 6.0;
                v.pearl(V2::new(v.snap(pearl), v.snap(mid)), 6.0, 1.0);
                let line = v.baseline(Role::Caption, top);
                let slot = Slot::left(r.x + 12.0 + S12, r.w - 24.0, line);
                let slot = v.lead(span, slot);
                v.say(TextId::ExtraLife, &[], Role::Caption, slot, AMBER);
            }
        },
    );
}

/// A medal as a pip: lit amber when earned, an empty slot when not.
pub(super) fn medal_pip(v: &Scene, r: Rect, earned: bool) {
    let r = v.snap_rect(r);
    let radius = r.h / 2.0 + 0.5;
    if earned {
        v.glow(
            Rect::new(r.x - 6.0, r.y - 6.0, r.w + 12.0, r.h + 12.0),
            opacity(AMBER, 0.6),
        );
        v.shape(r, [radius; 4], Fill::ramp(hex(0xffe2a3), AMBER));
    } else {
        v.shape(r, [radius; 4], Fill::flat(hex(0x232836)));
    }
}

/// Settings rows: their height, the gap between them, and the padding
/// inside them, which bleeds past the sheet's own padding.
const SETTING_ROW: f32 = 48.0;
const SETTING_GAP: f32 = 4.0;
const SETTING_PAD: f32 = S16;

/// Sound, volume, display and language, each changed in place.
pub(super) fn settings(v: &Scene, ui: &Ui, profile: &Profile, focus: usize) {
    if v.class == Class::Compact {
        compact::settings(v, profile, focus);
        return;
    }
    let rows = settings::ROWS;
    let chip = chips::size(v, true);
    let small = chips::size(v, false);
    let row_h = v.at_least(SETTING_ROW, 32.0);
    let help_line = v.line_h(Role::Caption).max(small);
    let value_w = |row: settings::Row| match row.value(&profile.settings) {
        Value::Toggle(_) => 40.0,
        Value::Level(..) => 10.0 * 12.0 - 3.0 + 10.0 + 18.0,
        Value::Text(id) => v.width_of(id, &[], Role::Caption),
        Value::Native(name) => v.measure(name, Role::Caption),
    };
    let fits_in = |w: f32| {
        let room = w - 2.0 * PAD;
        fits(v, TextId::Settings, Role::Title.into(), room - chip - S16)
            && rows
                .iter()
                .all(|&row| v.width_of(row.name(), &[], Role::Body) + S24 + value_w(row) <= room)
    };
    let w = width(v, fits_in);
    let gap = v.at_least(SETTING_GAP, 2.0);
    let list = rows.len() as f32 * (row_h + gap) - gap;
    let h = PAD + head(v, true) + S24 + list + S16 + help_line + PAD;
    let r = open(
        v,
        ui,
        (w, h),
        (TextId::Settings, w - 2.0 * PAD - chip - S16),
        true,
    );
    let mut y = r.y + PAD + head(v, true) + S24;
    v.hits.borrow_mut().begin(List::Settings);
    for (i, &row) in rows.iter().enumerate() {
        let rr = v.snap_rect(Rect::new(
            r.x + PAD - SETTING_PAD,
            y,
            w - 2.0 * (PAD - SETTING_PAD),
            row_h,
        ));
        if i == focus {
            v.outline(rr, 12.0, v.thick(1.5), opacity(CYAN, 0.55));
        }
        let baseline = v.snap(rr.y + rr.h / 2.0 + v.cap(Role::Body) / 2.0);
        let left = rr.x + SETTING_PAD;
        let right = rr.x + rr.w - SETTING_PAD;
        // The name leads and the value ends the row, mirrored in Arabic.
        let span = (rr.x, rr.w);
        v.say(
            row.name(),
            &[],
            Role::Body,
            v.lead(span, Slot::left(left, right - left, baseline)),
            INK,
        );
        let mid = rr.y + rr.h / 2.0;
        let caption = v.snap(mid + v.cap(Role::Caption) / 2.0);
        match row.value(&profile.settings) {
            Value::Toggle(on) => {
                let x = v.mirror(span, right - 40.0, 40.0);
                toggle(v, Rect::new(x, mid - 11.0, 40.0, 22.0), on);
            }
            Value::Level(level, max) => {
                let figure = Figures::count(v.locale, u32::from(level));
                v.put(
                    figure.as_str(),
                    Role::Caption,
                    v.lead(span, Slot::right(right, 18.0, caption)),
                    INK,
                );
                let first = right - 18.0 - 10.0 - (f32::from(max) * 12.0 - 3.0);
                for i in 0..max {
                    let x = v.mirror(span, first + f32::from(i) * 12.0, 9.0);
                    let pip = v.snap_rect(Rect::new(x, mid - 2.5, 9.0, 5.0));
                    let fill = if i < level {
                        Fill::ramp(hex(0xb8f3fc), CYAN)
                    } else {
                        Fill::flat(hex(0x232836))
                    };
                    v.shape(pip, [2.0; 4], fill);
                }
            }
            Value::Text(id) => {
                v.say(
                    id,
                    &[],
                    Role::Caption,
                    v.lead(span, Slot::right(right, right - left, caption)),
                    DIM,
                );
            }
            Value::Native(name) => {
                v.put(
                    name,
                    Role::Caption,
                    v.lead(span, Slot::right(right, right - left, caption)),
                    DIM,
                );
            }
        }
        v.hits.borrow_mut().push(rr);
        y += row_h + gap;
    }
    // The help line shows how to change the focused row.
    let top = y - gap + S16;
    let mid = top + help_line / 2.0;
    let span = (r.x + PAD, w - 2.0 * PAD);
    // The pair of glyphs keeps its order, left then right, and leads the
    // line from the language's side.
    let pair = chips::width(v, Prompt::Left, small) + S8 + chips::width(v, Prompt::Right, small);
    let mut at = v.mirror(span, span.0, pair);
    for prompt in [Prompt::Left, Prompt::Right] {
        at += chips::chip(v, prompt, (at, mid), small, Lit::Neutral) + S8;
    }
    let verb = settings::ROWS[focus.min(rows.len() - 1)].verb();
    let text = span.0 + pair + S12;
    let baseline = v.snap(mid + v.cap(Role::Caption) / 2.0);
    v.say(
        verb,
        &[],
        Role::Caption,
        v.lead(span, Slot::left(text, span.0 + span.1 - text, baseline)),
        DIM,
    );
    v.set_motion(1.0, 0.0);
}

/// A switch: cyan glass with the pearl knob at the right when on, dark
/// with the knob at the left when off; mirrored in Arabic.
fn toggle(v: &Scene, r: Rect, on: bool) {
    let r = v.snap_rect(r);
    let rim = v.thick(1.5);
    let radius = r.h / 2.0;
    let body = Rect::new(r.x + rim, r.y + rim, r.w - 2.0 * rim, r.h - 2.0 * rim);
    if on {
        v.shape(r, [radius; 4], Fill::ramp(hex(0xb8f3fc), CYAN));
        v.shape(
            body,
            [radius - rim; 4],
            Fill::ramp(mix(GLASS_BOTTOM, CYAN, 0.26), mix(GLASS_BOTTOM, CYAN, 0.12)),
        );
    } else {
        v.shape(r, [radius; 4], Fill::flat(hex(0x2a3142)));
        v.shape(body, [radius - rim; 4], Fill::flat(GLASS_BOTTOM));
    }
    let knob = 7.5;
    let cx = if on {
        r.x + r.w - 2.0 - knob
    } else {
        r.x + 2.0 + knob
    };
    // On is the end a line reads towards: the left in Arabic.
    let cx = v.mirror((r.x, r.w), cx - knob, 2.0 * knob) + knob;
    v.pearl(
        V2::new(cx, r.y + r.h / 2.0),
        knob,
        if on { 1.0 } else { 0.5 },
    );
}

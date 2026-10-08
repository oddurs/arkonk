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
/// A glyph chip beside body text or inside a button, and beside captions.
pub(super) const CHIP: f32 = 28.0;
pub(super) const CHIP_SMALL: f32 = 22.0;
const TITLE_BOX: f32 = 26.0 * 1.2;
const HELP_LINE: f32 = 16.0 * 1.4;

const GLASS_TOP: Color = hex(0x151925);
const GLASS_BOTTOM: Color = hex(0x10131c);
const GLASS_EDGE: Color = hex(0x1f2534);
const ROW_INK: Color = hex(0xc9d1de);
/// How long a sheet takes to rise into place, and how far it rises.
const OPEN_SECONDS: f32 = 0.14;
const RISE: f32 = 8.0;

/// What a sheet shows; its content block is drawn by the caller.
pub(super) struct Spec {
    pub title: TextId,
    pub back: bool,
    pub menu: Menu,
    pub focus: usize,
    /// The height of what sits between the title and the actions.
    pub content: f32,
    /// The content needs the wide sheet whatever the actions need.
    pub wide: bool,
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

/// Whether `id` can be set in `room`, in its full or short wording.
fn fits(v: &Scene, id: TextId, style: Style, room: f32) -> bool {
    [Form::Full, Form::Short]
        .into_iter()
        .any(|form| v.format(id, &[], form, |t| v.measure(t, style) <= room))
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
    let actions = spec.menu.actions;
    let strong = Style::from(Role::Body).strong();
    let row_room = |w: f32| w - 2.0 * PAD - ROW_TEXT - GLYPH_INSET - CHIP - S8;
    let title_room = |w: f32| w - 2.0 * PAD - if spec.back { CHIP + S16 } else { 0.0 };
    // The fit chain's fourth step: the sheet grows before anything is cut.
    let narrow_fits = fits(v, spec.title, Role::Title.into(), title_room(NARROW))
        && actions
            .iter()
            .all(|&a| fits(v, label(a), strong, row_room(NARROW)));
    let w = if spec.wide || !narrow_fits {
        WIDE
    } else {
        NARROW
    };
    let inner = w - 2.0 * PAD;
    let help_lines = actions
        .iter()
        .filter_map(|&a| help(a))
        .map(|id| v.lines((id, &[]), Role::Caption, inner - ROW_TEXT))
        .max()
        .unwrap_or(0);
    let head = head(spec.back);
    let list = actions
        .iter()
        .map(|&a| row_height(a) + ROW_GAP)
        .sum::<f32>()
        - ROW_GAP;
    let content_h = if spec.content > 0.0 {
        spec.content + S24
    } else {
        0.0
    };
    let help_h = if help_lines > 0 {
        S16 + help_lines as f32 * HELP_LINE
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
    v.hits.borrow_mut().list = Some(spec.menu.into());
    for (i, &action) in actions.iter().enumerate() {
        let row_rect = Rect::new(x, y, inner, row_height(action));
        row(
            v,
            row_rect,
            (label(action), &[]),
            (i == 0, i == spec.focus),
            None,
        );
        v.hits.borrow_mut().push(row_rect);
        y += row_rect.h + ROW_GAP;
    }
    if help_lines > 0
        && let Some(id) = help(spec.menu.action(spec.focus))
    {
        let top = y - ROW_GAP + S16;
        let slot = Slot::left(
            x + ROW_TEXT,
            inner - ROW_TEXT,
            v.snap(frame::baseline(top, 16.0, 1.4)),
        );
        v.paragraph((id, &[]), Role::Caption, slot, help_lines, DIM);
    }
    v.set_motion(1.0, 0.0);
}

/// The height of a sheet's title row, with or without a back glyph.
fn head(back: bool) -> f32 {
    if back { TITLE_BOX.max(CHIP) } else { TITLE_BOX }
}

/// Places a `w` × `h` sheet in the middle of the field, sets it moving
/// into place, and draws its glass, title and back glyph. The caller
/// draws the rest and then stops the motion.
fn open(v: &Scene, ui: &Ui, (w, h): (f32, f32), (title, room): (TextId, f32), back: bool) -> Rect {
    let field_mid = (TOP + BOTTOM) / 2.0;
    let r = v.snap_rect(Rect::new(WIDTH / 2.0 - w / 2.0, field_mid - h / 2.0, w, h));
    let t = opening(ui.sheet_open);
    v.set_motion(t, v.snap(RISE * (1.0 - t)));
    glass(v, r, 16.0);
    let head = head(back);
    let top = r.y + PAD;
    let title_top = top + (head - TITLE_BOX) / 2.0;
    let baseline = v.snap(frame::baseline(title_top, 26.0, 1.2));
    v.say(
        title,
        &[],
        Role::Title,
        Slot::left(r.x + PAD, room, baseline),
        INK,
    );
    if back {
        let chip = Rect::new(r.x + w - PAD - CHIP, top + (head - CHIP) / 2.0, CHIP, CHIP);
        back_glyph(v, v.snap_rect(chip));
        v.hits.borrow_mut().back = Some(chip);
    }
    r
}

fn row_height(action: Action) -> f32 {
    if action == Action::Continue {
        TALL
    } else {
        ROW
    }
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
    let radius = if r.h > ROW { 22.0 } else { r.h / 2.0 };
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
    let room = r.w - ROW_TEXT - GLYPH_INSET - CHIP - S8;
    match detail {
        None => {
            let baseline = v.snap(r.y + r.h / 2.0 + v.cap(style) / 2.0);
            v.say(
                id,
                args,
                style,
                Slot::left(r.x + ROW_TEXT, room, baseline),
                color,
            );
        }
        Some((detail, detail_args)) => {
            // A 20-unit line and a 15-unit caption, 4 apart, centred.
            let top = r.y + (r.h - (20.0 + 4.0 + 15.0)) / 2.0;
            let name = v.snap(frame::baseline(top, 20.0, 1.0));
            v.say(
                id,
                args,
                style,
                Slot::left(r.x + ROW_TEXT, room, name),
                color,
            );
            let below = v.snap(frame::baseline(top + 24.0, 15.0, 1.0));
            let caption = Style::from(Role::Caption).sized(15.0);
            let slot = Slot::left(r.x + ROW_TEXT, room, below);
            v.say(detail, detail_args, caption, slot, hex(0x9fd9e6));
        }
    }
    if focused {
        let chip = Rect::new(
            r.x + r.w - GLYPH_INSET - CHIP,
            r.y + (r.h - CHIP) / 2.0,
            CHIP,
            CHIP,
        );
        let lit = if primary { Lit::Primary } else { Lit::Neutral };
        let w = chips::width(v, Prompt::Confirm, CHIP);
        chips::chip(
            v,
            Prompt::Confirm,
            (chip.x + CHIP - w, chip.y + CHIP / 2.0),
            CHIP,
            lit,
        );
    }
}

/// The back glyph at a sheet's top corner.
fn back_glyph(v: &Scene, r: Rect) {
    chips::chip(v, Prompt::Back, (r.x, r.y + r.h / 2.0), CHIP, Lit::Neutral);
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
    };
    let practice = game.mode() == Mode::Practice;
    draw(v, ui, spec, |a| label(a, practice), |_| {});
}

/// A Label over a Figure, as one block: 15 tall, 10 apart, 32 tall.
const FIGURE_BLOCK: f32 = 15.0 + 10.0 + 32.0;
fn figure(v: &Scene, x: f32, top: f32, w: f32, (label, value): (TextId, &str)) {
    let name = v.snap(frame::baseline(top, 15.0, 1.0));
    v.say(label, &[], Role::Label, Slot::left(x, w, name), DIM);
    let at = v.snap(frame::baseline(top + 25.0, 32.0, 1.0));
    v.put(value, Role::Figure, Slot::left(x, w, at), INK);
}

/// After the last life or the last sector: the outcome, the points, and
/// the way back in first.
pub(super) fn results(v: &Scene, ui: &Ui, game: &Game, victory: bool) {
    let spec = Spec {
        title: if victory {
            TextId::JourneyComplete
        } else {
            TextId::OneMoreOrbit
        },
        back: false,
        menu: ui::result_menu(victory),
        focus: ui.choice,
        content: FIGURE_BLOCK,
        wide: false,
    };
    let score = Figures::count(v.locale, game.score());
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

/// The medal row: a pip over each medal's name.
const MEDAL_BLOCK: f32 = 5.0 + 10.0 + 15.0;
const LIFE_LINE: f32 = 16.0 * 1.4;

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
    let mut content = FIGURE_BLOCK + S24 + MEDAL_BLOCK;
    if summary.life_earned {
        content += S24 + LIFE_LINE;
    }
    let spec = Spec {
        title: TextId::SectorClear,
        back: false,
        menu: ui::cleared_menu(),
        focus: 0,
        content,
        wide,
    };
    let practice = game.mode() == Mode::Practice;
    draw(
        v,
        ui,
        spec,
        |a| label(a, practice),
        |r| {
            let w = (r.w - 2.0 * S8) / 3.0;
            for (i, &stat) in stats.iter().enumerate() {
                figure(v, r.x + i as f32 * (w + S8), r.y, w, stat);
            }
            let top = r.y + FIGURE_BLOCK + S24;
            for (i, medal) in medals.into_iter().enumerate() {
                let x = r.x + i as f32 * (w + S8);
                let earned = summary.medals.contains(MEDAL_ORDER[i]);
                medal_pip(v, Rect::new(x, top, 28.0, 5.0), earned);
                let name = v.snap(frame::baseline(top + 15.0, 15.0, 1.0));
                let ink = if earned { INK } else { MUTED };
                v.say(medal, &[], Role::Label, Slot::left(x, w, name), ink);
            }
            if summary.life_earned {
                let top = top + MEDAL_BLOCK + S24;
                let mid = top + LIFE_LINE / 2.0;
                v.pearl(V2::new(v.snap(r.x + 6.0), v.snap(mid)), 6.0, 1.0);
                let line = v.snap(frame::baseline(top, 16.0, 1.4));
                let slot = Slot::left(r.x + 12.0 + S12, r.w - 24.0, line);
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
        v.halo(r, radius, 5.0, opacity(AMBER, 0.5));
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
    let rows = settings::ROWS;
    let value_w = |row: settings::Row| match row.value(&profile.settings) {
        Value::Toggle(_) => 40.0,
        Value::Level(..) => 10.0 * 12.0 - 3.0 + 10.0 + 18.0,
        Value::Text(id) => v.width_of(id, &[], Role::Caption),
        Value::Native(name) => v.measure(name, Role::Caption),
    };
    let fits_in = |w: f32| {
        let room = w - 2.0 * PAD;
        fits(v, TextId::Settings, Role::Title.into(), room - CHIP - S16)
            && rows
                .iter()
                .all(|&row| v.width_of(row.name(), &[], Role::Body) + S24 + value_w(row) <= room)
    };
    let w = if fits_in(NARROW) { NARROW } else { WIDE };
    let list = rows.len() as f32 * (SETTING_ROW + SETTING_GAP) - SETTING_GAP;
    let h = PAD + head(true) + S24 + list + S16 + HELP_LINE + PAD;
    let r = open(
        v,
        ui,
        (w, h),
        (TextId::Settings, w - 2.0 * PAD - CHIP - S16),
        true,
    );
    let mut y = r.y + PAD + head(true) + S24;
    v.hits.borrow_mut().list = Some(List::Settings);
    for (i, &row) in rows.iter().enumerate() {
        let rr = v.snap_rect(Rect::new(
            r.x + PAD - SETTING_PAD,
            y,
            w - 2.0 * (PAD - SETTING_PAD),
            SETTING_ROW,
        ));
        if i == focus {
            v.outline(rr, 12.0, v.thick(1.5), opacity(CYAN, 0.55));
        }
        let baseline = v.snap(rr.y + rr.h / 2.0 + v.cap(Role::Body) / 2.0);
        let left = rr.x + SETTING_PAD;
        let right = rr.x + rr.w - SETTING_PAD;
        v.say(
            row.name(),
            &[],
            Role::Body,
            Slot::left(left, right - left, baseline),
            INK,
        );
        let mid = rr.y + rr.h / 2.0;
        let caption = v.snap(mid + v.cap(Role::Caption) / 2.0);
        match row.value(&profile.settings) {
            Value::Toggle(on) => toggle(v, Rect::new(right - 40.0, mid - 11.0, 40.0, 22.0), on),
            Value::Level(level, max) => {
                let figure = Figures::count(v.locale, u32::from(level));
                v.put(
                    figure.as_str(),
                    Role::Caption,
                    Slot::right(right, 18.0, caption),
                    INK,
                );
                let first = right - 18.0 - 10.0 - (f32::from(max) * 12.0 - 3.0);
                for i in 0..max {
                    let pip =
                        v.snap_rect(Rect::new(first + f32::from(i) * 12.0, mid - 2.5, 9.0, 5.0));
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
                    Slot::right(right, right - left, caption),
                    DIM,
                );
            }
            Value::Native(name) => {
                v.put(
                    name,
                    Role::Caption,
                    Slot::right(right, right - left, caption),
                    DIM,
                );
            }
        }
        v.hits.borrow_mut().push(rr);
        y += SETTING_ROW + SETTING_GAP;
    }
    // The help line shows how to change the focused row.
    let top = y - SETTING_GAP + S16;
    let mid = top + HELP_LINE / 2.0;
    let x = r.x + PAD;
    let mut text = x;
    for prompt in [Prompt::Left, Prompt::Right] {
        text += chips::chip(v, prompt, (text, mid), CHIP_SMALL, Lit::Neutral) + S8;
    }
    let verb = settings::ROWS[focus.min(rows.len() - 1)].verb();
    let text = text - S8 + S12;
    let baseline = v.snap(frame::baseline(top, 16.0, 1.4));
    v.say(
        verb,
        &[],
        Role::Caption,
        Slot::left(text, r.x + w - PAD - text, baseline),
        DIM,
    );
    v.set_motion(1.0, 0.0);
}

/// A switch: cyan glass with the pearl knob at the right when on, dark
/// with the knob at the left when off.
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
    v.pearl(
        V2::new(cx, r.y + r.h / 2.0),
        knob,
        if on { 1.0 } else { 0.5 },
    );
}

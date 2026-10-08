//! The screens inside the frame: the title, sector select, and the ready
//! card. The band above carries each one's status; nothing floats in the
//! night around the arch.
use super::*;
use crate::ui::{List, Prompt};
use sheet::{CHIP, ROW, ROW_TEXT};

/// The logo's top edge and cell, and the tagline's line under it.
const LOGO_TOP: f32 = 200.0;
const LOGO_CELL: f32 = 12.0;
const TAGLINE_TOP: f32 = 316.0;
/// The title menu's column.
pub(super) const MENU_X: f32 = 280.0;
pub(super) const MENU_W: f32 = 400.0;

/// The help for the focused title action, if it needs one.
fn title_help(action: Action) -> Option<TextId> {
    match action {
        Action::NewJourney => Some(TextId::HelpNewJourney),
        Action::Sectors => Some(TextId::HelpSectors),
        Action::Settings => Some(TextId::HelpSettings),
        _ => None,
    }
}

/// The logo, the line under it, the menu, and help for the focused row.
pub(super) fn title(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::band_title(v, profile);
    v.logo(WIDTH / 2.0 - (35.0 * LOGO_CELL) / 2.0, LOGO_TOP, LOGO_CELL);
    let tagline = Style::from(Role::Caption).sized(18.0);
    let line = v.snap(frame::baseline(TAGLINE_TOP, 18.0, 1.4));
    v.say(TextId::Tagline, &[], tagline, Slot::line(line), DIM);
    let saved = profile.progress.checkpoint();
    let menu = ui::title_menu(saved.is_some());
    // Where the journey stands belongs on the button that resumes it.
    let detail = saved.map(|c| [Arg::Text(TextId::SectorName(c.sector)), Arg::Count(c.score)]);
    let detail = detail
        .as_ref()
        .map(|args| (TextId::ContinueDetail, &args[..]));
    v.hits.borrow_mut().list = Some(menu.into());
    let mut bottom = ui::TITLE_TOP;
    for (i, &action) in menu.actions.iter().enumerate() {
        let r = menu.column(ui::TITLE_TOP, i);
        let detail = detail.filter(|_| action == Action::Continue);
        sheet::row(
            v,
            r,
            (sheet::label(action, false), &[]),
            (i == 0, i == ui.choice),
            detail,
        );
        v.hits.borrow_mut().push(r);
        bottom = r.bottom();
    }
    if let Some(help) = title_help(menu.action(ui.choice)) {
        let top = bottom + S16;
        let slot = Slot::left(
            MENU_X + ROW_TEXT,
            MENU_W - ROW_TEXT,
            v.snap(frame::baseline(top, 16.0, 1.4)),
        );
        v.paragraph((help, &[]), Role::Caption, slot, 2, DIM);
    }
}

/// Where a sector's card sits: chapters in columns, sectors down them.
pub(super) fn card_rect(id: SectorId) -> Rect {
    let (chapter, row) = (id.index() / 4, id.index() % 4);
    Rect::new(
        80.0 + chapter as f32 * 272.0,
        176.0 + row as f32 * 104.0,
        256.0,
        88.0,
    )
}

/// The sector map: chapter columns of cards, and the selected sector's
/// detail docked under them with its Play action.
pub(super) fn sectors(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::band_sectors(v, profile);
    for chapter in Chapter::ALL {
        let x = 84.0 + chapter.first_sector().index() as f32 / 4.0 * 272.0;
        let at = v.snap(frame::baseline(152.0, 15.0, 1.0));
        v.say(
            TextId::ChapterName(chapter),
            &[],
            Role::Label,
            Slot::left(x, 252.0, at),
            sector_color(0, chapter),
        );
    }
    v.hits.borrow_mut().list = Some(List::Sectors);
    for id in SectorId::all() {
        card(v, id, id == ui.sector, profile);
        v.hits.borrow_mut().push(card_rect(id));
    }
    detail(v, ui.sector, profile);
}

/// Raised glass a little quieter than a sheet's.
fn card_glass(v: &Scene, r: Rect) {
    let hair = v.thick(1.0);
    v.shape(r, [12.0; 4], Fill::flat(hex(0x1a1f2c)));
    let inner = Rect::new(r.x + hair, r.y + hair, r.w - 2.0 * hair, r.h - 2.0 * hair);
    v.shape(
        inner,
        [12.0 - hair; 4],
        Fill::ramp(hex(0x131722), hex(0x0f121a)),
    );
    v.rect(r.x + 12.0, inner.y, r.w - 24.0, hair, opacity(WHITE, 0.05));
}

/// A card: the name, the layout in miniature, and a pip per medal, or a
/// padlock while the sector is closed. Times live in the detail.
fn card(v: &Scene, id: SectorId, selected: bool, profile: &Profile) {
    let r = v.snap_rect(card_rect(id));
    let (level, open) = (id.sector(), profile.progress.is_unlocked(id));
    if selected {
        v.halo(
            Rect::new(r.x + 8.0, r.y + 12.0, r.w - 16.0, r.h - 8.0),
            12.0,
            16.0,
            opacity(CYAN, 0.18),
        );
    }
    card_glass(v, r);
    if selected {
        v.outline(r, 12.0, v.thick(1.5), CYAN);
    }
    let (x, w) = (r.x + S16, r.w - 2.0 * S16);
    let name = v.snap(frame::baseline(r.y + 14.0, 18.0, 1.0));
    let style = Style::from(Role::Body).sized(18.0);
    v.say(
        TextId::SectorName(id),
        &[],
        style,
        Slot::left(x, w, name),
        if open { INK } else { MUTED },
    );
    // The bricks at their field proportions, bottom-left.
    let bottom = r.y + r.h - 14.0;
    let top = bottom - 7.0 * 4.0 + 1.0;
    for cell in FieldCell::all() {
        if level.layout.hp[cell.index()] == 0 {
            continue;
        }
        let colour = if !open {
            opacity(hex(0x2a3142), 0.6)
        } else if level.layout.cores.contains(cell) {
            AMBER
        } else {
            opacity(sector_color(cell.row(), level.chapter), 0.8)
        };
        v.rect(
            x + cell.col() as f32 * 9.0,
            top + cell.row() as f32 * 4.0,
            7.0,
            3.0,
            colour,
        );
    }
    let right = r.x + r.w - S16;
    if open {
        let record = profile.progress.record(id);
        for (j, medal) in MEDAL_ORDER.into_iter().enumerate() {
            let pip = Rect::new(
                right - 12.0 - (2 - j) as f32 * 16.0,
                bottom - 5.0,
                12.0,
                3.0,
            );
            sheet::medal_pip(v, pip, record.medals.contains(medal));
        }
    } else {
        v.padlock(right - 8.0, bottom - 8.0, MUTED);
    }
}

/// The detail sheet's place, the right column's width, and its padding.
const DETAIL: Rect = Rect {
    x: 80.0,
    y: 592.0,
    w: 800.0,
    h: 184.0,
};
const DETAIL_ACTION: f32 = 320.0;
const DETAIL_PAD: (f32, f32) = (32.0, 24.0);
/// How far the detail may grow down when its text wraps: it stops short
/// of the field's foot.
const DETAIL_FOOT: f32 = BOTTOM - S8;

/// The selected sector: where it sits, its name and tip, its medals with
/// the Swift target, and Play, or what opens it.
fn detail(v: &Scene, id: SectorId, profile: &Profile) {
    let level = id.sector();
    let (px, py) = DETAIL_PAD;
    let x = DETAIL.x + px;
    let w = DETAIL.w - 2.0 * px - DETAIL_ACTION - 28.0;
    let tip_lines = v.lines((TextId::SectorTip(id), &[]), Role::Caption, w);
    let medal_lines = if medal_line(v, id, profile, None, w) {
        1
    } else {
        2
    };
    let line = 16.0 * 1.4;
    let text = 15.0
        + 10.0
        + 26.0 * 1.2
        + 10.0
        + tip_lines as f32 * line
        + 16.0
        + 15.0
        + (medal_lines - 1) as f32 * line;
    let h = DETAIL.h.max(text + 2.0 * py).min(DETAIL_FOOT - DETAIL.y);
    let r = v.snap_rect(Rect::new(DETAIL.x, DETAIL.y, DETAIL.w, h));
    sheet::glass(v, r, 16.0);

    let mut y = r.y + py;
    let args = [
        Arg::Text(TextId::ChapterName(level.chapter)),
        Arg::Sector(id),
    ];
    let at = v.snap(frame::baseline(y, 15.0, 1.0));
    v.say(
        TextId::ReadyEyebrow,
        &args,
        Role::Label,
        Slot::left(x, w, at),
        sector_color(0, level.chapter),
    );
    y += 15.0 + 10.0;
    let at = v.snap(frame::baseline(y, 26.0, 1.2));
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Title,
        Slot::left(x, w, at),
        INK,
    );
    y += 26.0 * 1.2 + 10.0;
    let at = v.snap(frame::baseline(y, 16.0, 1.4));
    v.paragraph(
        (TextId::SectorTip(id), &[]),
        Role::Caption,
        Slot::left(x, w, at),
        tip_lines,
        DIM,
    );
    y += tip_lines as f32 * line + 16.0;
    medal_line(v, id, profile, Some(y), w);

    let column = Rect::new(r.x + r.w - px - DETAIL_ACTION, r.y, DETAIL_ACTION, r.h);
    let mid = column.y + column.h / 2.0;
    if profile.progress.is_unlocked(id) {
        let note = 15.0 * 1.4;
        let top = mid - (ROW + 12.0 + note) / 2.0;
        let play = Rect::new(column.x, top, column.w, ROW);
        sheet::row(
            v,
            play,
            (TextId::PlaySector, &[Arg::Sector(id)]),
            (true, true),
            None,
        );
        v.hits.borrow_mut().play = Some(play);
        let caption = Style::from(Role::Caption).sized(15.0);
        let at = v.snap(frame::baseline(top + ROW + 12.0, 15.0, 1.4));
        let slot = Slot::left(column.x + ROW_TEXT, column.w - ROW_TEXT, at);
        v.paragraph((TextId::PracticeNote, &[]), caption, slot, 2, DIM);
    } else {
        let before = [Arg::Sector(SectorId::clamped(id.index().saturating_sub(1)))];
        let at = v.snap(mid + v.cap(Role::Caption) / 2.0);
        let slot = Slot::left(column.x + ROW_TEXT, column.w - ROW_TEXT, at);
        v.paragraph((TextId::UnlockHint, &before), Role::Caption, slot, 2, DIM);
    }
}

/// The medals as pip and name pairs, then the Swift target and best time.
/// With `top`, draws the line there (the times on a second line when they
/// do not fit beside the medals); without, only answers whether one line
/// holds everything.
fn medal_line(v: &Scene, id: SectorId, profile: &Profile, top: Option<f32>, w: f32) -> bool {
    let (level, record) = (id.sector(), profile.progress.record(id));
    let names = [TextId::MedalClear, TextId::MedalClean, TextId::MedalSwift];
    let pairs: f32 = names
        .iter()
        .map(|&m| 16.0 + S8 + v.width_of(m, &[], Role::Label))
        .sum::<f32>()
        + 2.0 * 18.0;
    let par = Arg::Clock(level.par_seconds);
    let best = Arg::Clock(record.best_ticks / TICK_HZ);
    let times = [par, best];
    let time_w = if record.best_ticks > 0 {
        v.width_of(TextId::TargetBest, &times, Role::Caption)
    } else {
        v.measure(
            Figures::of(|f| write!(f, "{}", Clock(level.par_seconds * TICK_HZ))).as_str(),
            Role::Caption,
        )
    };
    let one_line = pairs + S8 + time_w <= w;
    let Some(top) = top else { return one_line };
    let x0 = DETAIL.x + DETAIL_PAD.0;
    let label = v.snap(frame::baseline(top, 15.0, 1.0));
    let mut x = x0;
    for (i, &name) in names.iter().enumerate() {
        let earned = record.medals.contains(MEDAL_ORDER[i]);
        sheet::medal_pip(v, Rect::new(x, top + 5.5, 16.0, 4.0), earned);
        x += 16.0 + S8;
        let lw = v.width_of(name, &[], Role::Label);
        v.say(
            name,
            &[],
            Role::Label,
            Slot::left(x, lw, label),
            if earned { INK } else { DIM },
        );
        x += lw + 18.0;
    }
    let (tx, ty) = if one_line {
        (
            x - 18.0 + S8,
            v.snap(top + 7.5 + v.cap(Role::Caption) / 2.0),
        )
    } else {
        (x0, v.snap(frame::baseline(top + 15.0 + 6.0, 16.0, 1.4)))
    };
    let room = x0 + w - tx;
    if record.best_ticks > 0 {
        v.say(
            TextId::TargetBest,
            &times,
            Role::Caption,
            Slot::left(tx, room, ty),
            DIM,
        );
    } else {
        let target = Figures::of(|f| write!(f, "{}", Clock(level.par_seconds * TICK_HZ)));
        v.put(
            target.as_str(),
            Role::Caption,
            Slot::left(tx, room, ty),
            DIM,
        );
    }
    one_line
}

/// The one moment text sits in the field: the chapter and sector, its
/// name, one tip, and how to serve. It goes on the serve.
pub(super) fn ready(v: &Scene, game: &Game) {
    let id = game.sector();
    let chapter = id.sector().chapter;
    let eyebrow = [Arg::Text(TextId::ChapterName(chapter)), Arg::Sector(id)];
    let mut top = 500.0;
    let at = v.snap(frame::baseline(top, 15.0, 1.0));
    v.say(
        TextId::ReadyEyebrow,
        &eyebrow,
        Role::Label,
        Slot::line(at),
        sector_color(0, chapter),
    );
    top += 15.0 + S12;
    let at = v.snap(frame::baseline(top, 40.0, 1.1));
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Display,
        Slot::line(at),
        INK,
    );
    top += 40.0 * 1.1 + S12;
    let at = v.snap(frame::baseline(top, 20.0, 1.4));
    let tip = Slot::centered(WIDTH / 2.0, 720.0, at);
    let lines = v.paragraph((TextId::SectorTip(id), &[]), Role::Body, tip, 2, DIM);
    top += lines as f32 * 20.0 * 1.4 + S24;
    let words = (TextId::ActionServe, Role::Body.into());
    let w = chips::prompt_width(v, Prompt::Serve, words, CHIP);
    let baseline = v.snap(top + CHIP / 2.0 + v.cap(Role::Body) / 2.0);
    chips::prompt(
        v,
        Prompt::Serve,
        words,
        (v.snap(WIDTH / 2.0 - w / 2.0), baseline),
        CHIP,
        CYAN,
    );
    let start = game.balls()[0].pos;
    let direction = game.launch_velocity().normalized();
    for i in 1..=5 {
        v.circle(
            start + direction * (14.0 * i as f32),
            1.5,
            opacity(CYAN, 0.45 - i as f32 * 0.06),
        );
    }
}

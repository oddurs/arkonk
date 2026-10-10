//! The screens inside the frame: the title, sector select, and the ready
//! card. The band above carries each one's status; nothing floats in the
//! night around the arch.
use super::*;
use crate::ui::{List, Prompt};
use sheet::ROW_TEXT;

/// The logo's top edge and cell, and the tagline's line under it.
const LOGO_TOP: f32 = 200.0;
const LOGO_CELL: f32 = 12.0;
const TAGLINE_TOP: f32 = 316.0;
/// The title menu's column.
pub(super) const MENU_X: f32 = 280.0;
pub(super) const MENU_W: f32 = 400.0;

/// The help for the focused title action, if it needs one.
pub(super) fn title_help(action: Action) -> Option<TextId> {
    match action {
        Action::NewJourney => Some(TextId::HelpNewJourney),
        Action::Sectors => Some(TextId::HelpSectors),
        Action::Settings => Some(TextId::HelpSettings),
        _ => None,
    }
}

/// The logo, the line under it, the menu, and help for the focused row.
pub(super) fn title(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::band_title(v, profile, ui.notice > 0.0);
    if v.class == Class::Compact {
        compact::title(v, ui, profile);
        return;
    }
    frame::field_region(v, 0);
    v.logo(WIDTH / 2.0 - (35.0 * LOGO_CELL) / 2.0, LOGO_TOP, LOGO_CELL);
    let tagline = Style::from(Role::Caption).sized(18.0);
    let line = v.baseline(tagline, TAGLINE_TOP);
    v.say(TextId::Tagline, &[], tagline, Slot::line(line), DIM);
    let saved = profile.progress.checkpoint();
    let menu = ui::title_menu(saved.is_some());
    // Where the journey stands belongs on the button that resumes it.
    let detail = saved.map(|c| [Arg::Text(TextId::SectorName(c.sector)), Arg::Count(c.score)]);
    let detail = detail
        .as_ref()
        .map(|args| (TextId::ContinueDetail, &args[..]));
    v.hits.borrow_mut().begin(menu);
    let mut bottom = ui::TITLE_TOP;
    let gap = v.at_least(S8, 2.0);
    for (i, &action) in menu.actions.iter().enumerate() {
        let top = if i == 0 { bottom } else { bottom + gap };
        let r = Rect::new(MENU_X, top, MENU_W, sheet::row_h(v, action));
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
            v.baseline(Role::Caption, top),
        );
        let slot = v.lead((MENU_X, MENU_W), slot);
        v.paragraph((help, &[]), Role::Caption, slot, 2, DIM);
    }
}

/// Where a sector's card sits on its chapter's page: a grid of four by
/// two on a Regular screen, two by four on a Small one, which keeps room
/// for the detail under it.
pub(super) fn card_rect(v: &Scene, id: SectorId) -> Rect {
    let i = id.chapter_index();
    if v.class == Class::Small {
        let (row, col) = (i / 2, i % 2);
        let h = v.at_least(SMALL_CARD.h, 32.0);
        let gap = v.at_least(4.0, 2.0);
        let (mid, chip) = pager(v);
        let top = mid + chip / 2.0 + S8;
        let w = (SMALL_CARD.w - S16) / 2.0;
        let x = SMALL_CARD.x + col as f32 * (w + S16);
        return Rect::new(x, top + row as f32 * (h + gap), w, h);
    }
    let (row, col) = (i / 4, i % 4);
    Rect::new(
        GRID.x + col as f32 * (CARD.0 + S16),
        GRID.y + row as f32 * (CARD.1 + S16),
        CARD.0,
        CARD.1,
    )
}
/// A Regular page's card grid: its top-left corner and the size of a card.
const GRID: Vec2 = Vec2::new(80.0, 212.0);
const CARD: (f32, f32) = (188.0, 172.0);
/// A Small page: two columns of short cards across this span.
const SMALL_CARD: Rect = Rect {
    x: 80.0,
    y: TOP,
    w: 800.0,
    h: 64.0,
};
/// The chapter tabs over a Regular page: eight in a row from the left.
const TABS: Rect = Rect {
    x: 80.0,
    y: 156.0,
    w: 44.0,
    h: 36.0,
};
const TAB_GAP: f32 = S8;

/// The middle of a Small screen's pager row and its glyphs' size. The
/// glyphs keep their 18-pixel floor, which on the smallest Small frame
/// is twice the design's 22 units, so the row is laid out from the
/// field's top rather than from the label's baseline.
pub(super) fn pager(v: &Scene) -> (f32, f32) {
    let chip = chips::size(v, false);
    (TOP + S8 + chip / 2.0, chip)
}

/// Sector select, a chapter a page: the chapters' tabs, the page's cards,
/// and the selected sector's detail docked under them with its Play
/// action. A Small screen turns pages with a pager instead of tabs; a
/// Compact one shows a sector a page.
pub(super) fn sectors(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::band_sectors(v, profile, ui.notice > 0.0);
    if v.class == Class::Compact {
        compact::sectors(v, ui, profile);
        return;
    }
    frame::field_region(v, 0);
    let page = ui.sector.sector().chapter;
    let small = v.class == Class::Small;
    if small {
        chapter_pager(v, page);
    } else {
        tabs(v, page, profile);
    }
    v.hits.borrow_mut().begin(List::Sectors);
    for id in SectorId::all() {
        if id.sector().chapter != page {
            // Off the page: an empty area keeps each card's index.
            v.hits.borrow_mut().push(Rect::default());
            continue;
        }
        card(v, id, id == ui.sector, profile);
        v.hits.borrow_mut().push(card_rect(v, id));
    }
    let top = if small {
        let last = page.sectors().last().unwrap_or(SectorId::FIRST);
        card_rect(v, last).bottom() + S16
    } else {
        DETAIL.y
    };
    frame::field_region(v, 0);
    detail(v, ui.sector, profile, top);
}

/// A chapter's tab. The tabs run left to right in every language, as the
/// arrows that turn them do.
fn tab_rect(chapter: Chapter) -> Rect {
    let x = TABS.x + chapter.index() as f32 * (TABS.w + TAB_GAP);
    Rect::new(x, TABS.y, TABS.w, TABS.h)
}

/// The chapters as a row of tabs, each numbered in its hue; the page's
/// tab is lit glass, and a chapter not yet reached shows a padlock. The
/// page's name stands at the row's other end.
fn tabs(v: &Scene, page: Chapter, profile: &Profile) {
    for chapter in Chapter::ALL {
        let r = v.snap_rect(tab_rect(chapter));
        let hue = sector_color(0, chapter);
        let open = profile.progress.is_unlocked(chapter.first_sector());
        let lit = chapter == page;
        if lit {
            v.halo(
                Rect::new(r.x + 4.0, r.y + 8.0, r.w - 8.0, r.h - 6.0),
                10.0,
                12.0,
                opacity(hue, 0.18),
            );
            let over = |k: f32| mix(pieces::FIELD, hue, k);
            v.shape(r, [10.0; 4], Fill::ramp(over(0.24), over(0.10)));
            v.outline(r, 10.0, v.thick(1.5), hue);
        } else {
            card_glass(v, r);
            let bar = v.snap_rect(Rect::new(r.x + 12.0, r.y + r.h - 6.0, r.w - 24.0, 2.0));
            let k = if open { 0.7 } else { 0.25 };
            v.rect(bar.x, bar.y, bar.w, bar.h, opacity(hue, k));
        }
        let c = r.center();
        if open {
            let number = Figures::count(v.locale, chapter.index() as u32 + 1);
            let style = Style::from(Role::Label);
            let at = v.snap(c.y + v.cap(style) / 2.0);
            let ink = if lit { INK } else { DIM };
            v.put(number.as_str(), style, Slot::centered(c.x, r.w, at), ink);
        } else {
            v.padlock(c.x, c.y - 1.0, MUTED);
        }
        v.hits.borrow_mut().tabs[chapter.index()] = Some(r);
    }
    let row_end = tab_rect(Chapter::Aurora).right();
    let right = DETAIL.x + DETAIL.w;
    let at = v.snap(TABS.y + TABS.h / 2.0 + v.cap(Role::Label) / 2.0);
    let slot = Slot::right(right, right - row_end - S24, at);
    v.say(
        TextId::ChapterName(page),
        &[],
        Role::Label,
        slot,
        sector_color(0, page),
    );
}

/// A Small screen's chapter heading, between the glyphs that page to the
/// chapters either side.
fn chapter_pager(v: &Scene, chapter: Chapter) {
    let (mid, size) = pager(v);
    let r = SMALL_CARD;
    let at = mid + v.cap(Role::Label) / 2.0;
    chips::chip(v, Prompt::Left, (r.x, mid), size, chips::Lit::Neutral);
    let right = chips::width(v, Prompt::Right, size);
    chips::chip(
        v,
        Prompt::Right,
        (r.x + r.w - right, mid),
        size,
        chips::Lit::Neutral,
    );
    let room = r.w - 2.0 * (size.max(right) + S12);
    let colour = sector_color(0, chapter);
    let slot = Slot::centered(WIDTH / 2.0, room, at);
    v.say(TextId::ChapterName(chapter), &[], Role::Label, slot, colour);
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

/// The bricks of `level` in miniature from (`x`, `y`), a cell every
/// `pitch` with bricks of `brick`: cores amber, gates faint, the rest in
/// their rows' hues, or all dim while the sector is closed.
fn mini_board(
    v: &Scene,
    level: &ark::sectors::Sector,
    open: bool,
    (x, y): (f32, f32),
    pitch: (f32, f32),
) {
    let brick = (pitch.0 - 2.0, pitch.1 - 2.0);
    for cell in FieldCell::all() {
        if level.layout.hp[cell.index()] == 0 {
            continue;
        }
        let colour = if !open {
            opacity(hex(0x2a3142), 0.6)
        } else if level.layout.cores.contains(cell) {
            AMBER
        } else if level.layout.gates.contains(cell) {
            opacity(sector_color(cell.row(), level.chapter), 0.35)
        } else {
            opacity(sector_color(cell.row(), level.chapter), 0.8)
        };
        v.rect(
            x + cell.col() as f32 * pitch.0,
            y + cell.row() as f32 * pitch.1,
            brick.0,
            brick.1,
            colour,
        );
    }
}

/// A card: the name, the layout in miniature, and a pip per medal, or a
/// padlock while the sector is closed. Times live in the detail.
fn card(v: &Scene, id: SectorId, selected: bool, profile: &Profile) {
    let r = v.snap_rect(card_rect(v, id));
    v.region(0, r);
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
    // In Arabic the card mirrors: the name at the right, the board and
    // medals swapping ends. The board itself is a map and never flips.
    let span = (r.x, r.w);
    let style = Style::from(Role::Body).sized(18.0);
    // A Small screen's short card sets the board at its right end,
    // beside the name, and its medals under the name.
    let short = v.class == Class::Small;
    let name = v.baseline(style, r.y + if short { 8.0 } else { 14.0 });
    let board = 12.0 * 9.0;
    // A short card's padlock sits beside the board: under the name there
    // is room for a medal pip, not for a padlock.
    let lock = if short && !open { LOCK_W + S12 } else { 0.0 };
    let w = if short { w - board - S16 - lock } else { w };
    v.say(
        TextId::SectorName(id),
        &[],
        style,
        v.lead(span, Slot::left(x, w, name)),
        if open { INK } else { MUTED },
    );
    // The bricks at their field proportions: across a tall card's middle,
    // or at a short card's right end.
    let bottom = r.y + r.h - 14.0;
    if short {
        let (bx, top) = (r.x + r.w - S16 - board, r.y + (r.h - 27.0) / 2.0);
        let bx = v.mirror(span, bx, board);
        mini_board(v, level, open, (bx, top), (9.0, 4.0));
    } else {
        let wide = 12.0 * 13.0;
        let bx = v.snap(r.x + (r.w - wide) / 2.0);
        mini_board(v, level, open, (bx, v.snap(r.y + 70.0)), (13.0, 6.0));
    }
    // Medals bottom-right, or under the name on a short card.
    let right = if short {
        x + 3.0 * 16.0 - 4.0
    } else {
        r.x + r.w - S16
    };
    let bottom = if short { r.y + r.h - 10.0 } else { bottom };
    if open {
        let record = profile.progress.record(id);
        for (j, medal) in MEDAL_ORDER.into_iter().enumerate() {
            let px = v.mirror(span, right - 12.0 - (2 - j) as f32 * 16.0, 12.0);
            let pip = Rect::new(px, bottom - 5.0, 12.0, 3.0);
            sheet::medal_pip(v, pip, record.medals.contains(medal));
        }
    } else {
        let (cx, cy) = if short {
            let bx = v.mirror(span, r.x + r.w - S16 - board, board);
            // Beside the board, on the side towards the name.
            let lx = if v.rtl() {
                bx + board + S12 + LOCK_W / 2.0
            } else {
                bx - S12 - LOCK_W / 2.0
            };
            (lx, r.y + r.h / 2.0)
        } else {
            let lx = v.mirror(span, right - LOCK_W, LOCK_W) + LOCK_W / 2.0;
            (lx, bottom - 8.0)
        };
        v.padlock(cx, cy, MUTED);
    }
}
/// A padlock's width.
const LOCK_W: f32 = 16.0;

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

/// The selected sector, its sheet's top at `top`: where it sits, its
/// name and tip, its medals with the Swift target, and Play, or what
/// opens it. The sheet grows down for wrapped text, short of the foot.
fn detail(v: &Scene, id: SectorId, profile: &Profile, top: f32) {
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
    let (label, title, line) = (
        v.line_h(Role::Label),
        v.line_h(Role::Title),
        v.line_h(Role::Caption),
    );
    let text = label
        + 10.0
        + title
        + 10.0
        + tip_lines as f32 * line
        + 16.0
        + medal_h(v)
        + (medal_lines - 1) as f32 * line;
    let note = Style::from(Role::Caption).sized(15.0);
    let play = sheet::row_h(v, Action::Resume);
    let note_w = DETAIL_ACTION - ROW_TEXT;
    let note_lines = v
        .lines((TextId::PracticeNote, &[]), note, note_w)
        .clamp(1, 3);
    let action = play + 12.0 + note_lines as f32 * v.line_h(note);
    let h = DETAIL
        .h
        .max(text.max(action) + 2.0 * py)
        .min(DETAIL_FOOT - top);
    let r = v.snap_rect(Rect::new(DETAIL.x, top, DETAIL.w, h));
    sheet::glass(v, r, 16.0);
    v.region(0, r);

    let mut y = r.y + py;
    // In Arabic the text column takes the right and Play the left.
    let span = (r.x, r.w);
    let args = [
        Arg::Text(TextId::ChapterName(level.chapter)),
        Arg::Sector(id),
    ];
    let at = v.baseline(Role::Label, y);
    v.say(
        TextId::ReadyEyebrow,
        &args,
        Role::Label,
        v.lead(span, Slot::left(x, w, at)),
        sector_color(0, level.chapter),
    );
    y += label + 10.0;
    let at = v.baseline(Role::Title, y);
    v.say(
        TextId::SectorName(id),
        &[],
        Role::Title,
        v.lead(span, Slot::left(x, w, at)),
        INK,
    );
    y += title + 10.0;
    let at = v.baseline(Role::Caption, y);
    v.paragraph(
        (TextId::SectorTip(id), &[]),
        Role::Caption,
        v.lead(span, Slot::left(x, w, at)),
        tip_lines,
        DIM,
    );
    y += tip_lines as f32 * line + 16.0;
    medal_line(v, id, profile, Some(y), w);

    let column_x = v.mirror(span, r.x + r.w - px - DETAIL_ACTION, DETAIL_ACTION);
    let column = Rect::new(column_x, r.y, DETAIL_ACTION, r.h);
    let inside = (column.x, column.w);
    let mid = column.y + column.h / 2.0;
    if profile.progress.is_unlocked(id) {
        let top = mid - action / 2.0;
        let button = Rect::new(column.x, top, column.w, play);
        sheet::row(
            v,
            button,
            (TextId::PlaySector, &[Arg::Sector(id)]),
            (true, true),
            None,
        );
        v.hits.borrow_mut().play = Some(button);
        let at = v.baseline(note, top + play + 12.0);
        let slot = v.lead(inside, Slot::left(column.x + ROW_TEXT, note_w, at));
        v.paragraph((TextId::PracticeNote, &[]), note, slot, note_lines, DIM);
    } else {
        let before = [Arg::Sector(SectorId::clamped(id.index().saturating_sub(1)))];
        let at = v.snap(mid + v.cap(Role::Caption) / 2.0);
        let slot = Slot::left(column.x + ROW_TEXT, column.w - ROW_TEXT, at);
        let slot = v.lead(inside, slot);
        v.paragraph((TextId::UnlockHint, &before), Role::Caption, slot, 2, DIM);
    }
}

/// The medal line's height: its labels, or the times beside them.
fn medal_h(v: &Scene) -> f32 {
    v.line_h(Role::Label).max(v.line_h(Role::Caption))
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
    // Where the names cannot fit beside each other, the labels hide and
    // the pips speak for themselves.
    let labelled = pairs <= w;
    let pairs = if labelled {
        pairs
    } else {
        3.0 * 16.0 + 2.0 * S8
    };
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
    let span = (DETAIL.x, DETAIL.w);
    let mid = top + medal_h(v) / 2.0;
    let label = v.snap(mid + v.cap(Role::Label) / 2.0);
    let mut x = x0;
    for (i, &name) in names.iter().enumerate() {
        let earned = record.medals.contains(MEDAL_ORDER[i]);
        let pip = Rect::new(v.mirror(span, x, 16.0), mid - 2.0, 16.0, 4.0);
        sheet::medal_pip(v, pip, earned);
        x += 16.0 + S8;
        if labelled {
            let lw = v.width_of(name, &[], Role::Label);
            let ink = if earned { INK } else { DIM };
            let slot = v.lead(span, Slot::left(x, lw, label));
            v.say(name, &[], Role::Label, slot, ink);
            x += lw + 18.0;
        }
    }
    // The times follow the last name, or the last pip, a gap after.
    let end = if labelled { x - 18.0 } else { x - S8 };
    let (tx, ty) = if one_line {
        (end + S8, v.snap(mid + v.cap(Role::Caption) / 2.0))
    } else {
        (x0, v.baseline(Role::Caption, top + medal_h(v)))
    };
    let room = x0 + w - tx;
    if record.best_ticks > 0 {
        v.say(
            TextId::TargetBest,
            &times,
            Role::Caption,
            v.lead(span, Slot::left(tx, room, ty)),
            DIM,
        );
    } else {
        let target = Figures::of(|f| write!(f, "{}", Clock(level.par_seconds * TICK_HZ)));
        v.put(
            target.as_str(),
            Role::Caption,
            v.lead(span, Slot::left(tx, room, ty)),
            DIM,
        );
    }
    one_line
}

/// The one moment text sits in the field: the chapter and sector, its
/// name, one tip, and how to serve. It goes on the serve. It stands from
/// y 500, or higher where floors make it taller than the room below.
pub(super) fn ready(v: &Scene, game: &Game) {
    frame::field_region(v, 0);
    let id = game.sector();
    let chapter = id.sector().chapter;
    let eyebrow = [Arg::Text(TextId::ChapterName(chapter)), Arg::Sector(id)];
    // A Small screen steps the hero line down to Title size.
    let display = match v.class {
        Class::Regular => Style::from(Role::Display),
        Class::Small | Class::Compact => Style::from(Role::Display).sized(26.0),
    };
    let tip_style = Style::from(Role::Body);
    let tip_w = (RIGHT - LEFT - 2.0 * S32).min(720.0);
    let tip = (TextId::SectorTip(id), &[][..]);
    // Floors make a Compact screen's lines few and tall: the tip may take
    // a fourth.
    let most = if v.class == Class::Compact { 4 } else { 3 };
    let tip_lines = v.lines(tip, tip_style, tip_w).min(most);
    let words = (TextId::ActionServe, tip_style);
    let chip = chips::size(v, true);
    let gap = v.at_least(S12, 2.0);
    let heights = [
        v.line_of((TextId::ReadyEyebrow, &eyebrow), Role::Label, 0.0)
            .0,
        v.line_of((TextId::SectorName(id), &[]), display, 0.0).0,
        tip_lines as f32 * v.line_of(tip, tip_style, 0.0).0,
    ];
    let total = heights.iter().sum::<f32>() + 2.0 * gap + v.at_least(S24, 2.0) + chip;
    let mut top = v.snap(500.0_f32.min(BOTTOM - DRAIN_ROOM - total).max(TOP + S8));
    let at = v
        .line_of((TextId::ReadyEyebrow, &eyebrow), Role::Label, top)
        .1;
    v.say(
        TextId::ReadyEyebrow,
        &eyebrow,
        Role::Label,
        Slot::line(at),
        sector_color(0, chapter),
    );
    top += heights[0] + gap;
    let at = v.line_of((TextId::SectorName(id), &[]), display, top).1;
    v.say(TextId::SectorName(id), &[], display, Slot::line(at), INK);
    top += heights[1] + gap;
    let at = v.line_of(tip, tip_style, top).1;
    let slot = Slot::centered(WIDTH / 2.0, tip_w, at);
    v.paragraph(tip, tip_style, slot, tip_lines, DIM);
    top += heights[2] + v.at_least(S24, 2.0);
    let w = chips::prompt_width(v, Prompt::Serve, words, chip);
    let baseline = v.snap(top + chip / 2.0 + v.cap(tip_style) / 2.0);
    chips::prompt(
        v,
        Prompt::Serve,
        words,
        (v.snap(WIDTH / 2.0 - w / 2.0), baseline),
        chip,
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

/// What the ready card keeps clear above the field's foot: the paddle
/// and the drain.
const DRAIN_ROOM: f32 = BOTTOM - PADDLE_Y + 16.0;

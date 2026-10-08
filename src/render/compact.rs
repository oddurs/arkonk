//! The Compact layout, for frames under 400 pixels wide: the 5×7 pixel
//! font at whole pixels, the field as one page of lines. Sheets become
//! full-screen lists, one row per line; sector select shows one sector a
//! page; glyphs are bracketed text. Everything here is measured in
//! physical pixels, since a few of them are all there is.
use super::*;
use crate::{
    settings::{self, Value},
    ui::{List, Prompt},
};
use chips::Lit;
use sheet::{Info, Spec};

/// The gap under a line, and the inset from the field's edge, in pixels.
const LEAD: f32 = 3.0;
const INSET: f32 = 3.0;
const ROW_INK: Color = hex(0xc9d1de);

/// Lines set down the field from its top: a cursor and the column.
struct Page<'s, 'a> {
    v: &'s Scene<'a>,
    x: f32,
    w: f32,
    y: f32,
    px: f32,
}
impl<'s, 'a> Page<'s, 'a> {
    fn new(v: &'s Scene<'a>) -> Self {
        let px = 1.0 / v.density;
        let field = v.snap_rect(Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
        Self {
            v,
            x: field.x + INSET * px,
            w: field.w - 2.0 * INSET * px,
            y: field.y + INSET * px,
            px,
        }
    }
    /// The height of one line of `id` in `style`: its face's own, pixel
    /// or Noto, and the gap under it.
    fn pitch(&self, (id, args): (TextId, &[Arg]), style: Style) -> f32 {
        self.v
            .format(id, args, Form::Full, |t| self.v.pitch(style, t))
            .max(self.v.pitch(style, "A"))
    }
    /// The baseline of a line of `id` set from the cursor.
    fn baseline(&self, (id, args): (TextId, &[Arg]), style: Style) -> f32 {
        let cap = self
            .v
            .format(id, args, Form::Full, |t| self.v.cap_of(style, t));
        self.v.snap(self.y + cap)
    }
    /// One line of `id`, left in a column `room` wide, or centred; moves
    /// the cursor past it and returns the line's box.
    fn line(
        &mut self,
        text: (TextId, &[Arg]),
        style: Style,
        (room, centred): (f32, bool),
        color: Color,
    ) -> Rect {
        let at = self.baseline(text, style);
        let slot = if centred {
            Slot::centered(self.x + self.w / 2.0, room, at)
        } else {
            self.v.lead(self.span(), Slot::left(self.x, room, at))
        };
        self.v.say(text.0, text.1, style, slot, color);
        self.advance(self.pitch(text, style))
    }
    /// Lines of wrapped `text` across the column, up to `max`.
    fn paragraph(&mut self, text: (TextId, &[Arg]), style: Style, max: usize, color: Color) {
        let lines = self.v.lines(text, style, self.w).min(max);
        let at = self.baseline(text, style);
        let slot = self.v.lead(self.span(), Slot::left(self.x, self.w, at));
        self.v.paragraph(text, style, slot, lines, color);
        let pitch = self
            .v
            .format(text.0, text.1, Form::Full, |t| self.v.pitch(style, t));
        self.y += lines as f32 * pitch;
    }
    /// Help in the lines left above the field's foot, up to three; none
    /// when not one is left, since help only explains.
    fn fill(&mut self, text: (TextId, &[Arg]), style: Style, color: Color) {
        let pitch = self
            .v
            .format(text.0, text.1, Form::Full, |t| self.v.pitch(style, t));
        let room = BOTTOM - INSET * self.px - self.y;
        let lines = ((room + LEAD * self.px) / pitch).floor().min(3.0);
        if lines >= 1.0 {
            self.paragraph(text, style, lines as usize, color);
        }
    }
    /// The column, for mirroring across it in Arabic.
    fn span(&self) -> (f32, f32) {
        (self.x, self.w)
    }
    /// Where the focus mark sits: in the inset before a line, which is
    /// the right in Arabic.
    fn mark(&self) -> f32 {
        let inset = INSET * self.px;
        let outer = (self.x - inset, self.w + 2.0 * inset);
        self.v.mirror(outer, self.x - inset, self.px)
    }
    /// Moves the cursor down `h` and returns the band it passed.
    fn advance(&mut self, h: f32) -> Rect {
        let r = Rect::new(self.x, self.y, self.w, h);
        self.y = self.v.snap(self.y + h);
        r
    }
    fn gap(&mut self, pixels: f32) {
        self.y = self.v.snap(self.y + pixels * self.px);
    }
}

/// Covers the field with its glass, for a page over whatever was there.
fn cover(v: &Scene) {
    let field = v.snap_rect(Rect::new(LEFT, TOP, RIGHT - LEFT, BOTTOM - TOP));
    v.rect(field.x, field.y, field.w, field.h, frame::FIELD_GLASS);
}

/// A sheet's title line, with the back glyph at its right end.
fn heading(page: &mut Page, id: TextId, back: bool) {
    let v = page.v;
    let style = Style::from(Role::Title);
    let chip = if back {
        chips::width(v, Prompt::Back, 0.0)
    } else {
        0.0
    };
    let room = page.w - if back { chip + 4.0 * page.px } else { 0.0 };
    let top = page.y;
    let r = page.line((id, &[]), style, (room, false), INK);
    if back {
        let at = v.snap(v.mirror(page.span(), page.x + page.w - chip, chip));
        chips::chip(
            v,
            Prompt::Back,
            (at, top + r.h / 2.0 - LEAD * page.px / 2.0),
            0.0,
            Lit::Neutral,
        );
        v.hits.borrow_mut().back = Some(Rect::new(at, top, chip, r.h));
    }
    page.gap(2.0);
}

/// One action on its own line: the focused one marked at the field's edge
/// and carrying the confirm glyph, the primary in cyan.
fn row(page: &mut Page, text: (TextId, &[Arg]), (primary, focused): (bool, bool)) -> Rect {
    let v = page.v;
    let style = if focused {
        Style::from(Role::Body).strong()
    } else {
        Style::from(Role::Body)
    };
    let chip = chips::width(v, Prompt::Confirm, 0.0);
    let room = page.w - chip - 4.0 * page.px;
    let color = match (primary, focused) {
        (true, true) => CYAN,
        (false, true) => INK,
        _ => ROW_INK,
    };
    let top = page.y;
    let r = page.line(text, style, (room, false), color);
    let mid = top + (r.h - LEAD * page.px) / 2.0;
    if focused {
        // The mark sits in the inset, two pixels clear of the letters.
        v.rect(page.mark(), top, page.px, r.h - LEAD * page.px, color);
        let lit = if primary { Lit::Primary } else { Lit::Neutral };
        let x = v.mirror(page.span(), page.x + page.w - chip, chip);
        chips::chip(v, Prompt::Confirm, (x, mid), 0.0, lit);
    }
    r
}

/// The three medals as pips, amber when earned: their names would not
/// fit beside each other in every language.
fn medal_line(page: &mut Page, medals: Medals) {
    let (v, px) = (page.v, page.px);
    for (i, medal) in MEDAL_ORDER.into_iter().enumerate() {
        let colour = if medals.contains(medal) {
            AMBER
        } else {
            hex(0x232836)
        };
        let x = v.mirror(page.span(), page.x + i as f32 * 7.0 * px, 5.0 * px);
        v.rect(x, page.y + 2.0 * px, 5.0 * px, 2.0 * px, colour);
    }
    page.gap(7.0);
}

/// A sheet as a list filling the field: its title, its figures and
/// medals as lines, the actions one per line, and the focused action's
/// help.
pub(super) fn sheet(
    v: &Scene,
    spec: &Spec,
    label: impl Fn(Action) -> TextId,
    help: impl Fn(Action) -> Option<TextId>,
) {
    cover(v);
    frame::field_region(v, 1);
    let mut page = Page::new(v);
    heading(&mut page, spec.title, spec.back);
    for info in spec.info {
        match *info {
            Info::Figure(name, value) => {
                let at = page.baseline((name, &[]), Role::Label.into());
                let span = page.span();
                v.say(
                    name,
                    &[],
                    Role::Label,
                    v.lead(span, Slot::left(page.x, page.w / 2.0, at)),
                    DIM,
                );
                let w = v.measure(value, Role::Figure);
                v.put(
                    value,
                    Role::Figure,
                    v.lead(span, Slot::right(page.x + page.w, w, at)),
                    INK,
                );
                page.advance(page.pitch((name, &[]), Role::Label.into()));
            }
            Info::Medals(medals) => medal_line(&mut page, medals),
            Info::Life => {
                page.line(
                    (TextId::ExtraLife, &[]),
                    Role::Caption.into(),
                    (page.w, false),
                    AMBER,
                );
            }
        }
    }
    if !spec.info.is_empty() {
        page.gap(2.0);
    }
    v.hits.borrow_mut().begin(spec.menu);
    for (i, &action) in spec.menu.actions.iter().enumerate() {
        let r = row(&mut page, (label(action), &[]), (i == 0, i == spec.focus));
        v.hits.borrow_mut().push(r);
    }
    if let Some(id) = help(spec.menu.action(spec.focus)) {
        page.gap(2.0);
        page.fill((id, &[]), Role::Caption.into(), DIM);
    }
}

/// How a setting's row sits on a Compact page: its value's width, whether
/// the value takes a line of its own, and the row's height.
fn setting_row(page: &Page, setting: settings::Row, profile: &Profile) -> (f32, bool, f32) {
    let v = page.v;
    let px = page.px;
    let caption = Style::from(Role::Caption);
    let value = setting.value(&profile.settings);
    let value_w = match value {
        Value::Toggle(_) => 9.0 * px,
        Value::Level(level, max) => {
            let level = Figures::count(v.locale, u32::from(level));
            v.measure(level.as_str(), caption) + 3.0 * px + f32::from(max) * 2.0 * px
        }
        Value::Text(id) => v.width_of(id, &[], caption).min(page.w),
        Value::Native(name) => v.measure(name, caption),
    };
    // A value too wide to share the line takes the next one.
    let gap = 4.0 * px;
    let name = Style::from(Role::Body);
    let alone = v.width_of(setting.name(), &[], name) + gap + value_w > page.w;
    let mut h = page.pitch((setting.name(), &[]), name);
    if alone {
        h += match value {
            Value::Native(name) => v.pitch(caption, name),
            Value::Text(id) => v.format(id, &[], Form::Full, |t| v.pitch(caption, t)),
            Value::Toggle(_) | Value::Level(..) => v.pitch(caption, "A"),
        };
    }
    (value_w, alone, h)
}

/// Settings as a list: each setting's name, and its value at the right
/// end of the line, drawn in pixels; how to change it under them. Where
/// the page cannot hold every row, it scrolls to keep the focused one in
/// view, and the help goes first.
pub(super) fn settings(v: &Scene, profile: &Profile, focus: usize) {
    cover(v);
    frame::field_region(v, 1);
    let mut page = Page::new(v);
    let px = page.px;
    heading(&mut page, TextId::Settings, true);
    v.hits.borrow_mut().begin(List::Settings);
    let rows = settings::ROWS;
    let focus = focus.min(rows.len() - 1);
    let bottom = BOTTOM - INSET * px;
    let height = |i: usize| setting_row(&page, rows[i], profile).2;
    // The first row shown: the earliest that still lets the focused one fit.
    let mut first = 0;
    while first < focus && page.y + (first..=focus).map(height).sum::<f32>() > bottom {
        first += 1;
    }
    let mut full = false;
    for (i, &setting) in rows.iter().enumerate() {
        let (value_w, alone, h) = setting_row(&page, setting, profile);
        full |= i >= first && page.y + h > bottom;
        if i < first || full {
            // Off the page: an empty hit area keeps the rows' numbering.
            v.hits.borrow_mut().push(Rect::default());
            continue;
        }
        let focused = i == focus;
        let style = Style::from(Role::Body);
        let value = setting.value(&profile.settings);
        let level = match value {
            Value::Level(l, _) => u32::from(l),
            _ => 0,
        };
        let level = Figures::count(v.locale, level);
        let caption = Style::from(Role::Caption);
        let gap = 4.0 * px;
        let top = page.y;
        let colour = if focused { INK } else { ROW_INK };
        let room = if alone {
            page.w
        } else {
            page.w - value_w - gap
        };
        let mut r = page.line((setting.name(), &[]), style, (room, false), colour);
        let value_top = if alone { page.y } else { top };
        let right = page.x + page.w;
        // Values end the line: at the right, or the left in Arabic.
        let span = page.span();
        let flip = |x: f32, w: f32| v.mirror(span, x, w);
        let mid = value_top + 3.5 * px;
        let at = |text: &str| v.snap(value_top + v.cap_of(caption, text));
        match value {
            Value::Toggle(on) => {
                // A switch nine pixels wide: lit with the knob right when on.
                let (fill, knob) = if on {
                    (opacity(CYAN, 0.5), right - 4.0 * px)
                } else {
                    (hex(0x2a3142), right - 8.0 * px)
                };
                let (x, knob) = (flip(right - 9.0 * px, 9.0 * px), flip(knob, 3.0 * px));
                v.rect(x, mid - 2.0 * px, 9.0 * px, 5.0 * px, fill);
                v.rect(knob, mid - px, 3.0 * px, 3.0 * px, INK);
            }
            Value::Level(on, max) => {
                let fw = v.measure(level.as_str(), caption);
                v.put(
                    level.as_str(),
                    caption,
                    v.lead(span, Slot::right(right, fw, at(level.as_str()))),
                    INK,
                );
                let first = right - value_w;
                for k in 0..max {
                    let lit = if k < on { CYAN } else { hex(0x232836) };
                    v.rect(
                        flip(first + f32::from(k) * 2.0 * px, px),
                        mid - 2.0 * px,
                        px,
                        5.0 * px,
                        lit,
                    );
                }
            }
            Value::Text(id) => {
                let baseline = v.format(id, &[], Form::Full, at);
                let slot = v.lead(span, Slot::right(right, value_w, baseline));
                v.say(id, &[], caption, slot, DIM);
            }
            Value::Native(name) => {
                let slot = v.lead(span, Slot::right(right, value_w, at(name)));
                v.put(name, caption, slot, DIM);
            }
        }
        if alone {
            let pitch = match value {
                Value::Native(name) => v.pitch(caption, name),
                Value::Text(id) => v.format(id, &[], Form::Full, |t| v.pitch(caption, t)),
                Value::Toggle(_) | Value::Level(..) => v.pitch(caption, "A"),
            };
            let below = page.advance(pitch);
            r.h += below.h;
        }
        if focused {
            v.rect(page.mark(), top, px, r.h - LEAD * px, INK);
        }
        v.hits.borrow_mut().push(r);
    }
    let verb = rows[focus].verb();
    let help = 2.0 * px + page.pitch((verb, &[]), Role::Caption.into());
    if full || page.y + help > bottom {
        return;
    }
    page.gap(2.0);
    let mid = page.y + 3.5 * px;
    let pair = chips::width(v, Prompt::Left, 0.0) + 2.0 * px + chips::width(v, Prompt::Right, 0.0);
    let mut at = v.mirror(page.span(), page.x, pair);
    for prompt in [Prompt::Left, Prompt::Right] {
        at += chips::chip(v, prompt, (at, mid), 0.0, Lit::Neutral) + 2.0 * px;
    }
    let x = page.x + pair + 2.0 * px;
    let baseline = page.baseline((verb, &[]), Role::Caption.into());
    let room = page.x + page.w - x - 2.0 * px;
    v.say(
        verb,
        &[],
        Role::Caption,
        v.lead(page.span(), Slot::left(x + 2.0 * px, room, baseline)),
        DIM,
    );
}

/// The title: the logo at a whole number of pixels a cell, the tagline,
/// the menu one row per line, and the focused row's help.
pub(super) fn title(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::field_region(v, 0);
    let mut page = Page::new(v);
    let px = page.px;
    let cell = (page.w / px / 35.0 / 2.0).floor().clamp(1.0, 4.0);
    page.gap(INSET);
    let logo_w = 35.0 * cell * px;
    v.logo(v.snap(page.x + (page.w - logo_w) / 2.0), page.y, cell * px);
    page.gap(7.0 * cell + 4.0);
    page.line(
        (TextId::Tagline, &[]),
        Role::Caption.into(),
        (page.w, true),
        DIM,
    );
    page.gap(4.0);
    let menu = ui::title_menu(profile.progress.checkpoint().is_some());
    v.hits.borrow_mut().begin(menu);
    for (i, &action) in menu.actions.iter().enumerate() {
        let r = row(
            &mut page,
            (sheet::label(action, false), &[]),
            (i == 0, i == ui.choice),
        );
        v.hits.borrow_mut().push(r);
    }
    if let Some(help) = screens::title_help(menu.action(ui.choice)) {
        page.gap(2.0);
        page.fill((help, &[]), Role::Caption.into(), DIM);
    }
}

/// Sector select, one sector a page: where it sits, its name, the board
/// in miniature, its medals and times, and Play or what opens it, with
/// the page's place among the twelve at the foot.
pub(super) fn sectors(v: &Scene, ui: &Ui, profile: &Profile) {
    frame::field_region(v, 0);
    let mut page = Page::new(v);
    let px = page.px;
    let id = ui.sector;
    let level = id.sector();
    let open = profile.progress.is_unlocked(id);
    let record = profile.progress.record(id);
    let eyebrow = [
        Arg::Text(TextId::ChapterName(level.chapter)),
        Arg::Sector(id),
    ];
    let hue = sector_color(0, level.chapter);
    let top = page.y;
    page.line(
        (TextId::ReadyEyebrow, &eyebrow),
        Role::Label.into(),
        (page.w, false),
        hue,
    );
    let name_colour = if open { INK } else { MUTED };
    page.line(
        (TextId::SectorName(id), &[]),
        Role::Title.into(),
        (page.w, false),
        name_colour,
    );
    // The board, a brick to a pair of pixels.
    for cell in FieldCell::all() {
        if level.layout.hp[cell.index()] == 0 {
            continue;
        }
        let colour = if !open {
            hex(0x2a3142)
        } else if level.layout.cores.contains(cell) {
            AMBER
        } else {
            sector_color(cell.row(), level.chapter)
        };
        // The board is a map and never flips; in Arabic it sits at the right.
        let board = (3.0 * ark::field::COLS as f32 - 1.0) * px;
        let x = v.mirror(page.span(), page.x, board) + cell.col() as f32 * 3.0 * px;
        v.rect(
            x,
            page.y + cell.row() as f32 * 2.0 * px,
            2.0 * px,
            px,
            colour,
        );
    }
    page.gap(7.0 * 2.0 + LEAD);
    if open {
        medal_line(&mut page, record.medals);
        let times = [
            Arg::Clock(level.par_seconds),
            Arg::Clock(record.best_ticks / TICK_HZ),
        ];
        if record.best_ticks > 0 {
            page.line(
                (TextId::TargetBest, &times),
                Role::Caption.into(),
                (page.w, false),
                DIM,
            );
        }
        page.gap(2.0);
        let r = row(
            &mut page,
            (TextId::PlaySector, &[Arg::Sector(id)]),
            (true, true),
        );
        v.hits.borrow_mut().play = Some(r);
    } else {
        let before = [Arg::Sector(SectorId::clamped(id.index().saturating_sub(1)))];
        page.paragraph((TextId::UnlockHint, &before), Role::Caption.into(), 2, DIM);
    }
    // The whole page answers the pointer as its sector's card.
    let card = Rect::new(page.x, top, page.w, page.y - top);
    v.hits.borrow_mut().begin(List::Sectors);
    for other in SectorId::all() {
        let r = if other == id { card } else { Rect::default() };
        v.hits.borrow_mut().push(r);
    }
    // Where this page is among the twelve, between the ways to turn it.
    let foot = v.snap(BOTTOM - INSET * px - 7.0 * px);
    let place = Figures::of(|f| write!(f, "{:02}/{:02}", id.index() + 1, SECTOR_COUNT));
    let w = v.measure(place.as_str(), Role::Figure);
    let mid = foot + 3.5 * px;
    let left = chips::width(v, Prompt::Left, 0.0);
    let x = v.snap(WIDTH / 2.0 - w / 2.0);
    chips::chip(
        v,
        Prompt::Left,
        (x - left - 4.0 * px, mid),
        0.0,
        Lit::Neutral,
    );
    chips::chip(v, Prompt::Right, (x + w + 4.0 * px, mid), 0.0, Lit::Neutral);
    v.put(
        place.as_str(),
        Role::Figure,
        Slot::left(x, w, foot + 7.0 * px),
        DIM,
    );
}

use crate::{input::Device, render::WIDTH};
use ark::{SectorSummary, Stage, sectors::SectorId};
/// The most rows any menu has.
const MAX_ROWS: usize = 8;
use macroquad::prelude::{Rect, Vec2};
#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Title,
    Sectors,
    Play,
}
#[derive(Clone)]
pub struct Ui {
    pub screen: Screen,
    pub paused: bool,
    pub choice: usize,
    /// The cursor on the sector grid.
    pub sector: SectorId,
    pub save_error: bool,
    /// What the player touched last; prompts and the cursor follow it.
    pub device: Device,
    /// A stage to draw instead of the game's own, so the smoke test can
    /// capture screens that play would take minutes to reach.
    pub preview: Option<Preview>,
    /// The sheet on screen, and for how many seconds it has been open: it
    /// fades and rises in over its first moments.
    pub sheet: Option<Sheet>,
    pub sheet_open: f32,
}

/// A modal card over the field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    Pause,
    Cleared,
    GameOver,
    Victory,
}
impl Ui {
    /// The sheet this state shows over `stage`, if any.
    pub fn sheet_for(&self, stage: Stage) -> Option<Sheet> {
        if self.screen != Screen::Play {
            return None;
        }
        if self.paused {
            return Some(Sheet::Pause);
        }
        match stage {
            Stage::Cleared => Some(Sheet::Cleared),
            Stage::GameOver => Some(Sheet::GameOver),
            Stage::Victory => Some(Sheet::Victory),
            Stage::Ready | Stage::Playing => None,
        }
    }
    /// Keeps the open sheet's age; a different sheet starts from zero.
    pub fn track_sheet(&mut self, stage: Stage, seconds: f32) {
        let sheet = self.sheet_for(stage);
        if sheet == self.sheet {
            self.sheet_open += seconds;
        } else {
            self.sheet = sheet;
            self.sheet_open = 0.0;
        }
    }
}

/// Where the last frame drew what the pointer can hit, so a click lands
/// on what the player saw, wherever the fit chain put it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hits {
    /// The menu `rows` belong to.
    pub menu: Option<Menu>,
    rows: [Rect; MAX_ROWS],
    count: usize,
    /// A sheet's back glyph.
    pub back: Option<Rect>,
}
impl Hits {
    pub fn push(&mut self, row: Rect) {
        if let Some(slot) = self.rows.get_mut(self.count) {
            *slot = row;
            self.count += 1;
        }
    }
    pub fn rows(&self) -> &[Rect] {
        &self.rows[..self.count]
    }
    /// The row of `menu` under `p`, when `menu` is the one drawn.
    pub fn row_at(&self, menu: Menu, p: Vec2) -> Option<usize> {
        if self.menu != Some(menu) {
            return None;
        }
        self.rows().iter().position(|r| r.contains(p))
    }
    pub fn back_at(&self, p: Vec2) -> bool {
        self.back.is_some_and(|r| r.contains(p))
    }
}

/// A stage, and the results card to show with it, drawn in place of the
/// game's own.
#[derive(Clone, Copy)]
pub struct Preview {
    pub stage: Stage,
    pub summary: SectorSummary,
}
impl Default for Ui {
    fn default() -> Self {
        Self {
            screen: Screen::Title,
            paused: false,
            choice: 0,
            sector: SectorId::FIRST,
            save_error: false,
            device: Device::KeyboardMouse,
            preview: None,
            sheet: None,
            // Tests and captures draw sheets already in place.
            sheet_open: f32::INFINITY,
        }
    }
}
/// What a menu row does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Continue,
    NewJourney,
    Sectors,
    Resume,
    Retry,
    MainMenu,
    /// On to the next sector, or back to the sectors after practice.
    Next,
}

/// A list of actions. The first row is the screen's primary action and
/// takes the focus when the menu opens. A menu lists only what can be done
/// now, so rows, hit areas and focus steps all come from `actions`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Menu {
    pub actions: &'static [Action],
}

/// The title's menu column: its width, the height of a one-line row and
/// of the Continue row with its caption, and the space between rows.
pub const MENU_WIDTH: f32 = 432.0;
pub const ROW: f32 = 48.0;
pub const TALL_ROW: f32 = 64.0;
pub const ROW_GAP: f32 = 8.0;
/// The top of the title menu's first row.
pub const TITLE_TOP: f32 = 352.0;

impl Menu {
    fn height(&self, row: usize) -> f32 {
        if self.actions[row] == Action::Continue {
            TALL_ROW
        } else {
            ROW
        }
    }
    /// A row of the menu laid out as a centred column from `top`.
    pub fn column(&self, top: f32, row: usize) -> Rect {
        let y = top + (0..row).map(|r| self.height(r) + ROW_GAP).sum::<f32>();
        Rect::new(
            WIDTH / 2.0 - MENU_WIDTH / 2.0,
            y,
            MENU_WIDTH,
            self.height(row),
        )
    }
    /// Moves the focus one row, wrapping at either end.
    pub fn step(&self, choice: usize, up: bool, down: bool) -> usize {
        let last = self.actions.len() - 1;
        match (up, down) {
            (true, false) if choice == 0 => last,
            (true, false) => choice - 1,
            (false, true) if choice >= last => 0,
            (false, true) => choice + 1,
            _ => choice.min(last),
        }
    }
    pub fn action(&self, choice: usize) -> Action {
        self.actions[choice.min(self.actions.len() - 1)]
    }
}

/// The title menu: Continue only when there is a journey to continue.
pub fn title_menu(saved: bool) -> Menu {
    use Action::*;
    Menu {
        actions: if saved {
            &[Continue, NewJourney, Sectors]
        } else {
            &[NewJourney, Sectors]
        },
    }
}
pub fn pause_menu() -> Menu {
    use Action::*;
    Menu {
        actions: &[Resume, Retry, MainMenu],
    }
}
/// After the last life or the last sector.
pub fn result_menu(victory: bool) -> Menu {
    use Action::*;
    Menu {
        actions: if victory {
            &[NewJourney, Sectors, MainMenu]
        } else {
            &[Retry, Sectors, MainMenu]
        },
    }
}
/// The single action on the sector-clear card.
pub fn cleared_menu() -> Menu {
    Menu {
        actions: &[Action::Next],
    }
}
pub fn back_rect() -> Rect {
    Rect::new(56.0, 46.0, 150.0, 36.0)
}
/// The panel under the sector grid that describes the selected sector.
pub fn detail_rect() -> Rect {
    Rect::new(96.0, 520.0, 768.0, 296.0)
}
/// The panel's padding, and the width of its action column.
pub const DETAIL_PAD: f32 = 32.0;
pub const DETAIL_ACTION: f32 = 288.0;
/// Play, in the panel's action column, above two lines of footnote.
pub fn play_rect() -> Rect {
    let panel = detail_rect();
    Rect::new(
        panel.x + panel.w - DETAIL_PAD - DETAIL_ACTION,
        panel.y + 176.0,
        DETAIL_ACTION,
        ROW,
    )
}
pub fn sector_rect(index: usize) -> Rect {
    Rect::new(
        96.0 + (index / 4) as f32 * 260.0,
        152.0 + (index % 4) as f32 * 88.0,
        248.0,
        80.0,
    )
}
pub fn hover_sector(mouse: Vec2) -> Option<SectorId> {
    SectorId::all().find(|s| sector_rect(s.index()).contains(mouse))
}

#[derive(Default)]
pub struct Controls {
    pub click: bool,
    pub confirm: bool,
    pub escape: bool,
    pub pause: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub restart: bool,
    pub focus_lost: bool,
}
impl Controls {
    pub fn read() -> Self {
        use macroquad::prelude::*;
        let escape = is_key_pressed(KeyCode::Escape);
        Self {
            click: is_mouse_button_pressed(MouseButton::Left),
            confirm: is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space),
            escape,
            pause: escape || is_key_pressed(KeyCode::P),
            up: is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W),
            down: is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S),
            left: is_key_pressed(KeyCode::Left),
            right: is_key_pressed(KeyCode::Right),
            restart: is_key_pressed(KeyCode::R),
            focus_lost: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menus_offer_only_what_can_be_done() {
        use Action::*;
        assert_eq!(title_menu(false).actions, [NewJourney, Sectors]);
        assert_eq!(title_menu(true).actions, [Continue, NewJourney, Sectors]);
        assert_eq!(pause_menu().action(0), Resume);
        // After a run, the way back in comes first and takes the focus.
        assert_eq!(result_menu(false).actions, [Retry, Sectors, MainMenu]);
        assert_eq!(result_menu(true).actions, [NewJourney, Sectors, MainMenu]);
    }
    #[test]
    fn menu_steps_wrap_over_the_rows_shown() {
        let two = title_menu(false);
        assert_eq!(two.step(0, true, false), 1);
        assert_eq!(two.step(1, false, true), 0);
        assert_eq!(two.step(0, false, true), 1);
        let three = pause_menu();
        assert_eq!(three.step(0, true, false), 2);
        assert_eq!(three.step(2, false, true), 0);
        assert_eq!(three.step(1, false, false), 1);
        // A focus left over from a longer menu lands on the last row.
        assert_eq!(two.step(2, false, false), 1);
    }
    #[test]
    fn rows_are_hit_only_for_the_menu_drawn() {
        let mut hits = Hits {
            menu: Some(pause_menu()),
            ..Hits::default()
        };
        let menu = pause_menu();
        for row in 0..menu.actions.len() {
            hits.push(menu.column(100.0, row));
        }
        for row in 0..menu.actions.len() {
            let r = menu.column(100.0, row);
            assert_eq!(hits.row_at(menu, r.center()), Some(row));
        }
        let below = menu.column(100.0, menu.actions.len() - 1);
        assert_eq!(
            hits.row_at(menu, Vec2::new(480.0, below.bottom() + 1.0)),
            None
        );
        // A stale frame's rows never answer for another menu.
        assert_eq!(
            hits.row_at(title_menu(false), menu.column(100.0, 0).center()),
            None
        );
    }
    #[test]
    fn a_new_sheet_opens_from_the_start() {
        let mut ui = Ui {
            screen: Screen::Play,
            ..Ui::default()
        };
        ui.track_sheet(Stage::Playing, 0.5);
        assert_eq!(ui.sheet, None);
        ui.paused = true;
        ui.track_sheet(Stage::Playing, 0.5);
        assert_eq!((ui.sheet, ui.sheet_open), (Some(Sheet::Pause), 0.0));
        ui.track_sheet(Stage::Playing, 0.1);
        assert_eq!(ui.sheet_open, 0.1);
        ui.paused = false;
        ui.track_sheet(Stage::GameOver, 0.1);
        assert_eq!((ui.sheet, ui.sheet_open), (Some(Sheet::GameOver), 0.0));
    }
    #[test]
    fn play_sits_in_the_detail_panel_below_the_grid() {
        let panel = detail_rect();
        let play = play_rect();
        assert!(play.x >= panel.x + DETAIL_PAD && play.right() <= panel.right() - DETAIL_PAD);
        assert!(play.y > panel.y && play.bottom() < panel.bottom() - DETAIL_PAD);
        for s in SectorId::all() {
            let card = sector_rect(s.index());
            assert!(card.bottom() < panel.y, "{s:?} overlaps the panel");
            assert_eq!(hover_sector(card.center()), Some(s));
        }
    }
}

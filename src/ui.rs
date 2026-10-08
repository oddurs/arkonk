use crate::input::Device;
use ark::{
    SectorSummary, Stage,
    sectors::{SECTOR_COUNT, SectorId},
};
/// The most rows any list has: sector select's, a card per sector.
const MAX_ROWS: usize = SECTOR_COUNT;
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
    /// Seconds left for news in the band (the save failure).
    pub notice: f32,
    /// What the player touched last; prompts and the cursor follow it.
    pub device: Device,
    /// The mouse, not the keyboard, has been driving the paddle, so field
    /// prompts show the mouse.
    pub mouse: bool,
    /// Glyphs whose input just fired, to flash them pressed.
    pub pressed: Pressed,
    /// A stage to draw instead of the game's own, so the smoke test can
    /// capture screens that play would take minutes to reach.
    pub preview: Option<Preview>,
    /// The sheet on screen, and for how many seconds it has been open: it
    /// fades and rises in over its first moments.
    pub sheet: Option<Sheet>,
    pub sheet_open: f32,
    /// The Settings sheet is open over the title or the pause sheet, with
    /// the focus on this row of [`crate::settings::ROWS`].
    pub settings: Option<usize>,
}

/// What an input glyph stands for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prompt {
    Confirm,
    Back,
    Left,
    Right,
    /// Serve or release a held ball: Space, the mouse, or the south button.
    Serve,
}

/// How long each prompt's glyph has left to show pressed.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pressed([f32; 5]);
impl Pressed {
    /// A glyph flashes pressed this long when its input fires.
    const FLASH: f32 = 0.12;
    fn index(prompt: Prompt) -> usize {
        prompt as usize
    }
    pub fn fire(&mut self, prompt: Prompt) {
        self.0[Self::index(prompt)] = Self::FLASH;
    }
    pub fn is(&self, prompt: Prompt) -> bool {
        self.0[Self::index(prompt)] > 0.0
    }
    pub fn tick(&mut self, seconds: f32) {
        for t in &mut self.0 {
            *t = (*t - seconds).max(0.0);
        }
    }
}

/// A modal card over the field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    Settings,
    Pause,
    Cleared,
    GameOver,
    Victory,
}
impl Ui {
    /// The sheet this state shows over `stage`, if any.
    pub fn sheet_for(&self, stage: Stage) -> Option<Sheet> {
        if self.settings.is_some() {
            return Some(Sheet::Settings);
        }
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hits {
    /// The list `rows` belong to.
    pub list: Option<List>,
    rows: [Rect; MAX_ROWS],
    count: usize,
    /// A sheet's or the band's back glyph.
    pub back: Option<Rect>,
    /// The sector detail's Play action.
    pub play: Option<Rect>,
}
impl Default for Hits {
    fn default() -> Self {
        Self {
            list: None,
            rows: [Rect::default(); MAX_ROWS],
            count: 0,
            back: None,
            play: None,
        }
    }
}
impl Hits {
    /// Starts the rows of `list`, dropping any drawn under it: a sheet
    /// over the title must not answer with the title's rows.
    pub fn begin(&mut self, list: impl Into<List>) {
        self.list = Some(list.into());
        self.count = 0;
    }
    pub fn push(&mut self, row: Rect) {
        if let Some(slot) = self.rows.get_mut(self.count) {
            *slot = row;
            self.count += 1;
        }
    }
    pub fn rows(&self) -> &[Rect] {
        &self.rows[..self.count]
    }
    /// The row of `list` under `p`, when `list` is the one drawn.
    pub fn row_at(&self, list: impl Into<List>, p: Vec2) -> Option<usize> {
        if self.list != Some(list.into()) {
            return None;
        }
        self.rows().iter().position(|r| r.contains(p))
    }
    pub fn back_at(&self, p: Vec2) -> bool {
        self.back.is_some_and(|r| r.contains(p))
    }
    pub fn play_at(&self, p: Vec2) -> bool {
        self.play.is_some_and(|r| r.contains(p))
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
            notice: 0.0,
            device: Device::KeyboardMouse,
            mouse: false,
            pressed: Pressed::default(),
            preview: None,
            sheet: None,
            // Tests and captures draw sheets already in place.
            sheet_open: f32::INFINITY,
            settings: None,
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
    Settings,
}

/// A list of rows the pointer can hit: a menu of actions, or the
/// Settings sheet's rows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum List {
    Menu(Menu),
    Settings,
    /// The sector cards, one row per sector in order.
    Sectors,
}
impl From<Menu> for List {
    fn from(menu: Menu) -> Self {
        List::Menu(menu)
    }
}

/// A list of actions. The first row is the screen's primary action and
/// takes the focus when the menu opens. A menu lists only what can be done
/// now, so rows, hit areas and focus steps all come from `actions`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Menu {
    pub actions: &'static [Action],
}

/// The top of the title menu's first row.
pub const TITLE_TOP: f32 = 392.0;

impl Menu {
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
            &[Continue, NewJourney, Sectors, Settings]
        } else {
            &[NewJourney, Sectors, Settings]
        },
    }
}
pub fn pause_menu() -> Menu {
    use Action::*;
    Menu {
        actions: &[Resume, Retry, Settings, MainMenu],
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
        assert_eq!(title_menu(false).actions, [NewJourney, Sectors, Settings]);
        assert_eq!(
            title_menu(true).actions,
            [Continue, NewJourney, Sectors, Settings]
        );
        assert_eq!(pause_menu().action(0), Resume);
        // After a run, the way back in comes first and takes the focus.
        assert_eq!(result_menu(false).actions, [Retry, Sectors, MainMenu]);
        assert_eq!(result_menu(true).actions, [NewJourney, Sectors, MainMenu]);
    }
    #[test]
    fn menu_steps_wrap_over_the_rows_shown() {
        let three = title_menu(false);
        assert_eq!(three.step(0, true, false), 2);
        assert_eq!(three.step(2, false, true), 0);
        assert_eq!(three.step(0, false, true), 1);
        let four = pause_menu();
        assert_eq!(four.step(0, true, false), 3);
        assert_eq!(four.step(3, false, true), 0);
        assert_eq!(four.step(1, false, false), 1);
        // A focus left over from a longer menu lands on the last row.
        assert_eq!(three.step(3, false, false), 2);
    }
    #[test]
    fn rows_are_hit_only_for_the_menu_drawn() {
        let mut hits = Hits {
            list: Some(pause_menu().into()),
            ..Hits::default()
        };
        let menu = pause_menu();
        let row = |i: usize| Rect::new(280.0, 100.0 + i as f32 * 56.0, 400.0, 48.0);
        for i in 0..menu.actions.len() {
            hits.push(row(i));
        }
        for i in 0..menu.actions.len() {
            assert_eq!(hits.row_at(menu, row(i).center()), Some(i));
        }
        let below = row(menu.actions.len() - 1);
        assert_eq!(
            hits.row_at(menu, Vec2::new(480.0, below.bottom() + 1.0)),
            None
        );
        // A stale frame's rows never answer for another menu.
        assert_eq!(hits.row_at(title_menu(false), row(0).center()), None);
    }
    #[test]
    fn a_list_drawn_over_another_starts_its_rows_afresh() {
        let mut hits = Hits::default();
        hits.begin(title_menu(false));
        let title_row = Rect::new(280.0, 400.0, 400.0, 48.0);
        hits.push(title_row);
        hits.begin(List::Settings);
        let setting = Rect::new(240.0, 200.0, 480.0, 48.0);
        hits.push(setting);
        // The title row under the sheet once answered as setting 0.
        assert_eq!(hits.rows(), [setting]);
        assert_eq!(hits.row_at(List::Settings, title_row.center()), None);
        assert_eq!(hits.row_at(List::Settings, setting.center()), Some(0));
    }
    #[test]
    fn every_sector_card_is_a_row() {
        let mut hits = Hits {
            list: Some(List::Sectors),
            ..Hits::default()
        };
        for i in 0..SECTOR_COUNT {
            hits.push(Rect::new(i as f32 * 10.0, 0.0, 10.0, 10.0));
        }
        // The last chapter's cards were dropped once the list held eight.
        assert_eq!(hits.row_at(List::Sectors, Vec2::new(115.0, 5.0)), Some(11));
    }
    #[test]
    fn a_pressed_glyph_flashes_briefly() {
        let mut p = Pressed::default();
        p.fire(Prompt::Back);
        assert!(p.is(Prompt::Back) && !p.is(Prompt::Confirm));
        p.tick(0.1);
        assert!(p.is(Prompt::Back));
        p.tick(0.05);
        assert!(!p.is(Prompt::Back));
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
}

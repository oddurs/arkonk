use crate::{input::Device, render::WIDTH};
use ark::{SectorSummary, Stage, sectors::SectorId};
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
}

/// A column of buttons. The first row is the screen's primary action and
/// takes the focus when the menu opens. A menu lists only what can be done
/// now, so rows, hit areas and focus steps all come from `actions`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Menu {
    pub actions: &'static [Action],
    /// Top of the first row, in scene units.
    pub top: f32,
}

/// Menu buttons: their width, the height of a one-line row, and the space
/// between rows. The Continue row is taller: it carries a caption line.
pub const MENU_WIDTH: f32 = 400.0;
pub const ROW: f32 = 48.0;
pub const TALL_ROW: f32 = 64.0;
pub const ROW_GAP: f32 = 8.0;

impl Menu {
    fn height(&self, row: usize) -> f32 {
        if self.actions[row] == Action::Continue {
            TALL_ROW
        } else {
            ROW
        }
    }
    pub fn rect(&self, row: usize) -> Rect {
        let y = self.top + (0..row).map(|r| self.height(r) + ROW_GAP).sum::<f32>();
        Rect::new(
            WIDTH / 2.0 - MENU_WIDTH / 2.0,
            y,
            MENU_WIDTH,
            self.height(row),
        )
    }
    pub fn hover(&self, mouse: Vec2) -> Option<usize> {
        (0..self.actions.len()).find(|&i| self.rect(i).contains(mouse))
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
        top: 380.0,
    }
}
pub fn pause_menu() -> Menu {
    use Action::*;
    Menu {
        actions: &[Resume, Retry, MainMenu],
        top: 380.0,
    }
}
/// After the last life or the last sector.
pub fn result_menu(victory: bool) -> Menu {
    use Action::*;
    Menu {
        actions: if victory {
            &[Sectors, NewJourney, MainMenu]
        } else {
            &[Sectors, Retry, MainMenu]
        },
        top: 380.0,
    }
}
/// The single action on the sector-clear card.
pub fn next_rect() -> Rect {
    Rect::new(310.0, 514.0, 340.0, 46.0)
}
pub fn back_rect() -> Rect {
    Rect::new(56.0, 46.0, 150.0, 36.0)
}
pub fn play_rect() -> Rect {
    Rect::new(310.0, 768.0, 340.0, 46.0)
}
pub fn sector_rect(index: usize) -> Rect {
    Rect::new(
        96.0 + (index / 4) as f32 * 260.0,
        196.0 + (index % 4) as f32 * 112.0,
        248.0,
        104.0,
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
    fn every_row_is_hit_at_its_centre_and_rows_never_overlap() {
        for menu in [
            title_menu(false),
            title_menu(true),
            pause_menu(),
            result_menu(true),
        ] {
            for row in 0..menu.actions.len() {
                let r = menu.rect(row);
                assert_eq!(menu.hover(r.center()), Some(row));
                if row > 0 {
                    let above = menu.rect(row - 1);
                    assert_eq!(r.y - (above.y + above.h), ROW_GAP);
                }
            }
            let below = menu.rect(menu.actions.len() - 1);
            assert_eq!(menu.hover(Vec2::new(480.0, below.y + below.h + 1.0)), None);
        }
        assert_eq!(title_menu(true).rect(0).h, TALL_ROW);
    }
}

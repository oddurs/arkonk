use macroquad::prelude::{Rect, Vec2};
#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Title,
    Sectors,
    Play,
}
pub struct Ui {
    pub screen: Screen,
    pub paused: bool,
    pub choice: usize,
    pub sector: usize,
    pub save_error: bool,
}
impl Default for Ui {
    fn default() -> Self {
        Self {
            screen: Screen::Title,
            paused: false,
            choice: 1,
            sector: 0,
            save_error: false,
        }
    }
}
pub fn menu_rect(row: usize) -> Rect {
    Rect::new(310.0, 380.0 + row as f32 * 56.0, 340.0, 46.0)
}
/// The single action on the sector-clear card.
pub fn next_rect() -> Rect {
    Rect::new(310.0, 524.0, 340.0, 46.0)
}
pub fn back_rect() -> Rect {
    Rect::new(64.0, 46.0, 104.0, 36.0)
}
pub fn play_rect() -> Rect {
    Rect::new(310.0, 768.0, 340.0, 46.0)
}
pub fn sector_rect(index: usize) -> Rect {
    Rect::new(
        105.0 + (index / 4) as f32 * 255.0,
        196.0 + (index % 4) as f32 * 118.0,
        240.0,
        104.0,
    )
}
pub fn hover_menu(mouse: Vec2) -> Option<usize> {
    (0..3).find(|&i| menu_rect(i).contains(mouse))
}
/// Moves through the three menu rows, wrapping, never landing below `first`.
pub fn step_menu(choice: usize, first: usize, up: bool, down: bool) -> usize {
    match (up, down) {
        (true, false) if choice <= first => 2,
        (true, false) => choice - 1,
        (false, true) if choice >= 2 => first,
        (false, true) => choice + 1,
        _ => choice,
    }
}
pub fn hover_sector(mouse: Vec2) -> Option<usize> {
    (0..12).find(|&i| sector_rect(i).contains(mouse))
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
    fn menu_steps_skip_unavailable_rows() {
        assert_eq!(step_menu(1, 1, true, false), 2);
        assert_eq!(step_menu(2, 1, false, true), 1);
        assert_eq!(step_menu(0, 0, true, false), 2);
        assert_eq!(step_menu(2, 0, false, true), 0);
        assert_eq!(step_menu(1, 0, false, false), 1);
    }
}

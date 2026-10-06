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
    Rect::new(280.0, 471.0 + row as f32 * 53.0, 400.0, 43.0)
}
pub fn sector_rect(index: usize) -> Rect {
    Rect::new(
        105.0 + (index / 4) as f32 * 255.0,
        239.0 + (index % 4) as f32 * 121.0,
        240.0,
        107.0,
    )
}
pub fn hover_menu(mouse: Vec2) -> Option<usize> {
    (0..3).find(|&i| menu_rect(i).contains(mouse))
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

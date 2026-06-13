use macroquad::color::Color;
use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{draw_rectangle, screen_height, screen_width};

pub struct TempInfo {
    pub resizing: bool,
}

struct Settings {
    robot_size: Vec2,
}

pub struct Right {
    pub width: f32,
    color: Color,
    pub(crate) settings: Settings,
    pub temp_info: TempInfo,
}

impl Right {
    // TODO: use this UI for the settings
    pub(crate) fn new(color: Color, width: f32) -> Self {
        Right {
            color,
            width,
            settings: Settings { robot_size: vec2(174.0, 160.0) },
            temp_info: TempInfo { resizing: false },
        }
    }

    pub fn render(&self) {
        draw_rectangle(screen_width() - self.width, 0.0, self.width, screen_height(), self.color);
    }

    pub fn resize_check(&mut self, tolerance: f32) {
        if (mouse_position().0 - screen_width() + self.width).abs() <= tolerance && is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            self.width = screen_width() - mouse_position().0;
            self.temp_info.resizing = true;
        } else if self.temp_info.resizing && is_mouse_button_down(macroquad::input::MouseButton::Left) {
            self.width = screen_width() - mouse_position().0
        } else {
            self.temp_info.resizing = false
        }
    }
}

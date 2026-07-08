use crate::rendering::left::PressedButton::Straight;
use crate::utils::load_texture_safe::load_texture_safe;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, is_mouse_button_pressed, mouse_position};
use macroquad::math::{Vec2, vec2};
use macroquad::prelude::{get_default_font, load_ttf_font_from_bytes};
use macroquad::window::screen_width;
use miniquad::KeyCode::F11;
use miniquad::MouseButton::Left;
use miniquad::window::set_fullscreen;

const FONT_DATA: &[u8] = include_bytes!("../../assets/font/Lexend-VariableFont_wght.ttf");
const FIELD_TEXTURE_BYTES: &[u8] = include_bytes!("../../assets/textures/field.png");

pub mod left;
pub mod middle;
pub mod right;

struct Settings {
    robot_size: Vec2,
}

struct TempInfo {
    is_fullscreen: bool,
    last_is_copy_code: bool,
}
pub(crate) struct Manager {
    pub(crate) left: left::Left,
    pub(crate) middle: middle::Middle,
    pub(crate) right: right::Right,
    settings: Settings,
    cursor: crate::utils::set_mouse_cursor::MouseCursor,
    temp_info: TempInfo,
}

impl Manager {
    pub(crate) fn new(drive_method_names: Vec<String>, color: Color) -> Self {
        let font = load_ttf_font_from_bytes(FONT_DATA).unwrap_or_else(|e| {
            eprintln!("Font failed to decode: {}. Use default font instead.", e);
            get_default_font()
        });
        Manager {
            left: left::Left::new(drive_method_names, 150.0, color, font),
            middle: middle::Middle::new(load_texture_safe(FIELD_TEXTURE_BYTES)),
            right: right::Right::new(color, 300.0),
            settings: Settings { robot_size: vec2(174.0, 160.0) },
            cursor: crate::utils::set_mouse_cursor::MouseCursor { state: miniquad::CursorIcon::Default },
            temp_info: TempInfo {
                is_fullscreen: false,
                last_is_copy_code: false,
            },
        }
    }

    pub(crate) fn render(&mut self, points: &[Vec2]) {
        self.middle.render(self.left.width, screen_width() - self.right.width, points, self.settings.robot_size);
        self.right.render();
        self.left.render();
    }

    pub(crate) fn resize_check(&mut self, tolerance: f32) {
        if (mouse_position().0 - self.left.width).abs() <= tolerance || (mouse_position().0 - screen_width() + self.right.width).abs() <= tolerance {
            self.cursor.set(miniquad::CursorIcon::EWResize);
        } else if !self.left.temp_info.resizing && !self.right.temp_info.resizing {
            self.cursor.set(miniquad::CursorIcon::Default);
        }

        self.left.resize_check(tolerance);
        self.right.resize_check(tolerance);
        if is_key_pressed(F11) {
            if self.temp_info.is_fullscreen {
                set_fullscreen(false);
                self.temp_info.is_fullscreen = false
            } else {
                set_fullscreen(true);
                self.temp_info.is_fullscreen = true
            }
        }
    }

    pub(crate) fn is_copy_code(&mut self) -> bool {
        let copy = self.left.is_copy_code();
        if copy && !self.temp_info.last_is_copy_code {
            self.temp_info.last_is_copy_code = true;
            true
        } else {
            if !copy && self.temp_info.last_is_copy_code {
                self.temp_info.last_is_copy_code = false;
            }
            false
        }
    }

    pub fn point_set_check(&mut self, points: &mut Vec<Vec2>) {
        if is_mouse_button_pressed(Left) && self.left.pressed_button == Straight {
            self.middle.point_set_check(points, self.left.width, screen_width() - self.right.width);
        }
    }
}

use crate::load_texture_safe::load_texture_safe;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, mouse_position};
use macroquad::prelude::{get_default_font, load_ttf_font_from_bytes};
use macroquad::window::screen_width;

const FONT_DATA: &[u8] = include_bytes!("../../assets/font/Lexend-VariableFont_wght.ttf");
const FIELD_TEXTURE_BYTES: &[u8] = include_bytes!("../../assets/textures/field.png");

pub mod left;
pub mod middle;
pub mod right;

pub(crate) struct Manager {
    left: left::Left,
    middle: middle::Middle,
    right: right::Right,
}

impl Manager {
    pub(crate) fn new(drive_method_names: Vec<String>, color: Color) -> Self {
        let font = load_ttf_font_from_bytes(FONT_DATA).unwrap_or_else(|e| {
            eprintln!("Font failed to decode: {}. Use default font instead.", e);
            get_default_font()
        });
        Manager {
            left: left::Left::new(drive_method_names, 150.0, color, font),
            middle: middle::Middle::new(vec![], load_texture_safe(FIELD_TEXTURE_BYTES)),
            right: right::Right::new(color, 300.0),
            cursor: crate::set_mouse_cursor::MouseCursor { state: miniquad::CursorIcon::Default },
            temp_info: TempInfo { is_fullscreen: false },
        }
    }

    pub(crate) fn render(&mut self) {
        self.middle.render(self.left.width, screen_width() - self.right.width);
        self.right.render();
        self.left.render();
    }

    pub(crate) fn resize_check(&mut self, tolerance: f32) {
        self.right.resize_check(tolerance);
        self.left.resize_check(tolerance);
    }
}

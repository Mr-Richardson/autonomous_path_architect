use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, screen_height, screen_width};

pub struct Right {
    color: Color,
    width: f32,
}

impl Right {
    pub(crate) fn new(color: Color, width: f32) -> Self {
        Right { color, width }
    }

    pub fn render(&self) {
        draw_rectangle(screen_width() - self.width, 0.0, screen_width(), screen_height(), self.color);
    }
}
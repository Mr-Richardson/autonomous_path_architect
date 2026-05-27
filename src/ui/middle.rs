use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_texture;
use macroquad::window::screen_height;

pub struct Middle {
    points: Vec<Vec2>,
    texture: macroquad::texture::Texture2D,
    color: Color,
    last_height: f32,
}

impl Middle {
    pub fn new(points: Vec<Vec2>, texture: macroquad::texture::Texture2D, color: Color) -> Self {
        Middle {
            points,
            texture,
            color,
            last_height: screen_height(),
        }
    }

    pub fn render(&self) {
        if self.last_height != screen_height() {
            draw_texture(&self.texture, 160.0, 0.0, WHITE);
        }
    }
}

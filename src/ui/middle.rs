use macroquad::color::{BLACK, Color, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_texture;
use macroquad::shapes::{draw_circle, draw_circle_lines};
use macroquad::window::screen_height;

pub struct Middle {
    points: Vec<Vec2>,
    texture: macroquad::texture::Texture2D,
    last_height: f32,
}

impl Middle {
    pub fn new(points: Vec<Vec2>, texture: macroquad::texture::Texture2D, color: Color) -> Self {
        Middle {
            points,
            texture,
            last_height: screen_height(),
        }
    }

    pub fn render(&self) {
        draw_texture(&self.texture, 160.0, 0.0, WHITE);
        for p in self.points.iter() {
            draw_circle(p.x, p.y, 5.0, WHITE);
            draw_circle_lines(p.x, p.y, 5.0, 2.0, BLACK);
        }
    }
}

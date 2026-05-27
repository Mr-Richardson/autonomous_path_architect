use macroquad::color::{BLACK, WHITE};
use macroquad::math::Vec2;
use macroquad::prelude::draw_texture;
use macroquad::shapes::{draw_circle, draw_circle_lines};

pub struct Middle {
    points: Vec<Vec2>,
    texture: macroquad::texture::Texture2D,
}

impl Middle {
    pub fn new(points: Vec<Vec2>, texture: macroquad::texture::Texture2D) -> Self {
        Middle { points, texture }
    }

    pub fn render(&self) {
        draw_texture(&self.texture, 160.0, 0.0, WHITE);
        for p in self.points.iter() {
            draw_circle(p.x, p.y, 5.0, WHITE);
            draw_circle_lines(p.x, p.y, 5.0, 2.0, BLACK);
        }
    }
}

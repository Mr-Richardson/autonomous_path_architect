use macroquad::color::{BLACK, WHITE};
use macroquad::math::{Vec2, vec2};
use macroquad::shapes::{draw_circle, draw_circle_lines};
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use macroquad::window::screen_height;

pub struct Middle {
    points: Vec<Vec2>,
    texture: macroquad::texture::Texture2D,
}

impl Middle {
    pub fn new(points: Vec<Vec2>, texture: macroquad::texture::Texture2D) -> Self {
        Middle { points, texture }
    }

    pub fn render(&self, x_start: f32, x_end: f32) {
        let x = self.texture.height() / self.texture.width();
        let h = (screen_height() - (x_end - x_start) * x) / 2.0;
        draw_texture_ex(
            &self.texture,
            x_start,
            h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(x_end - x_start, (x_end - x_start) * x)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        for p in self.points.iter() {
            draw_circle(p.x + x_start, p.y, 5.0, WHITE);
            draw_circle_lines(p.x + x_start, p.y, 5.0, 2.0, BLACK);
        }
    }
}

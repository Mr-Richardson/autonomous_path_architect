use macroquad::color::{BLACK, Color, WHITE};
use macroquad::math::{Vec2, vec2};
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use macroquad::window::screen_height;

pub struct Middle {
    texture: macroquad::texture::Texture2D,
}

impl Middle {
    pub fn new(texture: macroquad::texture::Texture2D) -> Self {
        Middle { texture }
    }

    pub fn render(&self, x_start: f32, x_end: f32, points: &[Vec2], robot_size: Vec2) {
        // FIXME: wide screens
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
        if points.len() != 0 {
            for i in 0..points.len() - 1 {
                draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, robot_size.x, Color::new(0.0, 0.0, 0.0, 0.5));
            }
        }
        for p in points.iter() {
            draw_circle(p.x + x_start, p.y, 5.0, WHITE);
            draw_circle_lines(p.x + x_start, p.y, 5.0, 2.0, BLACK);
        }
    }
}

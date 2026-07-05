use macroquad::color::{Color, BLACK, WHITE};
use macroquad::math::{vec2, Vec2};
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::window::screen_height;

pub struct Middle {
    texture: macroquad::texture::Texture2D,
    temp_info: TempInfo,
}

struct TempInfo {
    last_x_start: f32,
    last_x_end: f32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    aspect_ratio: f32,
}

impl Middle {
    pub fn new(texture: macroquad::texture::Texture2D) -> Self {
        let mut middle = Middle {
            texture,
            temp_info: TempInfo {
                last_x_start: 0.0,
                last_x_end: 0.0,
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
                aspect_ratio: 0.0,
            },
        };
        middle.temp_info.aspect_ratio = middle.texture.height() / middle.texture.width();
        middle
    }

    pub fn render(&self, x_start: f32, x_end: f32, points: &[Vec2], robot_size: Vec2) {
        let x: f32;
        let y: f32;
        let w: f32;
        let h: f32;
        let x_diff: f32 = x_end - x_start;
        if self.temp_info.aspect_ratio < screen_height() / x_diff {
            x = x_start;
            w = x_diff;
            h = x_diff * self.temp_info.aspect_ratio;
            y = (screen_height() - h) / 2.0;
        } else {
            y = 0.0;
            h = screen_height();
            w = screen_height() / self.temp_info.aspect_ratio;
            x = x_start + (x_diff - w) / 2.0;
        }
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
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

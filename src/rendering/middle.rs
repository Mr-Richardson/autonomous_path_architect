use macroquad::color::{BLACK, Color, WHITE};
use macroquad::input::mouse_position;
use macroquad::math::{Vec2, vec2};
use macroquad::shapes::{draw_circle, draw_circle_lines, draw_line};
use macroquad::texture::{DrawTextureParams, draw_texture_ex};
use macroquad::window::screen_height;

pub struct Middle {
    texture: macroquad::texture::Texture2D,
    temp_info: TempInfo,
}

struct TempInfo {
    last_dimensions: LastDimensions,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    aspect_ratio: f32,
}

#[derive(PartialEq)]
struct LastDimensions {
    last_x_start: f32,
    last_x_end: f32,
    last_screen_height: f32,
}

impl Middle {
    pub fn new(texture: macroquad::texture::Texture2D) -> Self {
        let mut middle = Middle {
            texture,
            temp_info: TempInfo {
                last_dimensions: LastDimensions {
                    last_x_start: 0.0,
                    last_x_end: 0.0,
                    last_screen_height: 0.0,
                },
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

    fn update(&mut self, x_start: f32, x_end: f32) {
        let x: f32;
        let y: f32;
        let w: f32;
        let h: f32;
        let dimensions = LastDimensions {
            last_x_start: x_start,
            last_x_end: x_end,
            last_screen_height: screen_height(),
        };
        if self.temp_info.last_dimensions != dimensions {
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
            self.temp_info.x = x;
            self.temp_info.y = y;
            self.temp_info.w = w;
            self.temp_info.h = h;
            self.temp_info.last_dimensions = dimensions;
        }
    }

    pub fn render(&mut self, x_start: f32, x_end: f32, points: &[Vec2], robot_size: Vec2) {
        self.update(x_start, x_end);
        draw_texture_ex(
            &self.texture,
            self.temp_info.x,
            self.temp_info.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.temp_info.w, self.temp_info.h)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        if !points.is_empty() {
            for i in 0..points.len() - 1 {
                draw_line(points[i].x, points[i].y, points[i + 1].x, points[i + 1].y, robot_size.x, Color::new(0.0, 0.0, 0.0, 0.5));
            }
        }
        for p in points.iter() {
            let x = p.x * self.temp_info.w + self.temp_info.x;
            let y = p.y * self.temp_info.w + self.temp_info.y;
            draw_circle(x, y, 5.0, WHITE);
            draw_circle_lines(x, y, 5.0, 2.0, BLACK);
        }
    }

    pub fn point_set_check(&mut self, points: &mut Vec<Vec2>, x_start: f32, x_end: f32) {
        self.update(x_start, x_end);
        if mouse_position().0 > self.temp_info.x && self.temp_info.x + self.temp_info.w > mouse_position().0 && mouse_position().1 > self.temp_info.y && self.temp_info.y + self.temp_info.h > mouse_position().1 {
            points.push(vec2((mouse_position().0 - self.temp_info.x) / self.temp_info.w, ((mouse_position().1) - self.temp_info.y) / self.temp_info.w))
        }
    }
}

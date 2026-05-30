use button_lib::button::Button;
use button_lib::button::Shape::Ellipse;
use button_lib::button::State::{Disabled, Hovered, Idle, Pressed};
use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::prelude::{Font, screen_height};
use macroquad::shapes::draw_rectangle;

#[derive(PartialEq)]
struct LastSize {
    width: f32,
    height: f32,
    size_multi: Vec<f32>,
}

pub struct Left {
    buttons: Vec<Button>,
    pub width: f32,
    color: Color,
    last_size: LastSize,
}

impl Left {
    pub fn new(button_names: Vec<String>, width: f32, color: Color, font: Font) -> Self {
        let mut buttons = Vec::new();
        for name in button_names {
            buttons.push(Button::new(
                vec2(0.0, 0.0),
                vec2(0.0, 0.0),
                Ellipse,
                Color {
                    r: 1.0 - color.r,
                    g: 1.0 - color.g,
                    b: 1.0 - color.b,
                    a: 1.0,
                },
                false,
                button_lib::button::Text {
                    text: name.to_string(),
                    font: font.clone(),
                    size: 20,
                    color: macroquad::color::BLACK,
                },
            ));
        }
        let mut left = Left {
            buttons,
            width,
            color,
            last_size: LastSize {
                width: 0.0,
                height: 0.0,
                size_multi: Vec::new(),
            },
        };
        left.update();
        left
    }

    pub fn render(&mut self) {
        self.update();
        draw_rectangle(0.0, 0.0, self.width, screen_height(), self.color);
        for b in &self.buttons {
            b.render()
        }
    }

    fn update(&mut self) {
        let new_size: LastSize = LastSize {
            width: self.width,
            height: screen_height(),
            size_multi: self
                .buttons
                .iter_mut()
                .map(|b| match b.get_state() {
                    Idle => 1.5,
                    Pressed => 1.75,
                    Hovered => 1.8,
                    Disabled => 0.0,
                })
                .collect(),
        };
        if self.last_size != new_size {
            let pos_y = (self.width).min(screen_height() / (self.buttons.len()) as f32) / 2.0;
            for (i, b) in self.buttons.iter_mut().enumerate() {
                b.set_pos(vec2(self.width / 2.0, pos_y * (i as f32 + 0.5) * 2.0));
                b.set_size(vec2(pos_y * new_size.size_multi[i], pos_y * new_size.size_multi[i]));
            }
            self.last_size = new_size
        }
    }

    pub fn resize_check(&mut self, tolerance: f32) {}

    pub fn set_width(&mut self, width: f32) {
        self.width = width.min(screen_height() / self.buttons.len() as f32);
        self.update();
    }
}

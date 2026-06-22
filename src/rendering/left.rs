use button_lib::button::Button;
use button_lib::button::Shape::{Ellipse, Rectangle};
use button_lib::button::State::{Disabled, Hovered, Idle, Pressed};
use macroquad::color::{Color, RED, WHITE};
use macroquad::input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position};
use macroquad::math::vec2;
use macroquad::prelude::{Font, screen_height};
use macroquad::shapes::draw_rectangle;

#[derive(PartialEq)]
struct LastSize {
    width: f32,
    height: f32,
    size_multi: Vec<f32>,
}

pub struct TempInfo {
    pub resizing: bool,
    last_size: LastSize,
}

pub struct Left {
    pub width: f32,
    color: Color,
    buttons: Vec<Button>,
    pub temp_info: TempInfo,
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
                true,
                button_lib::button::Text {
                    text: name.to_string(),
                    font: font.clone(),
                    size: 20,
                    color: macroquad::color::BLACK,
                },
            ));
        }
        buttons.push(Button::new(
            vec2(0.0, 0.0),
            vec2(0.0, 0.0),
            Rectangle,
            Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
            false,
            button_lib::button::Text {
                text: "copy code".to_string(),
                font: font.clone(),
                size: 20,
                color: WHITE,
            },
        ));
        let mut left = Left {
            buttons,
            width,
            color,
            temp_info: TempInfo {
                resizing: false,
                last_size: LastSize {
                    width: 0.0,
                    height: 0.0,
                    size_multi: Vec::new(),
                },
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
                    Pressed => 1.45,
                    Hovered => 1.55,
                    Disabled => 0.0,
                })
                .collect(),
        };
        if self.temp_info.last_size != new_size {
            let pos_y = self.width.min(screen_height() / self.buttons.len() as f32) / 2.0;
            for (i, b) in self.buttons.iter_mut().enumerate() {
                b.set_pos(vec2(self.width / 2.0, pos_y * (i as f32 + 0.5) * 2.0));
                b.set_size(vec2(pos_y * new_size.size_multi[i], pos_y * new_size.size_multi[i]));
                if i != new_size.size_multi.len() - 1 {
                    if new_size.size_multi[i] == 1.45 {
                        b.set_color(RED);
                    } else {
                        b.set_color(WHITE);
                    }
                }
            }
            self.temp_info.last_size = new_size
        }
    }

    pub fn resize_check(&mut self, tolerance: f32) {
        if (mouse_position().0 - self.width).abs() <= tolerance && is_mouse_button_pressed(macroquad::input::MouseButton::Left) {
            self.width = mouse_position().0;
            self.temp_info.resizing = true;
        } else if self.temp_info.resizing && is_mouse_button_down(macroquad::input::MouseButton::Left) {
            self.width = mouse_position().0
        } else {
            self.temp_info.resizing = false
        }
    }

    pub(crate) fn is_copy_code(&mut self) -> bool {
        if let Some(button) = self.buttons.last_mut() { button.get_state() == Pressed } else { false }
    }
}

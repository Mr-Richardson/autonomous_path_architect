use button_lib::Button;
use button_lib::Shape::Ellipse;
use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::prelude::screen_height;
use macroquad::shapes::draw_rectangle;

pub struct Left {
    buttons: Vec<Button>,
    color: Color,
    last_height: f32,
}

impl Left {
    pub fn new(button_names: Vec<String>, color: Color) -> Self {
        let mut buttons = Vec::new();
        for (i, name) in button_names.iter().enumerate() {
            buttons.push(Button::new(
                vec2(
                    screen_height() / button_names.len() as f32 / 2.0,
                    screen_height() / button_names.len() as f32 * (0.5 + i as f32),
                ),
                vec2(
                    screen_height() / button_names.len() as f32 * 0.8,
                    screen_height() / button_names.len() as f32 * 0.8,
                ),
                Ellipse,
                Color {
                    r: 1.0 - color.r,
                    g: 1.0 - color.g,
                    b: 1.0 - color.b,
                    a: 1.0,
                },
                name.to_string(),
                0.9,
                false,
            ));
        }
        Left {
            buttons,
            color,
            last_height: screen_height(),
        }
    }

    pub fn render(&mut self) {
        if self.last_height != screen_height() {
            self.last_height = screen_height();
            let number_buttons: f32 = self.buttons.len() as f32;
            for (i, b) in self.buttons.iter_mut().enumerate() {
                b.set_pos(vec2(
                    screen_height() / number_buttons / 2.0,
                    screen_height() / number_buttons * (0.5 + i as f32),
                ));
                b.set_size(vec2(
                    screen_height() / number_buttons * 0.8,
                    screen_height() / number_buttons * 0.8,
                ));
                println!("Updated button");
            }
        }
        draw_rectangle(
            0.0,
            0.0,
            screen_height() / self.buttons.len() as f32,
            screen_height(),
            self.color,
        );
        for b in &self.buttons {
            b.render()
        }
    }
}

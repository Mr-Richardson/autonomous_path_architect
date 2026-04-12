mod ui;

use crate::ui::left::Left;
use crate::ui::middle::Middle;
use crate::ui::right::Right;
use crate::ui::Manager;
use macroquad::color::Color;
use macroquad::math::vec2;
use macroquad::prelude::load_texture;
use macroquad::window::next_frame;

#[macroquad::main("Path Planning Engine")]
async fn main() {
    let mut ui: Manager = Manager::new(
        Left::new(
            vec!["straight".to_string(), "turn".to_string(), "arc".to_string(), "drive".to_string()],
            Color::new(0.2, 0.2, 0.2, 1.0),
        ),
        Middle::new(
            vec![vec2(10.0, 90.0)],
            load_texture("test.png").await.unwrap(),
            Color::new(0.2, 0.2, 0.2, 1.0),
        ),
        Right::new(
            Color::new(0.2, 0.2, 0.2, 1.0),
            300.0,
        ),
    );
    loop {
        ui.render();
        next_frame().await;
    }
}

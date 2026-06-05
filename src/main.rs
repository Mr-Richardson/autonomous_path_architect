mod load_texture_safe;
pub mod set_mouse_cursor;
mod ui;

use crate::ui::Manager;
use macroquad::color::Color;
use macroquad::window::{Conf, next_frame};

fn window_conf() -> Conf {
    Conf {
        window_title: "Autonomous Path Architect".to_string(),
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ui: Manager = Manager::new(vec!["straight".to_string(), "turn".to_string(), "arc".to_string(), "drive".to_string()], Color::new(0.1, 0.1, 0.1, 1.0));

    loop {
        ui.resize_check(4.0);
        ui.render();
        next_frame().await;
    }
}

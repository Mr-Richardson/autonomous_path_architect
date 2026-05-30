mod load_texture_safe;
mod ui;

use crate::load_texture_safe::load_texture_safe;
use crate::ui::Manager;
use crate::ui::left::Left;
use crate::ui::middle::Middle;
use crate::ui::right::Right;
use macroquad::color::Color;
use macroquad::prelude::load_ttf_font;
use macroquad::text::get_default_font;
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
    let font = match load_ttf_font("assets/font/Lexend-VariableFont_wght.ttf").await {
        Ok(loaded_font) => loaded_font,
        Err(e) => {
            eprintln!("Font failed to load: {}. Use default font instead.", e);
            get_default_font()
        }
    };

    let mut ui: Manager = Manager::new(
        Left::new(vec!["straight".to_string(), "turn".to_string(), "arc".to_string(), "drive".to_string()], 150.0, Color::new(0.1, 0.1, 0.1, 1.0), font),
        Middle::new(vec![], load_texture_safe("assets/textures/field.png").await),
        Right::new(Color::new(0.1, 0.1, 0.1, 1.0), 300.0),
    );
    loop {
        ui.resize_check(5.0);
        ui.render();
        next_frame().await;
    }
}

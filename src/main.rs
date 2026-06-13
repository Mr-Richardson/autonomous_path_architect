#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logic;
mod rendering;
pub mod utils;

use crate::rendering::Manager;
use macroquad::color::Color;
use macroquad::window::{Conf, clear_background, next_frame};

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
    let mut ui: Manager = Manager::new(vec!["straight".to_string(), "arc".to_string(), "until black".to_string()], Color::new(0.1, 0.1, 0.1, 1.0));
    println!("Program started");
    loop {
        clear_background(macroquad::color::BLACK);
        ui.resize_check(4.0);
        ui.render();
        next_frame().await;
    }
}

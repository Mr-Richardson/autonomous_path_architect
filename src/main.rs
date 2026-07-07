#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logic;
mod rendering;
pub mod utils;

use macroquad::color::Color;
use macroquad::input::is_mouse_button_pressed;
use macroquad::prelude::screen_width;
use macroquad::window::{Conf, clear_background, next_frame};
use miniquad::MouseButton::Left;

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
    let mut rendering: rendering::Manager = rendering::Manager::new(vec!["straight".to_string()], Color::new(0.1, 0.1, 0.1, 1.0)); //, "arc".to_string(), "until black".to_string()], Color::new(0.1, 0.1, 0.1, 1.0));
    let mut logic: logic::Manager = logic::Manager:new(3045.0579);
    loop {
        clear_background(macroquad::color::BLACK);
        rendering.resize_check(4.0);
        if is_mouse_button_pressed(Left) {
            logic.points = rendering.middle.point_set_check(logic.points, rendering.left.width, screen_width() - rendering.right.width);
        }
        rendering.render(&logic.points);
        if rendering.is_copy_code() {
            logic.copy_code();
        }
        next_frame().await;
    }
}

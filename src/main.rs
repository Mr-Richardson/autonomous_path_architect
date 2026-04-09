mod ui_left;

use crate::ui_left::UiLeft;
use macroquad::color::Color;
use macroquad::window::next_frame;

#[macroquad::main("Path Planning Engine")]
async fn main() {
    let mut ui_left: UiLeft = UiLeft::new(
        vec![
            "straight".to_string(),
            "turn".to_string(),
            "arc".to_string(),
            "drive until reflectivity".to_string(),
        ],
        Color::new(0.2, 0.2, 0.2, 1.0),
    );
    loop {
        ui_left.render();
        next_frame().await;
    }
}

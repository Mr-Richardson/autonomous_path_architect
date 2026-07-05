use arboard::Clipboard;
use macroquad::math::Vec2;
use std::ops::Mul;

mod code_generator;

pub(crate) struct Manager {
    pub(crate) points: Vec<Vec2>,
    field_width: f32,
    clipboard: Clipboard,
}

impl Manager {
    pub(crate) fn new(field_width: f32) -> Self {
        Self {
            points: Vec::new(),
            field_width,
            clipboard: Clipboard::new().expect("Clipboard creation failed"),
        }
    } // TODO: error handling

    pub(crate) fn copy_code(&mut self) {
        let mut coordinates = Vec::with_capacity(self.points.len());
        for point in &self.points {
            coordinates.push(point.mul(self.field_width))
        }
        let code = code_generator::generate(&self.points).unwrap_or_else(|e| {
            // TODO: error handling
            eprintln!("Error generating code: {}", e);
            "Select more points".to_string()
        });
        self.clipboard.set_text(code).expect("TODO: panic message");
    }
}

use macroquad::math::Vec2;
use std::ops::Mul;

mod code_generator;

pub(crate) struct Manager {
    pub(crate) points: Vec<Vec2>,
    field_width: f32,
}

impl Manager {
    pub(crate) fn new(field_width: f32) -> Self {
        Self { points: Vec::new(), field_width }
    }

    pub(crate) fn copy_code(&self) {
        // TODO: finish
        let mut coordinates = Vec::with_capacity(self.points.len());
        for point in &self.points {
            coordinates.push(point.mul(self.field_width))
        }
        code_generator::generate(&*self.points).unwrap_or_else(|e| {
            // TODO: error handling
            eprintln!("Error generating code: {}", e);
            "Select more points".to_string()
        });
    }
}

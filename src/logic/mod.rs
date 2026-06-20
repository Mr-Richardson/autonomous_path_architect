use macroquad::math::Vec2;

mod code_generator;

pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn new() -> Self {
        // TODO
        Self {}
    }

    pub(crate) fn copy_code(points: &[Vec2], field_width: f32) {
        // TODO: finish
        let mut coordinates = Vec::with_capacity(points.len());
        for mut point in points {
            coordinates.push(*point * field_width)
        }
        code_generator::generate(points).unwrap_or_else(|e| {
            // TODO: error handling
            eprintln!("Error generating code: {}", e);
            "Select more points".to_string()
        });
    }
}

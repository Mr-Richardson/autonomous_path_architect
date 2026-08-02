use glam::Vec2;

pub fn generate(positions: [Vec2; 3], code: String) -> String {
    let distance = positions[0].distance(positions[1]) as i32;
    let angle = (positions[1] - positions[0]).angle_to(positions[2] - positions[1]).to_degrees();
    // TODO
    code
}

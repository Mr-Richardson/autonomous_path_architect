use macroquad::math::Vec2;
use std::fmt::Write;

/// Convert the coordinates of the points on the field in mm to Python code
pub fn generate(points: &[Vec2]) -> Result<String, String> {
    if points.len() < 2 {
        return Err("Not enough points to generate code (need at least 2).".to_string());
    }
    let error_message = "The code generation failed. `write!` panicked!!!";
    let turn = (points[1] - *points.first().unwrap()).to_angle().to_degrees();
    let mut code: String = format!("from library import *\n\nasync def main():\n    await drive.turn({:.2})\n", turn);
    for i in 0..points.len() - 2 {
        let distance = points[i].distance(points[i + 1]);
        let angle = (points[i + 1] - points[i]).angle_between(points[i + 2] - points[i + 1]).to_degrees();
        if angle.is_nan() {
            write!(code, "    await drive.straight({:.2})\n", distance).expect(error_message);
        } else {
            write!(code, "    await drive.straight_and_turn({:.2}, {:.2})\n", distance, angle).expect(error_message);
        }
    }
    let distance = points[points.len() - 2].distance(*points.last().unwrap());
    write!(code, "    await drive.straight({:.2})\n\nrun_task(template())\n", distance).expect(error_message);
    Ok(code)
}

// #[cfg(test)]
// mod tests { // TODO: better tests
//     use super::*;
//     use macroquad::prelude::vec2;
//     #[test]
//     fn test_code_generation() {
//         let points2: Vec<Vec2> = vec![vec2(10.0, 10.0), vec2(10.0, 50.5)];
//         let points3: Vec<Vec2> = vec![vec2(10.0, 10.0), vec2(10.0, 10.0), vec2(150.0, 150.0)];
//         let points5: Vec<Vec2> = vec![vec2(10.0, 10.0), vec2(100.0, 500.0), vec2(150.0, 150.0), vec2(250.0, 250.0), vec2(30.0, 30.0)];
//         let result2: String = "from library import *\n\nasync def main():\n    await drive.turn(90.00)\n    await drive.straight(40.50)\n\nrun_task(template())\n".to_string();
//         let result3: String = "from library import *\n\nasync def main():\n    await drive.turn(0.00)\n    await drive.straight_and_turn(0.00, -161.46)\n    await drive.straight(353.55)\n\nrun_task(template())\n".to_string();
//         let result5: String = "from library import *\n\nasync def main():\n    await drive.turn(79.59)\n    await drive.straight_and_turn(498.20, -161.46)\n    await drive.straight_and_turn(353.55, 126.87)\n    await drive.straight_and_turn(141.42, 180.00)\n    await drive.straight(311.13)\n\nrun_task(template())\n".to_string();
//         let code2 = generate(&points2).unwrap();
//         let code3 = generate(&points3).unwrap();
//         let code5 = generate(&points5).unwrap();
//         println!("{}", code2);
//         println!("{}", code3);
//         println!("{}", code5);
//         assert_eq!(code2, result2);
//         assert_eq!(code3, result3);
//         assert_eq!(code5, result5);
//     }
// }

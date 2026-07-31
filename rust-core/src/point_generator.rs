use glam::Vec2;

pub fn generate(code: &str) -> Vec<Vec2> {
    let lines = code.lines().skip_while(|line| !line.contains("async def main(")).skip(1).take_while(|line| line.contains("    ")).collect::<Vec<&str>>();
    let mut points: Vec<Vec2> = Vec::new();
    // TODO
    points
}

#[cfg(test)]
mod tests {
    // TODO: better tests
    use super::*;
    #[test]
    fn test_point_generation() {
        print!(
            "{:?}",
            generate(
                "from pybricks.hubs import PrimeHub
from core.drive import Drive
from core.gear import Gear
from pybricks.tools import wait


async def main(hub: PrimeHub, drive: Drive, gear: Gear):
    hub.imu.reset_heading(0)

    samples = 500
    for i in range(samples):
        # Read the angular velocity on the Z axis (yaw)
        # Use -Axis.Z or Axis.Z depending on how your hub is oriented
        await drive.straight(distance=100)
        await drive.turn(360, 115)
        await drive.drive_until_resistance(-10, resistance=100)

        angle = (hub.imu.heading() + 180) % 360
"
            )
        )
    }
}

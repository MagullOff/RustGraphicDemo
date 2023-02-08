use super::Light;
use crate::consts::*;
use crate::traits::movable::Movable;
use crate::utils::get_angle;
use crate::utils::types::Point3;
impl Movable for Light {
    fn update(&mut self, tick: f32, animation: bool) {
        match self.direction {
            Some(_) => {
                if animation {
                    let angle = get_angle(tick);
                    self.position[0] = LIGHT_ORBIT_RADIUS * angle.sin();
                    self.position[1] = 0.0;
                    self.position[2] = LIGHT_ORBIT_RADIUS * angle.cos();
                }
                let a = LIGHT_ORBIT_RADIUS * self.rotation_angle.tan();
                let angle =
                    (self.position[0] / LIGHT_ORBIT_RADIUS).asin() - std::f32::consts::PI * 0.5;
                self.set_target(Point3::new(angle.cos() * a, 0.0, angle.sin() * a));
            }
            None => {}
        }
    }
}

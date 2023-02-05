use super::Light;
use crate::consts::*;
use crate::traits::movable::Movable;
use crate::utils::types::Point3;
impl Movable for Light {
    fn update(&mut self, tick: f32) {
        match self.direction {
            Some(_) => {
                let angle = (tick * 2.5).rem_euclid(2.0 * std::f32::consts::PI) as f32;
                self.position[0] = LIGHT_ORBIT_RADIUS * angle.sin();
                self.position[1] = LIGHT_ORBIT_RADIUS * angle.cos();
                self.position[2] = 0.0;
                self.set_target(Point3::new(0.0, 0.0, 0.0));
            }
            None => {}
        }
    }
}

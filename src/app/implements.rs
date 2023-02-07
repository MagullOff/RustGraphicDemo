use super::*;
use crate::utils::types::{Point3, Vector3};
use crate::utils::vector::Vector;
impl GraphicDemo {
    pub fn get_view_vector(&self, position: &Point3) -> Vector {
        Vector::from(self.camera.position - position).norm()
    }

    pub fn get_light_vector(&self, position: &Point3) -> Vec<(Vector, Color32, Option<Vector3>)> {
        self.lights
            .iter()
            .map(|light| {
                (
                    Vector::new(
                        light.get_position().x - position.x,
                        light.get_position().y - position.y,
                        light.get_position().z - position.z,
                    )
                    .norm(),
                    light.get_color(),
                    light.get_direction(),
                )
            })
            .collect()
    }
}

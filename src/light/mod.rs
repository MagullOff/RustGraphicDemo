use crate::consts::SHAPE_ORBIT_RADIUS;
use egui::Color32;
use nalgebra::{Point3, Vector3};

use crate::traits::movable::Movable;
mod implements;
mod traits;

#[derive(Clone, Copy)]
pub struct Light {
    position: Point3<f32>,
    color: Color32,
    direction: Option<Vector3<f32>>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            color: Color32::WHITE,
            direction: None,
        }
    }
}

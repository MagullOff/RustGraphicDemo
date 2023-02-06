use crate::utils::types::*;
use egui::Color32;

mod implements;
mod traits;

#[derive(Clone, Copy)]
pub struct Light {
    pub position: Point3,
    color: Color32,
    direction: Option<Vector3>,
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

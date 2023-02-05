use egui::Color32;
use nalgebra::Point3;

pub struct PointLight {
    pub position: Point3<f32>,
    color: Color32,
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            position: Point3::new(0.0, 0.0, 0.0),
            color: Color32::WHITE,
        }
    }
}

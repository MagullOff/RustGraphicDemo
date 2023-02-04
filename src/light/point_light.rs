use egui::Color32;
use nalgebra::Point3;

pub struct PointLight {
    position: Point3<f32>,
    color: Color32,
}

use egui::Color32;
use nalgebra::Point3;

pub struct Reflector {
    position: Point3<f32>,
    target: Point3<f32>,
    color: Color32,
}

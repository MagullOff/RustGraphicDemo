use crate::camera::Camera;
use crate::consts::*;
use crate::light::Light;
use crate::shape::Shape;
use crate::utils::types::Point3;
use egui::Color32;

pub struct GraphicDemo {
    pub shading_type: ShadingType,
    pub fov: f32,
    pub light_parameters: LightParameters,
    pub lights: Vec<Light>,
    pub camera: Camera,
    shapes: Vec<Shape>,
    animation: bool,
    fog: bool,
    fog_color: [f32; 3],
    light_force: f32,
}

pub struct LightParameters {
    pub kd: f32,
    pub ks: f32,
    pub m: f32,
}

#[derive(PartialEq, Eq)]
pub enum ShadingType {
    Constant,
    Gouraud,
    Phong,
}

pub mod default;
pub mod fog;
pub mod implements;
pub mod painter;
pub mod ui;

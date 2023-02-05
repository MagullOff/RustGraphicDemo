use crate::camera::Camera;
use crate::consts::*;
use crate::light::point_light::PointLight;
use crate::shape::{Shape, ShapeMovementType};
use crate::utils::file_load::load_polygons;
use egui::Color32;
use nalgebra::Point3;
pub struct GraphicDemo {
    filling_type: FillingType,
    pub fov: f32,
    pub light_parameters: LightParameters,
    pub light: PointLight,
    shapes: Vec<Shape>,
    animation: bool,
    pub camera: Camera,
}

pub struct LightParameters {
    pub kd: f32,
    pub ks: f32,
    pub m: f32,
}

#[derive(PartialEq, Eq)]
pub enum FillingType {
    Constant,
    Gouraud,
    Phong,
}

pub mod painter;
pub mod ui;

impl Default for GraphicDemo {
    fn default() -> Self {
        GraphicDemo {
            filling_type: FillingType::Constant,
            fov: 90.0,
            light_parameters: LightParameters {
                kd: MAX_KD / 2.0,
                ks: MAX_KS / 2.0,
                m: 1.0,
            },
            animation: true,
            shapes: vec![
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, -150.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::GRAY,
                ),
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, 150.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::BLUE,
                ),
                Shape::new(
                    load_polygons("assets/sphere.obj"),
                    Point3::new(0.0, -150.0, 0.0),
                    ShapeMovementType::Orbital,
                    Color32::RED,
                ),
            ],
            camera: Camera::default(),
            light: PointLight::default(),
        }
    }
}

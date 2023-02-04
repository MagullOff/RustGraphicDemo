use crate::camera::Camera;
use crate::consts::*;
use crate::shape::{Shape, ShapeMovementType};
use crate::utils::file_load::load_polygons;
use nalgebra::Point3;
pub struct GraphicDemo {
    filling_type: FillingType,
    fov: f32,
    light_parameters: LightParameters,
    shapes: Vec<Shape>,
    animation: bool,
    camera: Camera,
}

pub struct LightParameters {
    kd: f32,
    ks: f32,
    m: f32,
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
                m: MAX_M / 2.0,
            },
            animation: true,
            shapes: vec![
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, -150.0, 0.0),
                    ShapeMovementType::Static,
                ),
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, 150.0, 0.0),
                    ShapeMovementType::Static,
                ),
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, -150.0, 0.0),
                    ShapeMovementType::Orbital,
                ),
            ],
            camera: Camera::default(),
        }
    }
}

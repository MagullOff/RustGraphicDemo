use crate::consts::*;
use crate::shape::{Shape, ShapeMovementType};
use crate::utils::file_load::load_polygons;
pub struct GraphicDemo {
    filling_type: FillingType,
    chosen_camera: ChosenCamera,
    light_rotation: f32,
    light_parameters: LightParameters,
    shapes: Vec<Shape>,
    animation: bool,
}

pub struct LightParameters {
    kd: f32,
    ks: f32,
    m: f32,
}

#[derive(PartialEq, Eq)]
pub enum ChosenCamera {
    Static,
    Following,
    Moving,
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
            chosen_camera: ChosenCamera::Static,
            filling_type: FillingType::Constant,
            light_rotation: 0.0,
            light_parameters: LightParameters {
                kd: MAX_KD / 2.0,
                ks: MAX_KS / 2.0,
                m: MAX_M / 2.0,
            },
            animation: true,
            shapes: vec![Shape {
                position: [0, 0, 0],
                polygons: load_polygons("assets/sphere.obj"),
                ..Shape::default()
            }],
        }
    }
}

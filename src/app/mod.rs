use crate::camera::Camera;
use crate::consts::*;
use crate::light::Light;
use crate::shape::{Shape, ShapeMovementType};
use crate::utils::file_load::load_polygons;
use crate::utils::types::{Point3, Vector3};
use crate::utils::vector::Vector;
use egui::Color32;

pub struct GraphicDemo {
    filling_type: FillingType,
    pub fov: f32,
    pub light_parameters: LightParameters,
    pub lights: Vec<Light>,
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
                    load_polygons("assets/sphere.obj"),
                    Point3::new(0.0, -150.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::WHITE,
                ),
                Shape::new(
                    load_polygons("assets/sphere.obj"),
                    Point3::new(0.0, 150.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::WHITE,
                ),
                Shape::new(
                    load_polygons("assets/cube.obj"),
                    Point3::new(0.0, 0.0, 0.0),
                    ShapeMovementType::Orbital,
                    Color32::WHITE,
                ),
            ],
            camera: Camera::default(),
            lights: vec![
                *Light::default()
                    .set_position(STATIC_LIGHT1_POSITION)
                    .set_color(Color32::WHITE),
                *Light::default()
                    .set_target(DYNAMIC_LIGHT_TARGET)
                    .set_color(Color32::RED),
            ],
        }
    }
}

impl GraphicDemo {
    pub fn get_view_vector(&self, position: &Point3) -> Vector {
        Vector::from(self.camera.position - position).norm()
    }

    pub fn get_light_vector(&self, position: &Point3) -> Vec<(Vector, Color32, Option<Vector3>)> {
        self.lights
            .iter()
            .map(|light| {
                (
                    Vector::from(light.get_position() - position).norm(),
                    light.get_color(),
                    light.get_direction(),
                )
            })
            .collect()
    }
}

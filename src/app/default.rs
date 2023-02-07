use super::*;
use crate::light::Light;
use crate::shape::{Shape, ShapeMovementType};
use crate::utils::file_load::load_polygons;
impl Default for GraphicDemo {
    fn default() -> Self {
        GraphicDemo {
            shading_type: ShadingType::Phong,
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
                    Point3::new(-200.0, 0.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::YELLOW,
                ),
                Shape::new(
                    load_polygons("assets/sphere.obj"),
                    Point3::new(0.0, 200.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::RED,
                ),
                Shape::new(
                    load_polygons("assets/sphere.obj"),
                    Point3::new(0.0, -200.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::GREEN,
                ),
                Shape::new(
                    load_polygons("assets/sphere.obj"),
                    Point3::new(200.0, 0.0, 0.0),
                    ShapeMovementType::Static,
                    Color32::BLUE,
                ),
                Shape::new(
                    load_polygons("assets/rat.obj"),
                    Point3::new(0.0, 0.0, 0.0),
                    ShapeMovementType::Orbital,
                    Color32::DARK_GREEN,
                ),
            ],
            camera: Camera::default(),
            lights: vec![
                *Light::default()
                    .set_position(STATIC_LIGHT2_POSITION)
                    .set_color(Color32::WHITE),
                *Light::default()
                    .set_position(STATIC_LIGHT1_POSITION)
                    .set_color(Color32::WHITE),
                *Light::default()
                    .set_target(DYNAMIC_LIGHT_TARGET)
                    .set_color(Color32::WHITE),
            ],
            fog: false,
            fog_color: [0.6, 0.6, 0.6],
            light_force: 1.0,
        }
    }
}

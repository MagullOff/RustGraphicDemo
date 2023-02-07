#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    use nalgebra::{Point3, Vector3};
    pub const IMAGE_SIZE: usize = 800;
    pub const SHAPE_SIZE: u32 = 200;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;

    pub const SHAPE_ORBIT_RADIUS: f32 = 800.0;
    pub const CAMERA_ORBIT_RADIUS: f32 = 400.0 + SHAPE_ORBIT_RADIUS;

    pub const FOLLOWING_CAMERA_POSITION: Point3<f32> = Point3::new(-900.0, -900.0, -1000.0);

    pub const MOVING_CAMERA_TARGET: Point3<f32> = Point3::new(0.0, 0.0, 0.0);
    pub const MOVING_CAMERA_Y: f32 = -500.0;

    pub const STATIC_CAMERA_POSITION: Point3<f32> = Point3::new(900.0, -500.0, 500.0);
    pub const STATIC_CAMERA_TARGET: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    pub const UP_VECTOR: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

    pub const CAMERA_NEAR: f32 = 1.0;
    pub const CAMERA_FAR: f32 = 4000.0;

    pub const STATIC_LIGHT1_POSITION: Point3<f32> = Point3::new(200.0, -500.0, 200.0);
    pub const STATIC_LIGHT2_POSITION: Point3<f32> = Point3::new(-200.0, 500.0, -200.0);
    pub const DYNAMIC_LIGHT_TARGET: Point3<f32> = Point3::new(0.0, 0.0, 0.0);

    pub const LIGHT_ORBIT_RADIUS: f32 = 650.0;

    pub const AMBIENT_KA: f32 = 0.17;
}

mod app;
mod camera;
mod filler;
mod light;
mod shape;
mod traits;
mod utils;
pub use app::GraphicDemo;
pub use shape::polygon;
pub use utils::file_load;

#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: usize = 900;
    pub const SHAPE_SIZE: u32 = 200;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;

    pub const SHAPE_ORBIT_RADIUS: f32 = 800.0;
    pub const CAMERA_ORBIT_RADIUS: f32 = 400.0 + SHAPE_ORBIT_RADIUS;

    pub const FOLLOWING_CAMERA_POSITION: [f32; 3] = [-900.0, 900.0, -1000.0];

    pub const MOVING_CAMERA_TARGET: [f32; 3] = [0.0, 0.0, 0.0];
    pub const MOVING_CAMERA_Z: f32 = -500.0;

    pub const STATIC_CAMERA_POSITION: [f32; 3] = [900.0, 900.0, -500.0];
    pub const STATIC_CAMERA_TARGET: [f32; 3] = [0.0, 0.0, 0.0];

    pub const UP_VECTOR: [i32; 3] = [0, 0, 1];

    pub const CAMERA_NEAR: f32 = 1.0;
    pub const CAMERA_FAR: f32 = 4000.0;
}

mod app;
mod camera;
mod light;
mod shape;
mod traits;
mod utils;
pub use app::GraphicDemo;
pub use shape::polygon;
pub use utils::{edge, file_load, vector};

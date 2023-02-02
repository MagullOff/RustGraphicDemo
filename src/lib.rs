#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: usize = 900;
    pub const SHAPE_SIZE: u32 = 200;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;

    pub const STATIC_CAMERA_POSITION: [i32; 3] = [0, 0, 0];
    pub const STATIC_CAMERA_TARGET: [i32; 3] = [0, 0, 0];
    pub const UP_VECTOR: [i32; 3] = [0, 0, 1];
}

mod app;
mod camera;
mod shape;
mod traits;
mod utils;
pub use app::GraphicDemo;
pub use shape::polygon;
pub use utils::{edge, file_load, vector};

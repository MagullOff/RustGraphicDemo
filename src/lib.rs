#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: usize = 700;
    pub const SHAPE_SIZE: u32 = 200;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;
}

mod app;
mod edge;
mod shape;
mod utils;
mod vector;
pub use app::GraphicDemo;
pub use shape::polygon;

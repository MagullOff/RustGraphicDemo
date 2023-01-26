#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: usize = 700;
    pub const MAX_KD: f32 = 1.0;
    pub const MAX_KS: f32 = 1.0;
    pub const MAX_M: f32 = 100.0;
}

mod app;

pub use app::GraphicDemo;

#![warn(clippy::all, rust_2018_idioms)]

pub mod consts {
    pub const IMAGE_SIZE: u32 = 700;
}

mod app;

pub use app::GraphicDemo;

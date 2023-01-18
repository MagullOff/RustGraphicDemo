pub struct GraphicDemo {
    filling_type: FillingType,
    chosen_camera: ChosenCamera,
    light_rotation: f32,
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
        }
    }
}

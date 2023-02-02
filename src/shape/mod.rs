pub mod polygon;
use polygon::Polygon;

use crate::traits::movable::Movable;
pub enum ShapeMovementType {
    Static,
    Orbital,
}

pub struct Shape {
    pub polygons: Vec<Polygon>,
    pub position: [i32; 3],
    pub movement_type: ShapeMovementType,
}

impl Movable for Shape {
    fn update(&mut self, tick: f32) {
        match self.movement_type {
            ShapeMovementType::Static => {}
            ShapeMovementType::Orbital => {
                todo!()
            }
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            polygons: vec![],
            position: [0, 0, 0],
            movement_type: ShapeMovementType::Static,
        }
    }
}

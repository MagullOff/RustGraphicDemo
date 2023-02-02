pub mod polygon;
use polygon::{Polygon, Vertex};

use crate::traits::movable::Movable;

#[derive(Debug)]
pub enum ShapeMovementType {
    Static,
    Orbital,
}

#[derive(Debug)]
pub struct Shape {
    pub transformed_polygons: Vec<Polygon>,
    polygons: Vec<Polygon>,
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

impl Shape {
    pub fn new(
        polygons: Vec<Polygon>,
        position: [i32; 3],
        movement_type: ShapeMovementType,
    ) -> Self {
        let mut new_polygons = polygons.clone();
        new_polygons
            .iter_mut()
            .for_each(|p| p.move_vertices(position));

        Shape {
            transformed_polygons: new_polygons,
            polygons,
            position,
            movement_type,
        }
    }
}

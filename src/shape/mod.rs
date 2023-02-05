pub mod polygon;
use crate::utils::types::*;
use egui::Color32;
use polygon::Polygon;

use crate::{consts::SHAPE_ORBIT_RADIUS, traits::movable::Movable};

#[derive(Debug)]
pub enum ShapeMovementType {
    Static,
    Orbital,
}

#[derive(Debug)]
pub struct Shape {
    pub transformed_polygons: Vec<Polygon>,
    polygons: Vec<Polygon>,
    pub position: Point3,
    pub movement_type: ShapeMovementType,
    pub matrix: Matrix4,
    pub color: Color32,
}

impl Movable for Shape {
    fn update(&mut self, tick: f32) {
        match self.movement_type {
            ShapeMovementType::Static => {}
            ShapeMovementType::Orbital => {
                let angle = (tick * 2.5).rem_euclid(2.0 * std::f32::consts::PI) as f32;
                self.position[0] = SHAPE_ORBIT_RADIUS * angle.sin();
                self.position[1] = SHAPE_ORBIT_RADIUS * angle.cos();
                let mut new_polygons = self.polygons.clone();
                new_polygons
                    .iter_mut()
                    .for_each(|p| p.move_vertices(self.position));
                self.transformed_polygons = new_polygons;
                self.matrix = Matrix4::new_rotation_wrt_point(
                    Vector3::from_vec(vec![0.0, 0.0, angle]),
                    Point3::new(
                        self.position[0] as f32,
                        self.position[1] as f32,
                        self.position[2] as f32,
                    ),
                )
            }
        }
    }
}

impl Shape {
    pub fn new(
        polygons: Vec<Polygon>,
        position: Point3,
        movement_type: ShapeMovementType,
        color: Color32,
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
            matrix: Matrix4::new_rotation(Vector3::zeros()),
            color,
        }
    }
}

use super::GraphicDemo;
use crate::consts::*;
use crate::polygon::{Polygon, Vertex};
use crate::utils::types::{Matrix4, Point3, Vector4};
use egui::*;
use itertools::Itertools;

pub fn is_in_range(p: Vector4) -> bool {
    (0..=2)
        .map(|i| p[i] > -p[3] && p[i] < p[3])
        .all(|cond| cond)
}

fn calculate_point_vector(position: Point3) -> Vector4 {
    Vector4::new(position.x, position.y, position.z, 1.0)
}

fn calculate_clip_cords(
    perspective_matrix: Matrix4,
    view_matrix: Matrix4,
    rotation_matrix: Matrix4,
    p: Vector4,
) -> (Vector4, Vector4) {
    let rotated_points = rotation_matrix * p;
    (
        perspective_matrix * (view_matrix * rotated_points),
        rotated_points,
    )
}

fn check_if_in_range(point1: Vector4, point2: Vector4) -> bool {
    is_in_range(point1) && is_in_range(point2)
}
fn calculate_normalized_cords(p: Vector4) -> Point3 {
    Point3::new(
        (p[0] / p[3] + 1.0) / 2.0 * IMAGE_SIZE as f32,
        (p[1] / p[3] + 1.0) / 2.0 * IMAGE_SIZE as f32,
        (p[2] / p[3] + 1.0) / 2.0 * IMAGE_SIZE as f32,
    )
}

impl GraphicDemo {
    fn draw_polygon(
        &self,
        view_matrix: Matrix4,
        rotation_matrix: Matrix4,
        polygon: &Polygon,
        map: &mut ColorImage,
        zbuffor: &mut Vec<Vec<f32>>,
        color: Color32,
    ) {
        let a = 1.0;
        let fov = (self.fov / 180.0) * std::f32::consts::PI;

        let perspective_matrix = Matrix4::new_perspective(a, fov, CAMERA_NEAR, CAMERA_FAR);
        let normalized_vertices = polygon
            .vertices
            .iter()
            .map(|v| calculate_point_vector(v.position))
            .map(|p| calculate_clip_cords(perspective_matrix, view_matrix, rotation_matrix, p))
            .map(|(clip_cords, rotated_vertices)| {
                (
                    calculate_normalized_cords(clip_cords),
                    clip_cords,
                    rotated_vertices,
                )
            });

        let viewport_vertices = normalized_vertices.clone().map(|(p, _, _)| p).collect_vec();
        let is_not_clipped = normalized_vertices
            .clone()
            .combinations(2)
            .map(|pair| check_if_in_range(pair[0].1, pair[1].1))
            .all(|cond| cond);
        if is_not_clipped {
            let normal_vertices = polygon.vertices.iter().map(|v| v.normal).collect_vec();
            let rotated_vertices = normalized_vertices
                .clone()
                .map(|(_, _, x)| Point3::new(x[0], x[1], x[2]))
                .collect_vec();
            self.fill_polygon(
                &viewport_vertices,
                &rotated_vertices,
                &normal_vertices,
                map,
                zbuffor,
                color,
            );
        }
    }

    pub fn paint(&mut self) -> egui::ColorImage {
        let mut map = ColorImage::new(
            [(IMAGE_SIZE + 1) as usize, (IMAGE_SIZE + 1) as usize],
            Color32::BLACK,
        );

        let mut zbuffor: Vec<Vec<f32>> =
            vec![vec![std::f32::MAX; IMAGE_SIZE as usize + 1]; IMAGE_SIZE as usize + 1];

        for shape in &self.shapes {
            for polygon in &shape.transformed_polygons {
                self.draw_polygon(
                    self.camera.matrix,
                    shape.matrix,
                    polygon,
                    &mut map,
                    &mut zbuffor,
                    shape.color,
                );
            }
        }

        if self.fog {
            self.apply_fog(zbuffor, &mut map);
        }

        for x in 0..IMAGE_SIZE {
            [(x, 0), (0, x), (IMAGE_SIZE, x), (x, IMAGE_SIZE)]
                .into_iter()
                .for_each(|index| map[index] = Color32::WHITE);
        }

        map
    }
}

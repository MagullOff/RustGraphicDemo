use super::GraphicDemo;
use crate::consts::*;
use crate::polygon::Polygon;
use crate::utils::bresenham::draw_bresenham_line;
use egui::*;
use itertools::Itertools;
use nalgebra::{Matrix4, Point3, Vector4};
type Cords = (f32, f32);

pub fn is_in_range(p: Vector4<f32>) -> bool {
    (0..=2)
        .map(|i| p[i] > -p[3] && p[i] < p[3])
        .all(|cond| cond)
}

fn calculate_point_vector(position: Point3<f32>) -> Vector4<f32> {
    Vector4::new(position.x, position.y, position.z, 1.0)
}

fn calculate_clip_cords(
    perspective_matrix: Matrix4<f32>,
    view_matrixrix: Matrix4<f32>,
    rotation_matrix: Matrix4<f32>,
    p: Vector4<f32>,
) -> Vector4<f32> {
    perspective_matrix * (view_matrixrix * (rotation_matrix * p))
}

fn draw_if_in_range(
    point1: (Cords, Vector4<f32>),
    point2: (Cords, Vector4<f32>),
    map: &mut ColorImage,
    color: Color32,
) {
    if is_in_range(point1.1) && is_in_range(point2.1) {
        draw_bresenham_line(map, point1.0, point2.0, color);
    }
}
fn calculate_normalized_cords(p: Vector4<f32>) -> (f32, f32) {
    (
        (p[0] / p[3] + 1.0) / 2.0 * IMAGE_SIZE as f32,
        (p[1] / p[3] + 1.0) / 2.0 * IMAGE_SIZE as f32,
    )
}

impl GraphicDemo {
    fn draw_lines(
        &self,
        view_matrix: Matrix4<f32>,
        rotation_matrix: Matrix4<f32>,
        polygon: &Polygon,
        map: &mut ColorImage,
        color: Color32,
    ) {
        let a = 1.0;
        let fov_deg = self.fov;
        let fov = (fov_deg / 180.0) * std::f32::consts::PI;

        let perspective_matrix = Matrix4::new_perspective(a, fov, CAMERA_NEAR, CAMERA_FAR);
        polygon
            .vertices
            .iter()
            .map(|v| calculate_point_vector(v.position))
            .map(|p| calculate_clip_cords(perspective_matrix, view_matrix, rotation_matrix, p))
            .map(|clip_cords| (calculate_normalized_cords(clip_cords), clip_cords))
            .combinations(2)
            .for_each(|pair| draw_if_in_range(pair[0], pair[1], map, color));
    }

    pub fn paint(&mut self) -> egui::ColorImage {
        let mut map = ColorImage::new(
            [(IMAGE_SIZE + 1) as usize, (IMAGE_SIZE + 1) as usize],
            Color32::TRANSPARENT,
        );

        // let mut zbuffor: Vec<Vec<f32>> =
        //     vec![vec![std::f32::MAX; IMAGE_SIZE as usize + 1]; IMAGE_SIZE as usize + 1];

        for shape in &self.shapes {
            for polygon in &shape.transformed_polygons {
                self.draw_lines(
                    self.camera.matrix,
                    shape.matrix,
                    polygon,
                    &mut map,
                    Color32::LIGHT_GRAY,
                );
            }
        }

        for x in 0..IMAGE_SIZE {
            [(x, 0), (0, x), (IMAGE_SIZE, x), (x, IMAGE_SIZE)]
                .into_iter()
                .for_each(|index| map[index] = Color32::WHITE);
        }
        map
    }
}

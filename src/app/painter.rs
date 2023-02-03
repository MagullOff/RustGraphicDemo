use super::GraphicDemo;
use crate::consts::*;
use crate::polygon::Polygon;
use crate::utils::bresenham::draw_bresenham_line;
use nalgebra::{Matrix4, Vector4};
// use crate::utils::bresenham::draw_bresenham_line;
use egui::*;
pub fn is_in_range(p: (usize, usize)) -> bool {
    p.0 > 0 && p.1 > 0 && p.0 < IMAGE_SIZE as usize && p.1 < IMAGE_SIZE as usize
}

impl GraphicDemo {
    fn draw_lines(
        &self,
        view_mat: Matrix4<f32>,
        model_mat2: Matrix4<f32>,
        polygon: &Polygon,
        map: &mut ColorImage,
        color: Color32,
    ) {
        let p1: Vector4<f32> = Vector4::new(
            (polygon.vertices[0].position[0] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[0].position[1] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[0].position[2] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            1.0,
        );
        let p2: Vector4<f32> = Vector4::new(
            (polygon.vertices[1].position[0] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[1].position[1] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[1].position[2] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            1.0,
        );
        let p3: Vector4<f32> = Vector4::new(
            (polygon.vertices[2].position[0] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[2].position[1] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            (polygon.vertices[2].position[2] - (IMAGE_SIZE as i32 / 2)) as f32
                / (IMAGE_SIZE / 2) as f32,
            1.0,
        );
        let a = 1.0;
        let fov_deg = self.light_rotation;
        let n = 0.5;
        let f = 100.0;
        let fov = (fov_deg / 180.0) * std::f32::consts::PI;

        let m = Matrix4::new_perspective(a, fov, n, f);

        let xy = m * (view_mat * (model_mat2 * p1));
        let x1 = (((xy[0] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let y1 = (((xy[1] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let z1 = (((xy[2] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let xy = m * (view_mat * (model_mat2 * p2));
        let x2 = (((xy[0] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let y2 = (((xy[1] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let z2 = (((xy[2] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let xy = m * (view_mat * (model_mat2 * p3));
        let x3 = (((xy[0] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let y3 = (((xy[1] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        let z3 = (((xy[2] + 1.0) / 2.0) * IMAGE_SIZE as f32) as usize;
        if is_in_range((x1, y1)) && is_in_range((x2, y2)) {
            draw_bresenham_line(map, (x1 as f32, y1 as f32), (x2 as f32, y2 as f32), color);
        }
        if is_in_range((x3, y3)) && is_in_range((x2, y2)) {
            draw_bresenham_line(map, (x2 as f32, y2 as f32), (x3 as f32, y3 as f32), color);
        }
        if is_in_range((x1, y1)) && is_in_range((x2, y2)) {
            draw_bresenham_line(map, (x1 as f32, y1 as f32), (x2 as f32, y2 as f32), color);
        }
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

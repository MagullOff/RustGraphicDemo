use super::edge::Edge;
use crate::app::ShadingType;
use crate::consts::AMBIENT_KA;
use crate::polygon::Polygon;
use crate::utils::types::{Point2, Point3, Vector3};
use crate::utils::vector::Vector;
use crate::GraphicDemo;
use egui::{Color32, ColorImage};
use itertools::Itertools;

fn get_barocenttric_coordinates(vertices: &[Point3], point: Point2) -> (f32, f32, f32) {
    let (x1, y1) = (vertices[0][0] as i32, vertices[0][1] as i32);
    let (x2, y2) = (vertices[1][0] as i32, vertices[1][1] as i32);
    let (x3, y3) = (vertices[2][0] as i32, vertices[2][1] as i32);
    let w1 = ((y2 - y3) * (point.x - x3) + (x3 - x2) * (point.y - y3)) as f32
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
    let w2 = ((y3 - y1) * (point.x - x3) + (x1 - x3) * (point.y - y3)) as f32
        / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
    let w3 = 1.0 - w1 - w2;
    (w1, w2, w3)
}

fn to_color(vector: Vector) -> Color32 {
    Color32::from_rgb(
        (vector.x * 255.0) as u8,
        (vector.y * 255.0) as u8,
        (vector.z * 255.0) as u8,
    )
}

enum ColorInterpolation {
    Phong,
    Gouraud([Vector; 3]),
    Constant(Color32),
}

impl GraphicDemo {
    fn get_color_at(&self, polygon: &Polygon, point: Point3, color: Color32) -> Vector {
        let bacrocentric_coordinates = get_barocenttric_coordinates(
            &polygon.vertices.iter().map(|v| v.position).collect_vec(),
            Point2::new(point.x as i32, point.y as i32),
        );

        let (w1, w2, w3) = bacrocentric_coordinates;
        let normals = polygon
            .vertices
            .iter()
            .map(|v| Vector::from(v.normal).norm())
            .collect::<Vec<Vector>>();
        let true_normal = Vector::new(
            normals[0].x * w1 + normals[1].x * w2 + normals[2].x * w3,
            normals[0].y * w1 + normals[1].y * w2 + normals[2].y * w3,
            normals[0].z * w1 + normals[1].z * w2 + normals[2].z * w3,
        );
        let n_vec = true_normal.norm();
        let v_vec = self.get_view_vector(&polygon.center);
        self.get_light_vector(&polygon.center)
            .into_iter()
            .map(|(vec, col, direction)| {
                let vec1 = vec;
                let r_vec = n_vec.multiply(n_vec * vec1 * 2.0) - vec1;
                self.get_color(
                    n_vec,
                    vec1,
                    v_vec,
                    r_vec,
                    (0 as u32, 0 as u32),
                    color,
                    col,
                    direction,
                )
            })
            .fold(
                Vector::new(AMBIENT_KA, AMBIENT_KA, AMBIENT_KA),
                |acc, val| acc + val,
            )
    }

    fn get_color(
        &self,
        n_vec: Vector,
        l_vec: Vector,
        v_vec: Vector,
        r_vec: Vector,
        _cords: (u32, u32),
        color1: Color32,
        light_color: Color32,
        direction: Option<Vector3>,
    ) -> Vector {
        let light_rgb = [
            light_color.r() as f32 / 255.0,
            light_color.g() as f32 / 255.0,
            light_color.b() as f32 / 255.0,
        ];
        let color = [
            color1.r() as f32 / 255.0,
            color1.g() as f32 / 255.0,
            color1.b() as f32 / 255.0,
        ];
        let view_angle_multiplyer = Vector::cos(v_vec, r_vec)
            .max(0.0)
            .powf(self.light_parameters.m);

        let target_multiplier = match direction {
            Some(vec) => Vector::cos(Vector::from(vec).multiply(-1.0), r_vec).max(0.0),
            None => 1.0,
        };

        let light_angle_multiplyer = Vector::cos(n_vec, l_vec.norm()).max(0.0);
        let rgb = (0..=2)
            .map(|i| {
                self.get_color_component(
                    light_rgb[i],
                    color[i],
                    light_angle_multiplyer,
                    view_angle_multiplyer,
                    target_multiplier,
                )
            })
            .collect::<Vec<f32>>();
        Vector::new(rgb[0], rgb[1], rgb[2])
    }

    fn get_color_component(
        &self,
        light_component: f32,
        color_component: f32,
        light_angle_multiplyer: f32,
        view_angle_multiplyer: f32,
        target_multiplier: f32,
    ) -> f32 {
        target_multiplier
            * (self.light_parameters.kd
                * light_component
                * color_component
                * light_angle_multiplyer
                + self.light_parameters.ks
                    * light_component
                    * color_component
                    * view_angle_multiplyer)
    }

    pub fn paint_line(
        &self,
        aet: &Vec<Edge>,
        polygon: &Polygon,
        viewport_vertices: &[Point3],
        y: i32,
        map: &mut ColorImage,
        zbuffor: &mut Vec<Vec<f32>>,
        color: Color32,
    ) {
        let polygon_color = match self.shading_type {
            ShadingType::Constant => ColorInterpolation::Constant(to_color(self.get_color_at(
                polygon,
                polygon.center,
                color,
            ))),
            ShadingType::Phong => ColorInterpolation::Phong,
            ShadingType::Gouraud => ColorInterpolation::Gouraud([
                self.get_color_at(polygon, polygon.vertices[0].position, color),
                self.get_color_at(polygon, polygon.vertices[1].position, color),
                self.get_color_at(polygon, polygon.vertices[2].position, color),
            ]),
        };
        for i in (0..=((aet.len() as i8) - 2)).step_by(2) {
            for x in (aet[i as usize].min as i32)..(aet[(i + 1) as usize].min as i32) {
                let bacrocentric_coordinates =
                    get_barocenttric_coordinates(viewport_vertices, Point2::new(x, y));

                let z = get_interpolated_z(viewport_vertices, bacrocentric_coordinates);
                if z > zbuffor[x as usize][y as usize] {
                    return;
                }
                let rgb_res = match polygon_color {
                    ColorInterpolation::Constant(polygon_color) => polygon_color,
                    ColorInterpolation::Phong => {
                        to_color(self.get_color_at(polygon, polygon.vertices[0].position, color))
                    }
                    ColorInterpolation::Gouraud(color_vec) => {
                        let (w1, w2, w3) = get_barocenttric_coordinates(
                            &polygon.vertices.iter().map(|v| v.position).collect_vec(),
                            Point2::new(x as i32, y as i32),
                        );
                        to_color(Vector::new(
                            color_vec[0].x * w1 + color_vec[1].x * w2 + color_vec[2].x * w3,
                            color_vec[0].y * w1 + color_vec[1].y * w2 + color_vec[2].y * w3,
                            color_vec[0].z * w1 + color_vec[1].z * w2 + color_vec[2].z * w3,
                        ))
                    }
                };

                map[(x as usize, y as usize)] = rgb_res;
                zbuffor[x as usize][y as usize] = z;
            }
        }
    }
}

fn get_interpolated_z(vertices: &[Point3], coordinates: (f32, f32, f32)) -> f32 {
    vertices[0].z * coordinates.0 + vertices[1].z * coordinates.1 + vertices[2].z * coordinates.2
}

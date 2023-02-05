use self::edge::Edge;
use crate::polygon::*;
use crate::utils::vector::*;
use crate::GraphicDemo;
use egui::{Color32, ColorImage};
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;
pub mod edge;

pub fn normalize(v: &Vector3<f32>) -> Vector3<f32> {
    let len = len(v);
    Vector3::new(v.x / len, v.y / len, v.z / len)
}

pub fn to_vec(v: Vector3<f32>) -> Vector {
    Vector::new(v[0], v[1], v[2])
}

pub fn len(v: &Vector3<f32>) -> f32 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

pub fn cos(a: Vector3<f32>, b: Vector3<f32>) -> f32 {
    (a.dot(&b)) / (a.len() * b.len()) as f32
}
pub fn multiply(v: &Vector3<f32>, a: f32) -> Vector3<f32> {
    Vector3::new(v.x * a, v.y * a, v.z * a)
}

pub fn get_prev(i: usize, size: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        size - 1
    }
}

pub fn get_next(i: usize, size: usize) -> usize {
    (i + 1) % size
}

impl GraphicDemo {
    fn get_color(
        &self,
        n_vec: Vector,
        l_vec: Vector,
        v_vec: Vector,
        r_vec: Vector,
        cords: (u32, u32),
        color1: Color32,
    ) -> Vector {
        let light_rgb = [1.0, 1.0, 1.0];
        let color = [
            color1.r() as f32 / 255.0,
            color1.g() as f32 / 255.0,
            color1.b() as f32 / 255.0,
        ];
        let r =
            self.light_parameters.kd * light_rgb[0] * color[0] * Vector::cos(n_vec, l_vec).max(0.0)
                + self.light_parameters.ks
                    * light_rgb[0]
                    * color[0]
                    * Vector::cos(v_vec, r_vec)
                        .max(0.0)
                        .powf(self.light_parameters.m);
        let g =
            self.light_parameters.kd * light_rgb[1] * color[1] * Vector::cos(n_vec, l_vec).max(0.0)
                + self.light_parameters.ks
                    * light_rgb[1]
                    * color[1]
                    * Vector::cos(v_vec, r_vec)
                        .max(0.0)
                        .powf(self.light_parameters.m);
        let b =
            self.light_parameters.kd * light_rgb[2] * color[2] * Vector::cos(n_vec, l_vec).max(0.0)
                + self.light_parameters.ks
                    * light_rgb[2]
                    * color[2]
                    * Vector::cos(v_vec, r_vec)
                        .max(0.0)
                        .powf(self.light_parameters.m);
        Vector::new(r, g, b)
    }
    pub fn get_view_vector(&self, position: &Point3<f32>) -> Vector {
        Vector::new(
            self.camera.position[0] - position[0],
            self.camera.position[1] - position[1],
            self.camera.position[2] - position[2],
        )
    }

    pub fn get_light_vector(&self, position: &Point3<f32>) -> Vector {
        Vector::new(
            self.light.position[0] - position[0],
            self.light.position[1] - position[1],
            self.light.position[2] - position[2],
        )
    }

    pub fn fill_polygon(
        &self,
        polygon: &Polygon,
        map: &mut ColorImage,
        zbuffor: &mut Vec<Vec<f32>>,
        color: Color32,
    ) {
        let mut aet: Vec<Edge> = vec![];
        let mut edge_collection: HashMap<(usize, usize), i32> = HashMap::new();

        let ind = polygon.get_sorted_indeces();
        let positions = polygon
            .vertices
            .iter()
            .map(|v| {
                [
                    v.position.x as i32,
                    v.position.y as i32,
                    v.position.z as i32,
                ]
            })
            .collect::<Vec<[i32; 3]>>();
        let ymin = positions[*ind.first().unwrap()][1];
        let ymax = positions[*ind.last().unwrap()][1];
        let mut k = 0;
        for y in ymin..=ymax {
            let mut points_prev_scanline: Vec<usize> = vec![];
            while positions[ind[k]][1] == y - 1 {
                points_prev_scanline.push(ind[k]);
                k += 1;
            }
            for v in points_prev_scanline {
                let prev = get_prev(v, ind.len());
                if positions[prev][1] > positions[v][1] {
                    let new_edge = Edge::new(prev, v, &positions);
                    edge_collection.insert((prev, v), new_edge.id);
                    aet.push(new_edge);
                }

                if positions[prev][1] < positions[v][1] {
                    let remove_index = if edge_collection.contains_key(&(prev, v)) {
                        edge_collection.get(&(prev, v)).unwrap()
                    } else {
                        edge_collection.get(&(v, prev)).unwrap()
                    };

                    aet.retain(|e| e.id != *remove_index);
                }
                let next = get_next(v, ind.len());

                if positions[next][1] > positions[v][1] {
                    let new_edge = Edge::new(next, v, &positions);
                    edge_collection.insert((next, v), new_edge.id);
                    aet.push(new_edge);
                }

                if positions[next][1] < positions[v][1] {
                    let remove_index = if edge_collection.contains_key(&(next, v)) {
                        edge_collection.get(&(next, v)).unwrap()
                    } else {
                        edge_collection.get(&(v, next)).unwrap()
                    };

                    aet.retain(|e| e.id != *remove_index);
                }
            }
            aet.sort_by(|a, b| a.min.partial_cmp(&b.min).unwrap());
            self.paint_line(&aet, polygon, y, map, zbuffor, color);
            for i in 0..aet.len() {
                aet[i].min += aet[i].inv;
            }
        }
    }

    fn paint_line(
        &self,
        aet: &Vec<Edge>,
        polygon: &Polygon,
        y: i32,
        map: &mut ColorImage,
        zbuffor: &mut Vec<Vec<f32>>,
        color: Color32,
    ) {
        let mut i = 0;
        let (x1, y1) = (
            polygon.vertices[0].position[0] as i32,
            polygon.vertices[0].position[1] as i32,
        );
        let (x2, y2) = (
            polygon.vertices[1].position[0] as i32,
            polygon.vertices[1].position[1] as i32,
        );
        let (x3, y3) = (
            polygon.vertices[2].position[0] as i32,
            polygon.vertices[2].position[1] as i32,
        );
        while i <= (aet.len() as i8) - 2 {
            for x in (aet[i as usize].min as i32)..(aet[(i + 1) as usize].min as i32) {
                //interpolation
                let w1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) as f32
                    / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
                let w2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) as f32
                    / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3)) as f32;
                let w3 = 1.0 - w1 - w2;

                let z = polygon.vertices[0].position[2] as f32 * w1
                    + polygon.vertices[1].position[2] as f32 * w2
                    + polygon.vertices[2].position[2] as f32 * w3;

                if z > zbuffor[x as usize][y as usize] {
                    return;
                }

                let normals = (
                    to_vec(polygon.vertices[0].normal).norm(),
                    to_vec(polygon.vertices[1].normal).norm(),
                    to_vec(polygon.vertices[2].normal).norm(),
                );
                let true_normal = Vector::new(
                    normals.0.x * w1 + normals.1.x * w2 + normals.2.x * w3,
                    normals.0.y * w1 + normals.1.y * w2 + normals.2.y * w3,
                    normals.0.z * w1 + normals.1.z * w2 + normals.2.z * w3,
                );
                let z = polygon.vertices[0].position[2] as f32 * w1
                    + polygon.vertices[1].position[2] as f32 * w2
                    + polygon.vertices[2].position[2] as f32 * w3;

                let n_vec = true_normal.norm();

                let l_vec = self
                    .get_light_vector(&Point3::new(x as f32, y as f32, z))
                    .norm();
                let v_vec = self
                    .get_view_vector(&Point3::new(x as f32, y as f32, z))
                    .norm();
                let r_vec = n_vec.multiply(n_vec * l_vec * 2.0) - l_vec;
                let rgb = self.get_color(n_vec, l_vec, v_vec, r_vec, (x as u32, y as u32), color);
                let rgb_res = Color32::from_rgb(
                    (rgb.x * 255.0) as u8,
                    (rgb.y * 255.0) as u8,
                    (rgb.z * 255.0) as u8,
                );
                map[(x as usize, y as usize)] = rgb_res;
                zbuffor[x as usize][y as usize] = z;
            }
            i += 2;
        }
    }
}

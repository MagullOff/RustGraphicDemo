use self::edge::Edge;
use crate::app::ShadingType;
use crate::utils::types::*;
use crate::GraphicDemo;
use egui::{Color32, ColorImage};
use paint_line::*;
use std::collections::HashMap;
mod edge;
mod paint_line;

fn get_sorted_indeces(vertices: &Vec<Point3>) -> Vec<usize> {
    let mut sort_vec = vertices
        .iter()
        .map(|v| *v)
        .enumerate()
        .collect::<Vec<(usize, Point3)>>();
    sort_vec.sort_by(|(_, p1), (_, p2)| {
        if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sort_vec.iter().map(|elem| elem.0).collect()
}

impl GraphicDemo {
    pub fn fill_polygon(
        &self,
        viewport_vertices: &Vec<Point3>,
        rotated_vertices: &Vec<Point3>,
        normal_vectors: &Vec<Vector3>,
        map: &mut ColorImage,
        zbuffor: &mut Vec<Vec<f32>>,
        color: Color32,
    ) {
        let polygon_color = match self.shading_type {
            ShadingType::Constant => ColorInterpolation::Constant(to_color(self.get_color_at(
                rotated_vertices,
                get_center(rotated_vertices),
                normal_vectors,
                color,
            ))),
            ShadingType::Phong => ColorInterpolation::Phong,
            ShadingType::Gouraud => ColorInterpolation::Gouraud([
                self.get_color_at(rotated_vertices, rotated_vertices[0], normal_vectors, color),
                self.get_color_at(rotated_vertices, rotated_vertices[1], normal_vectors, color),
                self.get_color_at(rotated_vertices, rotated_vertices[2], normal_vectors, color),
            ]),
        };
        let mut aet: Vec<Edge> = vec![];
        let mut edge_collection: HashMap<(usize, usize), i32> = HashMap::new();

        let ind = get_sorted_indeces(viewport_vertices);
        let positions = viewport_vertices
            .iter()
            .map(|p| [p.x as i32, p.y as i32, p.z as i32])
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
            self.paint_line(
                &aet,
                viewport_vertices,
                rotated_vertices,
                normal_vectors,
                polygon_color,
                y,
                map,
                zbuffor,
                color,
            );
            for i in 0..aet.len() {
                aet[i].min += aet[i].inv;
            }
        }
    }
}
fn get_prev(i: usize, size: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        size - 1
    }
}

fn get_next(i: usize, size: usize) -> usize {
    (i + 1) % size
}

use nalgebra::{Point3, Vector3};

#[derive(Clone, Debug)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn get_sorted_indeces(&self) -> Vec<usize> {
        let mut sort_vec = self
            .vertices
            .iter()
            .map(|v| v.position)
            .enumerate()
            .collect::<Vec<(usize, Point3<f32>)>>();
        sort_vec.sort_by(|(_, p1), (_, p2)| {
            if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sort_vec.iter().map(|elem| elem.0).collect()
    }

    pub fn move_vertices(&mut self, vector: Point3<f32>) {
        self.vertices
            .iter_mut()
            .for_each(|v| v.move_by_vector(vector));
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    fn move_by_vector(&mut self, vector: Point3<f32>) {
        (0..=2).for_each(|i| self.position[i] += vector[i]);
    }
}

use crate::utils::types::*;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
    pub center: Point3,
}

impl Polygon {
    pub fn move_vertices(&mut self, vector: Point3) {
        self.vertices
            .iter_mut()
            .for_each(|v| v.move_by_vector(vector));
        self.center = Point3::new(
            vector.x + self.center.x,
            vector.y + self.center.y,
            vector.z + self.center.z,
        );
    }

    pub fn new(vertices: Vec<Vertex>) -> Self {
        let length = vertices.len() as f32;
        let (x, y, z) = &vertices
            .iter()
            .map(|v| (v.position.x, v.position.y, v.position.z))
            .fold((0f32, 0f32, 0f32), |acc, val| {
                (acc.0 + val.0, acc.1 + val.1, acc.2 + val.2)
            });
        Polygon {
            vertices,
            center: Point3::new(x / length, y / length, z / length),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Point3,
    pub normal: Vector3,
}

impl Vertex {
    fn move_by_vector(&mut self, vector: Point3) {
        (0..=2).for_each(|i| self.position[i] += vector[i]);
    }
}

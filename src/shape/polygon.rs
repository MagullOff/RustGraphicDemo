use crate::vector::Vector3;

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
            .collect::<Vec<(usize, [i32; 3])>>();
        sort_vec.sort_by(|(_, [x1, y1, _]), (_, [x2, y2, _])| {
            if y1 < y2 || (y1 == y2 && x1 < x2) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sort_vec.iter().map(|elem| elem.0).collect()
    }

    pub fn move_vertices(&mut self, vector: [i32; 3]) {
        self.vertices
            .iter_mut()
            .for_each(|v| v.move_by_vector(vector));
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: [i32; 3],
    pub normal: Vector3,
    pub color: Vector3,
}

impl Vertex {
    fn move_by_vector(&mut self, vector: [i32; 3]) {
        (0..=2).for_each(|i| self.position[i] += vector[i]);
    }
}

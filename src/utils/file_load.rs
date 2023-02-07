use crate::polygon::*;
use nalgebra::{Point3, Vector3};

pub fn load_polygons(_file_path: &str) -> Vec<Polygon> {
    vec![
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
            ],
            center: Point3::new(33.333332, -33.333332, -100.0),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 0.0, -1.0),
                },
            ],
            center: Point3::new(-33.333332, 33.333332, -100.0),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, 100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, -100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
            ],
            center: Point3::new(-100.0, 33.333332, -33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, -100.0, 100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, 100.0),
                    normal: Vector3::new(-1.0, 0.0, 0.0),
                },
            ],
            center: Point3::new(-100.0, -33.333332, 33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
            ],
            center: Point3::new(33.333332, 100.0, -33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, 100.0, -100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 1.0, 0.0),
                },
            ],
            center: Point3::new(-33.333332, 100.0, 33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(100.0, -100.0, -100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, -100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
            ],
            center: Point3::new(100.0, 33.333332, -33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(100.0, -100.0, -100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, 100.0),
                    normal: Vector3::new(1.0, 0.0, 0.0),
                },
            ],
            center: Point3::new(100.0, -33.333332, 33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
            ],
            center: Point3::new(33.333332, -100.0, -33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, -100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
                Vertex {
                    position: Point3::new(-100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                },
            ],
            center: Point3::new(-33.333332, -100.0, 33.333332),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: Point3::new(100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
            ],
            center: Point3::new(33.333332, -33.333332, 100.0),
        },
        Polygon {
            vertices: vec![
                Vertex {
                    position: Point3::new(-100.0, -100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: Point3::new(100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
                Vertex {
                    position: Point3::new(-100.0, 100.0, 100.0),
                    normal: Vector3::new(0.0, 0.0, 1.0),
                },
            ],
            center: Point3::new(-33.333332, 33.333332, 100.0),
        },
    ]
}

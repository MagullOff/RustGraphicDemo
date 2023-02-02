use crate::consts::*;
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(PartialEq, Eq)]
pub enum CameraKind {
    Static,
    Following,
    Moving,
}

struct Camera {
    position: [i32; 3],
    target: [i32; 3],
    matrix: Matrix4<f32>,
    kind: CameraKind,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: STATIC_CAMERA_POSITION,
            target: STATIC_CAMERA_POSITION,
            matrix: Matrix4::look_at_lh(
                &Point3::new(
                    STATIC_CAMERA_POSITION[0] as f32,
                    STATIC_CAMERA_POSITION[1] as f32,
                    STATIC_CAMERA_POSITION[2] as f32,
                ),
                &Point3::new(
                    STATIC_CAMERA_TARGET[0] as f32,
                    STATIC_CAMERA_TARGET[1] as f32,
                    STATIC_CAMERA_TARGET[2] as f32,
                ),
                &Vector3::from_vec(UP_VECTOR.map(|a| a as f32).to_vec()),
            ),
            kind: CameraKind::Static,
        }
    }
}


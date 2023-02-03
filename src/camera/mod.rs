use crate::consts::*;
use crate::traits::movable::Movable;
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(PartialEq, Eq)]
pub enum CameraKind {
    Static,
    Following,
    Moving,
}

pub struct Camera {
    pub position: [i32; 3],
    pub target: [i32; 3],
    pub matrix: Matrix4<f32>,
    pub kind: CameraKind,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: STATIC_CAMERA_POSITION,
            target: STATIC_CAMERA_POSITION,
            kind: CameraKind::Static,
            matrix: Matrix4::face_towards(
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
        }
    }
}

impl Movable for Camera {
    fn update(&mut self, tick: f32) {
        match self.kind {
            CameraKind::Following => todo!(),
            CameraKind::Moving => todo!(),
            CameraKind::Static => {}
        }
    }
}

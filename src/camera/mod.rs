use crate::consts::*;
use crate::traits::movable::Movable;
use nalgebra::{Matrix4, Point3, Vector3};

#[derive(PartialEq, Eq, Debug)]
pub enum CameraKind {
    Static,
    Following,
    Moving,
}

#[derive(Debug)]
pub struct Camera {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub matrix: Matrix4<f32>,
    pub kind: CameraKind,
}

impl Camera {
    fn calculate_matrix(&mut self) {
        self.matrix = Matrix4::look_at_rh(
            &Point3::new(
                self.position[0] as f32,
                self.position[1] as f32,
                self.position[2] as f32,
            ),
            &Point3::new(
                self.target[0] as f32,
                self.target[1] as f32,
                self.target[2] as f32,
            ),
            &Vector3::from_vec(UP_VECTOR.map(|a| a as f32).to_vec()),
        );
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: STATIC_CAMERA_POSITION,
            target: STATIC_CAMERA_TARGET,
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
        let angle = (tick * 2.5).rem_euclid(2.0 * std::f32::consts::PI) as f32;
        match self.kind {
            CameraKind::Following => {
                self.target[0] = SHAPE_ORBIT_RADIUS * angle.sin();
                self.target[1] = SHAPE_ORBIT_RADIUS * angle.cos();
                self.position = FOLLOWING_CAMERA_POSITION;
                self.calculate_matrix();
            }
            CameraKind::Moving => {
                self.target = MOVING_CAMERA_TARGET;
                self.position[0] = CAMERA_ORBIT_RADIUS * angle.sin();
                self.position[1] = CAMERA_ORBIT_RADIUS * angle.cos();
                self.position[2] = MOVING_CAMERA_Z;
                self.calculate_matrix();
            }
            CameraKind::Static => {
                self.position = STATIC_CAMERA_POSITION;
                self.target = STATIC_CAMERA_TARGET;
                self.calculate_matrix();
            }
        }
    }
}

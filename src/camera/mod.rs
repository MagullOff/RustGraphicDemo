use crate::consts::*;
use crate::traits::movable::Movable;
use crate::utils::get_angle;
use crate::utils::types::{Matrix4, Point3};

#[derive(PartialEq, Eq, Debug)]
pub enum CameraKind {
    Static,
    Following,
    Moving,
}

#[derive(Debug)]
pub struct Camera {
    pub position: Point3,
    pub target: Point3,
    pub matrix: Matrix4,
    pub kind: CameraKind,
}

impl Camera {
    fn calculate_matrix(&mut self) {
        self.matrix = Matrix4::look_at_rh(&self.position, &self.target, &UP_VECTOR);
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
                &UP_VECTOR,
            ),
        }
    }
}

impl Movable for Camera {
    fn update(&mut self, tick: f32) {
        let angle = get_angle(tick);
        match self.kind {
            CameraKind::Following => {
                self.target.x = SHAPE_ORBIT_RADIUS * angle.sin();
                self.target.z = SHAPE_ORBIT_RADIUS * angle.cos();
                self.position = FOLLOWING_CAMERA_POSITION;
                self.calculate_matrix();
            }
            CameraKind::Moving => {
                self.target = MOVING_CAMERA_TARGET;
                self.position.x = CAMERA_ORBIT_RADIUS * angle.sin();
                self.position.y = MOVING_CAMERA_Y;
                self.position.z = CAMERA_ORBIT_RADIUS * angle.cos();
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

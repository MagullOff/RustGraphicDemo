pub mod bresenham;
pub mod file_load;
pub mod vector;

pub fn get_angle(tick: f32) -> f32 {
    (tick * 2.5).rem_euclid(2.0 * std::f32::consts::PI) as f32
}

pub mod types {
    pub type Point3 = nalgebra::Point3<f32>;
    pub type Point2 = nalgebra::Point2<i32>;
    pub type Vector3 = nalgebra::Vector3<f32>;
    pub type Vector4 = nalgebra::Vector4<f32>;
    pub type Matrix4 = nalgebra::Matrix4<f32>;
}

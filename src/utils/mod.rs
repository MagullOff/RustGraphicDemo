pub mod bresenham;
pub mod file_load;
pub mod vector;

pub mod types {
    pub type Point3 = nalgebra::Point3<f32>;
    pub type Point2 = nalgebra::Point2<i32>;
    pub type Vector3 = nalgebra::Vector3<f32>;
    pub type Vector4 = nalgebra::Vector4<f32>;
    pub type Matrix4 = nalgebra::Matrix4<f32>;
}

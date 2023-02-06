use nalgebra::Vector3;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn from_array(arr: [f32; 3]) -> Vector {
        Vector::new(arr[0], arr[1], arr[2])
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn default() -> Vector {
        Vector::new(0.0, 0.0, 1.0)
    }

    pub fn norm(&self) -> Vector {
        let len = self.len();
        Vector::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn multiply(&self, coeff: f32) -> Vector {
        Vector::new(self.x * coeff, self.y * coeff, self.z * coeff)
    }

    pub fn cos(a: Vector, b: Vector) -> f32 {
        (a * b) / (a.len() * b.len())
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            (self.z * other.x - self.x * other.z) * (-1.0),
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector {}

impl From<Vector3<f32>> for Vector {
    fn from(value: Vector3<f32>) -> Self {
        Vector::new(value.x, value.y, value.z)
    }
}

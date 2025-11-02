use std::{fmt, ops};
use std::process::exit;
use std::f32::EPSILON;

#[derive(Debug, Copy, Clone)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d { x, y, z }
    }

    pub fn length_squared(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalised(&self) -> Vector3d {
        let len = self.length();
        if len > 0.0 {
            *self / len
        } else {
            Vector3d::new(0.0, 0.0, 0.0)
        }
    }

    pub fn normalise(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    pub fn dot(&self, rhs: &Vector3d) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    pub fn cross(&self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

impl PartialEq for Vector3d {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON &&
        (self.y - other.y).abs() < EPSILON &&
        (self.z - other.z).abs() < EPSILON
    }
}
impl Eq for Vector3d {}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vector3d> for Vector3d {
    fn add_assign(&mut self, rhs: Vector3d) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vector3d> for Vector3d {
    fn sub_assign(&mut self, rhs: Vector3d) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, len: f32) -> Vector3d {
        if len <= 0.0 {
            eprintln!("Dividing by {} is not legit.", len);
            exit(-1);
        }
        Vector3d {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl ops::Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Vector3d {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector3d> for f32 {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Vector3d {
        rhs * self
    }
}

impl ops::Mul<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Neg for Vector3d {
    type Output = Vector3d;

    fn neg(self) -> Vector3d {
        Vector3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vector3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector is ({}, {}, {})", self.x, self.y, self.z)
    }
}

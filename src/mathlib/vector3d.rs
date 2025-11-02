use std::ops;
use std::process::exit;

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

    // TODO
    // pub fn cross(&self, rhs: Vector3d)
}

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

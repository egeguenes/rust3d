use crate::mathlib::vector3d::Vector3d;
use crate::mathlib::ray::Ray;

pub struct Camera {
    pub position: Vector3d,
    pub forward_direction: Vector3d,
    pub up_direction: Vector3d,
    pub right_direction: Vector3d,
    pub focus: f32,
}

impl Camera {
    pub fn set_position(&mut self, vector: &Vector3d) {
        self.position = *vector;
    }
}

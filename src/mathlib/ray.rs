use crate::mathlib::vector3d::Vector3d;

pub struct Ray {
    pub origin: Vector3d,
    pub direction: Vector3d,
    pub length: f32,
}

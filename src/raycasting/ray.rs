use nalgebra::Vector3;

type Vector3f = Vector3<f32>;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3f {
        self.origin + self.direction * t
    }
}

pub struct HitPoint {
    pub position: Vector3f,
    /// normalized vector
    pub normal: Vector3f,
    pub front_face: bool
}

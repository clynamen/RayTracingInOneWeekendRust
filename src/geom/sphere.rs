use crate::types::Vector3f;
use crate::raycasting::ray::Ray;

pub struct Sphere {
   pub origin: Vector3f,
   pub radius: f32
}

pub fn hit_sphere(sphere: &Sphere, ray: Ray) -> bool {
    let oc = ray.origin - sphere.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius*sphere.radius;
    let discriminant = b*b - 4.0f32 * a * c;
    discriminant > 0.0f32
}
use crate::types::Vector3f;
use crate::raycasting::ray::{Ray, HitPoint};
use super::hittable::Hittable;

pub struct Sphere {
   pub origin: Vector3f,
   pub radius: f32
}

pub fn hit_sphere(sphere: &Sphere, ray: &Ray) -> Option<HitPoint> {
    let oc = ray.origin - sphere.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius*sphere.radius;
    let discriminant = b*b - 4.0f32 * a * c;
    if discriminant > 0.0f32 {
        let t =  (-b - discriminant.sqrt() ) / (2.0*a);
        let hit_position = ray.direction * t;
        let normal = (hit_position - sphere.origin).normalize();
        let is_front_face = normal.dot(&ray.direction) > 0f32;
        let hitpoint = HitPoint{
            position: hit_position,
            normal: normal,
            front_face: is_front_face
        };
        Some(hitpoint)
    } else {
        None
    }
}


impl Hittable for Sphere {

    fn ray_intersaction(&self, ray: &Ray) -> Option<HitPoint> {
        return hit_sphere(self, ray);
    }

}
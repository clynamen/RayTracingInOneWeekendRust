use crate::types::Vector3f;
use crate::raycasting::ray::{Ray, HitPoint};
use super::hittable::Hittable;

pub struct Sphere {
   pub origin: Vector3f,
   pub radius: f32
}

pub fn hit_sphere(sphere: &Sphere, ray: &Ray, t_min:f32, t_max:f32) -> Option<HitPoint> {
    let oc = ray.origin - sphere.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius*sphere.radius;
    let discriminant = b*b - 4.0f32 * a * c;
    if discriminant > 0.0f32 {
        let t1 = (-b - discriminant.sqrt() ) / (2.0*a);
        let t2 = (-b + discriminant.sqrt() ) / (2.0*a);
        let t1_hit_position = ray.direction * t1;
        let t2_hit_position = ray.direction * t2;

        // let t1_after_ray_origin = (t1_hit_position-ray.origin).dot(&ray.direction) > 0.0f32;
        // let t2_after_ray_origin = (t2_hit_position-ray.origin).dot(&ray.direction) > 0.0f32;
        let t1_after_ray_origin = t1 > t_min;
        // let t2_after_ray_origin = t2 > t_min;
        let t2_after_ray_origin = false;

        if t1_after_ray_origin || t2_after_ray_origin {
            let hit_position = if t1_after_ray_origin {
                t1_hit_position
            } else {
                t2_hit_position
            };
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
    } else {
        None
    }
}


impl Hittable for Sphere {

    fn ray_intersaction(&self, ray: &Ray, t_min:f32, t_max:f32) -> Option<HitPoint> {
        return hit_sphere(self, ray, t_min, t_max);
    }

}
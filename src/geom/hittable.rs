use crate::raycasting::ray::Ray;
use crate::raycasting::ray::HitPoint;

pub trait Hittable {

    fn ray_intersaction(&self, ray: &Ray, t_min:f32, t_max:f32) -> Option<HitPoint>;

}

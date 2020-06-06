use crate::raycasting::ray::Ray;
use crate::raycasting::ray::HitPoint;

pub trait Hittable {

    fn ray_intersaction(&self, ray: &Ray) -> Option<HitPoint>;

}

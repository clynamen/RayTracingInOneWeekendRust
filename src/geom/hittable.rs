use crate::raycasting::ray::Ray;
use crate::raycasting::ray::HitPoint;
use crate::material::material::Material;

pub trait Hittable {

    fn ray_intersaction(&self, ray: &Ray, t_min:f32, t_max:f32) -> Option<HitPoint>;
    fn material(&self) -> &Option<Box<dyn Material>>;

}

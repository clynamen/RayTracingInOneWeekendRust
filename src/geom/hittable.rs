use crate::raycasting::ray::Ray;
use crate::raycasting::ray::HitPoint;
use crate::material::material::Material;

use dyn_clone::{DynClone, clone_trait_object};

pub trait Hittable : Send+Sync+DynClone {

    fn ray_intersaction(&self, ray: &Ray, t_min:f32, t_max:f32) -> Option<HitPoint>;
    fn material(&self) -> &Option<Box<dyn Material>>;

}

clone_trait_object!(Hittable);
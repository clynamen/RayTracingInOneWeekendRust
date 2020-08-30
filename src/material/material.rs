use crate::raycasting::ray::HitPoint;
use crate::raycasting::ray::Ray;
use crate::types::{Vector3f};
use crate::geom::rand_geom::random_in_unit_sphere;

pub trait Material {
    fn scatter  (&self,
        ray: &Ray, rec: &HitPoint) -> Option<(Vector3f, Ray)>;
}

pub struct Lambertian {
    albedo : Vector3f
}

impl Lambertian {
    pub fn new(albedo: Vector3f) -> Lambertian {
        Lambertian{albedo}
    }
}

impl Material for Lambertian {

        fn scatter  (&self, ray: &Ray, rec: &HitPoint)-> Option<(Vector3f, Ray)> {
            let scatter_direction = rec.normal + random_in_unit_sphere();
            let scattered = Ray{origin: rec.position, direction: scatter_direction};
            let attenuation = self.albedo;
            Some((attenuation, scattered))
        }

}

pub struct Metal {
    albedo : Vector3f
}

impl Metal {
    pub fn new(albedo: Vector3f) -> Metal {
       Metal{albedo} 
    }
}

fn reflect(v: &Vector3f, n: &Vector3f) -> Vector3f {
    return v - 2f32*(v.dot(n)*n);
}

impl Material for Metal {

    fn scatter  (&self, ray: &Ray, rec: &HitPoint)-> Option<(Vector3f, Ray)> {
        let reflected : Vector3f = reflect(&ray.direction.normalize(), &rec.normal);
        let scattered = Ray{origin: rec.position, direction: reflected};
        let attenuation = self.albedo;
        let scatter_same_normal_direction = scattered.direction.dot(&rec.normal) > 0f32;
        if scatter_same_normal_direction {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

}
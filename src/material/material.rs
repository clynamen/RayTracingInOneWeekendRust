use crate::raycasting::ray::HitPoint;
use crate::raycasting::ray::Ray;
use crate::types::{Vector3f};
use crate::geom::rand_geom::random_in_unit_sphere;

pub trait Material : Send{
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
    albedo : Vector3f,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vector3f, fuzziness: f32) -> Metal {
        assert!(fuzziness >= 0f32 && fuzziness <= 1.0f32);
        Metal{albedo, fuzziness} 
    }
}

fn reflect(v: &Vector3f, n: &Vector3f) -> Vector3f {
    return v - 2f32*(v.dot(n)*n);
}



fn refract(uv: &Vector3f, n: &Vector3f, etai_over_etat: f32) -> Vector3f {
    let cos_theta = (-uv).dot(n);
    let r_out_perp: Vector3f = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0- r_out_perp.magnitude_squared()).abs().sqrt() * n;
    let r_out = r_out_perp + r_out_parallel;
    r_out
}

// vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
//     auto cos_theta = dot(-uv, n);
//     vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
//     vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
//     return r_out_perp + r_out_parallel;
// }

impl Material for Metal {

    fn scatter  (&self, ray: &Ray, rec: &HitPoint)-> Option<(Vector3f, Ray)> {
        let reflected : Vector3f = reflect(&ray.direction.normalize(), &rec.normal);
        let scattered_direction = reflected + self.fuzziness * random_in_unit_sphere(); 
        let scattered = Ray{origin: rec.position, direction: scattered_direction};
        let attenuation = self.albedo;
        let scatter_same_normal_direction = scattered.direction.dot(&rec.normal) > 0f32;
        if scatter_same_normal_direction {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

}

pub struct Dielectric {
    reflective_index: f32
}

impl Dielectric {
    pub fn new(reflective_index: f32) -> Dielectric {
        Dielectric{reflective_index}
    }
}

impl Material for Dielectric {

    fn scatter  (&self, ray: &Ray, rec: &HitPoint)-> Option<(Vector3f, Ray)> {
        let attenuation = Vector3f::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face { 1.0 / self.reflective_index } else {self.reflective_index};
        let unit_direction = ray.direction.normalize();
        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        let scattered = Ray{origin: rec.position, direction: refracted};
        Some((attenuation, scattered))
    }
}

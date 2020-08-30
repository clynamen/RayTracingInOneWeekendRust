extern crate color;
extern crate nalgebra;
extern crate rand;

mod geom;
mod image;
mod ppm;
mod raycasting;
mod renderer;
mod types;
mod material;

use crate::geom::hittable::Hittable;
use crate::geom::sphere::Sphere;
use crate::types::Vector3f;
use crate::renderer::camera::Camera;
use crate::material::material::{Lambertian, Metal};

fn main() {
    let renderer = renderer::renderer::Renderer::new();
    let camera = Camera::new(300);

    let material_ground = Lambertian::new(Vector3f::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vector3f::new(0.7, 0.3, 0.3));
    let material_left   = Metal::new(Vector3f::new(0.8, 0.8, 0.8));
    let material_right  = Metal::new(Vector3f::new(0.8, 0.6, 0.2));

    let sphere_ground = Sphere {
        origin: Vector3f::new(0f32, -100.5f32, -1.0f32),
        radius: 100f32,
        material: Some(Box::new(material_ground))
    };
    let sphere_center = Sphere {
        origin: Vector3f::new(0f32, 0f32, -1f32),
        radius: 0.5f32,
        material: Some(Box::new(material_center))
    };
    let sphere_left = Sphere {
        origin: Vector3f::new(-1f32, 0f32, -1f32),
        radius: 0.5f32,
        material: Some(Box::new(material_left))
    };
    let sphere_right = Sphere {
        origin: Vector3f::new(1f32, 0f32, -1f32),
        radius: 0.5f32,
        material: Some(Box::new(material_right))
    };

    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(sphere_ground),
        Box::new(sphere_left), 
        Box::new(sphere_right), 
        Box::new(sphere_center), 
    ];

    let image = renderer.run(&camera, &hittables);
    ppm::save_image_to_ppm(
        image.data.as_slice(),
        image.size.width(),
        image.size.height(),
        "output.ppm",
    )
}

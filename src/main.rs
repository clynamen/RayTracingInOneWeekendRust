extern crate color;
extern crate nalgebra;
extern crate rand;

mod geom;
mod image;
mod ppm;
mod raycasting;
mod renderer;
mod types;

use crate::geom::hittable::Hittable;
use crate::geom::sphere::Sphere;
use crate::types::Vector3f;
use crate::renderer::camera::Camera;

fn main() {
    let renderer = renderer::renderer::Renderer::new();
    let camera = Camera::new(300);

    let sphere1 = Sphere {
        origin: Vector3f::new(0f32, 0f32, -2.0f32),
        radius: 0.5f32,
    };

    let sphere2 = Sphere {
        origin: Vector3f::new(0f32, -2f32, -2f32),
        radius: 2f32,
    };

    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(sphere1), 
        Box::new(sphere2),
    ];

    let image = renderer.run(&camera, &hittables);
    ppm::save_image_to_ppm(
        image.data.as_slice(),
        image.size.width(),
        image.size.height(),
        "output.ppm",
    )
}

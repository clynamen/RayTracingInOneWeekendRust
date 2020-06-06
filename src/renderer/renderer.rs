use crate::types::{Size2i, Vector2i, Vector3f, Rgb8};
use crate::types::{vector3f_to_rgb8, normal3f_to_rgb8};
use crate::raycasting::ray::Ray;
use crate::image::image::Image;
use crate::renderer::camera::Camera;
use crate::geom::hittable::Hittable;
use std::vec::Vec;
use std::boxed::Box;

pub struct Renderer {
}

impl Renderer {

    pub fn new() -> Renderer {
        Renderer{}
    }

    fn eval_background_color(&self, r: &Ray) -> Rgb8 {
        let unit_direction = r.direction.normalize();
        let t = 0.5*(unit_direction.y + 1.0);
        let color = (1.0-t)*Vector3f::new(1.0, 1.0, 1.0) + t*Vector3f::new(0.5, 0.7, 1.0);
        vector3f_to_rgb8(color)
    }

    fn eval_ray_color(&self, r: &Ray, hittables: &Vec::<Box<dyn Hittable>>) -> Rgb8 {
        let mut color = self.eval_background_color(r);
        for hittable in hittables {
            let sphere_hitpoint = hittable.ray_intersaction(r);
            match sphere_hitpoint {
                Some(hitpoint) => {
                    color = normal3f_to_rgb8(hitpoint.normal)
                },
                None => {}
            }
        }
        color
    }

    pub fn run(&self, camera: &Camera, hittables: &Vec::<Box<dyn Hittable>>) -> Image {

        let image_size = Size2i::new(camera.viewport.image_height(),
                                     camera.viewport.image_width());
        let mut image = Image::new(image_size);

        for image_y in 0..camera.viewport.image_height() {
            for image_x in 0..camera.viewport.image_width() {
                let ray = camera.get_ray_from_image_yx(image_y, image_x);
                let pixel_color = self.eval_ray_color(&ray, &hittables);
                let image_position = Vector2i::new(image_x, image_y);
                image.set_pixel(image_position, pixel_color);
            }
        }

        image
    }

}

use crate::geom::hittable::Hittable;
use crate::image::image::Image;
use crate::raycasting::ray::Ray;
use crate::renderer::camera::Camera;
use crate::types::{normal3f_to_rgb8, vector3f_to_rgb8, rgb8_to_vector3f};
use crate::types::{Rgb8, Size2i, Vector2i, Vector3f};
use std::boxed::Box;
use std::vec::Vec;

pub struct RendererSettings {
    antialiasing_on: bool,
    antialiasing_samples: u32,
}

pub struct Renderer {
    settings: RendererSettings,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            settings: RendererSettings {
                antialiasing_on: true,
                antialiasing_samples: 50,
            },
        }
    }

    fn eval_background_color(&self, r: &Ray) -> Rgb8 {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color = (1.0 - t) * Vector3f::new(1.0, 1.0, 1.0) + t * Vector3f::new(0.5, 0.7, 1.0);
        vector3f_to_rgb8(color)
    }

    fn eval_ray_color(&self, r: &Ray, hittables: &Vec<Box<dyn Hittable>>) -> Rgb8 {
        let mut color = self.eval_background_color(r);
        for hittable in hittables {
            let sphere_hitpoint = hittable.ray_intersaction(r);
            match sphere_hitpoint {
                Some(hitpoint) => color = normal3f_to_rgb8(hitpoint.normal),
                None => {}
            }
        }
        color
    }

    pub fn eval_pixel_color(
        &self,
        camera: &Camera,
        pixel_position: Vector2i,
        hittables: &Vec<Box<dyn Hittable>>,
    ) -> Rgb8 {
        if self.settings.antialiasing_on {
            let mut pixel_color_vector = Vector3f::zeros();
            for _i in 0..self.settings.antialiasing_samples {
                let ray = camera.get_random_ray_from_image_xy(pixel_position);
                let sample_color = self.eval_ray_color(&ray, &hittables);
                pixel_color_vector += rgb8_to_vector3f(sample_color);
            }
            vector3f_to_rgb8(pixel_color_vector / self.settings.antialiasing_samples as f32)
        } else {
            let ray = camera.get_ray_from_image_xy(pixel_position);
            self.eval_ray_color(&ray, &hittables)
        }
    }

    pub fn run(&self, camera: &Camera, hittables: &Vec<Box<dyn Hittable>>) -> Image {
        let image_size = Size2i::new(
            camera.viewport.image_height(),
            camera.viewport.image_width(),
        );
        let mut image = Image::new(image_size);

        for image_y in 0..camera.viewport.image_height() {
            for image_x in 0..camera.viewport.image_width() {
                let pixel_position = Vector2i::new(image_x, image_y);
                let pixel_color = self.eval_pixel_color(camera, pixel_position, &hittables);
                image.set_pixel(pixel_position, pixel_color);
            }
        }

        image
    }
}

use crate::geom::hittable::Hittable;
use crate::image::image::Image;
use crate::raycasting::ray::Ray;
use crate::renderer::camera::Camera;
use crate::types::{normal3f_to_rgb8, vector3f_to_rgb8, rgb8_to_vector3f};
use crate::types::{Rgb8, Size2i, Vector2i, Vector3f, scale_rgb8, PointwiseSqrtExt };
use std::boxed::Box;
use std::vec::Vec;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vector3f {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-1.0, 1.0);
    let y = rng.gen_range(-1.0, 1.0);
    let z = rng.gen_range(-1.0, 1.0);
    Vector3f::new(x, y, z).normalize()
}

pub struct RendererSettings {
    antialiasing_on: bool,
    antialiasing_samples: u32,
    max_depth: u32
}

pub struct Renderer {
    settings: RendererSettings,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            settings: RendererSettings {
                antialiasing_on: true,
                antialiasing_samples: 100,
                max_depth: 50
            },
        }
    }

    fn eval_background_color(&self, r: &Ray) -> Vector3f {
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let color =  (1.0 - t)
             * Vector3f::new(1.0, 1.0, 1.0) +
             t * Vector3f::new(0.5, 0.7, 1.0) ;
        color
    }

    fn eval_ray_color(&self, r: &Ray, hittables: &Vec<Box<dyn Hittable>>, remaining_depth: u32) -> Vector3f {
        let mut color = self.eval_background_color(r);
        if remaining_depth == 0 {
            color = Vector3f::new(0.0f32, 0.0f32, 0.0f32)
            // color =  (hitpoint.normal + Vector3f::new(1f32, 1f32, 1f32)) * 0.5f32
        } else {
            for hittable in hittables {
                let sphere_hitpoint = hittable.ray_intersaction(r, 0.001, 10000.0);
                match sphere_hitpoint {
                    Some(hitpoint) => {
                        let new_target = hitpoint.position + hitpoint.normal + random_in_unit_sphere();
                        let new_ray = Ray{
                            origin: hitpoint.position, 
                            direction: new_target - hitpoint.position
                        };
                        // let normal_color = (hitpoint.normal + Vector3f::new(1f32, 1f32, 1f32)) * 0.5f32;
                        let normal_color = Vector3f::zeros();
                        // let normal_color = Vector3f::new(0.3f32, 0.3f32, 0.3f32);
                        // return Vector3f::new(1.0, 0.0, 0.0 )
                        let reflected_color = self.eval_ray_color(&new_ray, hittables, remaining_depth-1) * 0.5f32;
                        // let reflected_color = Vector3f::zeros();
                        color = normal_color + reflected_color;
                    },
                    None => {}
                }
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
        let color_vector : Vector3f = if self.settings.antialiasing_on {
            let mut pixel_color_vector = Vector3f::zeros();
            for _i in 0..self.settings.antialiasing_samples {
                let ray = camera.get_random_ray_from_image_xy(pixel_position);
                let sample_color = self.eval_ray_color(&ray, &hittables, self.settings.max_depth);
                pixel_color_vector += sample_color;
            }
            let color_vector : Vector3f = pixel_color_vector / self.settings.antialiasing_samples as f32;

            // apply gamma 2 correction
            let corrected_color_vector = color_vector.pointwise_sqrt();

            // println!("{}", color_vector);
            corrected_color_vector
        } else {
            let ray = camera.get_ray_from_image_xy(pixel_position);
            let color_vector = self.eval_ray_color(&ray, &hittables, self.settings.max_depth);
            // println!("{}", color_vector);
            color_vector
        };
        vector3f_to_rgb8(color_vector)
    }

    pub fn run(&self, camera: &Camera, hittables: &Vec<Box<dyn Hittable>>) -> Image {
        let image_size = Size2i::new(
            camera.viewport.image_height(),
            camera.viewport.image_width(),
        );
        let mut image = Image::new(image_size);

        // render from upper left corner
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

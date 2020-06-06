use crate::types::{Size2i, Vector2i, Vector3f, Rgb8};
use crate::types::vector3f_to_rgb8;
use crate::raycasting::ray::Ray;
use crate::image::image::Image;
use super::viewport::Viewport;

pub struct Renderer {
    viewport: Viewport
}

impl Renderer {

    pub fn new() -> Renderer {
        let viewport = Viewport::new_by_width(16.0/9.0, 100);
        Renderer {
            viewport: viewport
        }
    }

    fn eval_background_color(&self, r: &Ray) -> Rgb8 {
        let unit_direction = r.direction.normalize();
        let t = 0.5*(unit_direction.y + 1.0);
        let color = (1.0-t)*Vector3f::new(1.0, 1.0, 1.0) + t*Vector3f::new(0.5, 0.7, 1.0);
        vector3f_to_rgb8(color)
    }

    fn eval_ray_color(&self, r: &Ray) -> Rgb8 {
        self.eval_background_color(r)
    }

    pub fn run(&self) -> Image {

        let image_size = Size2i::new(self.viewport.image_height(), self.viewport.image_width());
        let mut image = Image::new(image_size);

        for image_y in 0..self.viewport.image_height() {
            for image_x in 0..self.viewport.image_width() {
                let ray = self.viewport.get_ray_from_image_yx(image_y, image_x);
                let pixel_color = self.eval_ray_color(&ray);
                let image_position = Vector2i::new(image_x, image_y);
                image.set_pixel(image_position, pixel_color);
            }
        }

        image
    }

}

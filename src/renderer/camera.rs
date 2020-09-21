use crate::raycasting::ray::Ray;
use crate::renderer::viewport::Viewport;
use crate::types::Vector2i;
use crate::types::Vector3f;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vector3f,
    pub viewport: Viewport,
}

impl Camera {

    pub fn new(width: i32, height: i32) -> Camera {
        let aspect_ratio = width as f32 / height as f32;
        // let aspect_ratio = 1f32;
        let viewport = Viewport::new_by_width(aspect_ratio, width);
        Camera {
            origin: Vector3f::new(0.0f32, 0.0f32, 0f32),
            viewport: viewport,
        }
    }

    pub fn fps_move(&mut self, wasd_vector: Vector3f) {
        self.origin += wasd_vector;
    }

    pub fn get_ray_from_image_xy(&self, xy: Vector2i) -> Ray {
        self.get_ray_from_image_yx(xy.y as f32, xy.x as f32)
    }

    pub fn get_random_ray_from_image_xy(&self, xy: Vector2i) -> Ray {
        let mut rng = rand::thread_rng();
        self.get_ray_from_image_yx(xy.y as f32 + rng.gen::<f32>() - 0.5f32,
                                   xy.x as f32 + rng.gen::<f32>() - 0.5f32)
    }

    /// get a ray from a pixel in the image (between (0,0) and (image_y, image_x))
    /// ray x will be between (-viewport_width/2, +viewport_width/2)
    /// y from image (0, image_y) is mapped from  to (viewport_height/2, -viewport_height/2)
    /// origin at top left
    /// ray exit at camera -z
    pub fn get_ray_from_image_yx(&self, image_y: f32, image_x: f32) -> Ray {
        let image_v = self.viewport.to_image_v(image_y);
        let image_u = self.viewport.to_image_u(image_x);
        let direction: Vector3f = image_u * self.viewport.horizontal()
            + image_v * self.viewport.vertical()
            + self.front() * self.focal_length();
        Ray {
            origin: self.origin,
            direction: direction,
        }
    }

    pub fn front(&self) -> Vector3f {
        Vector3f::new(0f32, 0f32, -1f32)
    }

    pub fn focal_length(&self) -> f32 {
        0.6
    }

}

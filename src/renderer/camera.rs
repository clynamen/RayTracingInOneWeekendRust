use crate::raycasting::ray::Ray;
use crate::renderer::viewport::Viewport;
use crate::types::Vector2i;
use crate::types::Vector3f;

pub struct Camera {
    pub origin: Vector3f,
    pub viewport: Viewport,
}

impl Camera {

    pub fn new() -> Camera {
        let viewport = Viewport::new_by_width(16.0 / 9.0, 250);
        Camera {
            origin: Vector3f::new(0f32, 0f32, 0f32),
            viewport: viewport,
        }
    }

    pub fn get_ray_from_image_xy(&self, xy: Vector2i) -> Ray {
        self.get_ray_from_image_yx(xy.y, xy.x)
    }

    pub fn get_ray_from_image_yx(&self, image_y: i32, image_x: i32) -> Ray {
        let image_v = self.viewport.to_image_v(image_y);
        let image_u = self.viewport.to_image_u(image_x);
        let direction: Vector3f = image_u * self.viewport.horizontal()
            + image_v * self.viewport.vertical()
            - self.front() * self.focal_length();
        Ray {
            origin: self.origin,
            direction: direction,
        }
    }

    pub fn front(&self) -> Vector3f {
        Vector3f::new(0f32, 0f32, -1f32)
    }

    pub fn focal_length(&self) -> f32 {
        1.0
    }

    pub fn lower_left_corner(&self) -> Vector3f {
        self.origin
            - self.viewport.horizontal() / 2f32
            - self.viewport.vertical()   / 2f32
            - Vector3f::new(0f32, 0f32, self.focal_length())
    }

}

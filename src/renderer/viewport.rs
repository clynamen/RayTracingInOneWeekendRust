use crate::raycasting::ray::Ray;
use crate::types::{Vector3f};

pub struct Viewport {
    width: i32,
    height: i32,
}

impl Viewport {
    pub fn get_ray_from_image_yx(&self, image_y: i32, image_x: i32) -> Ray {
        let image_v = self.to_image_v(image_y);
        let image_u = self.to_image_u(image_x);
        let direction : Vector3f = image_u * self.horizontal() + image_v * self.vertical()
                - Vector3f::new(0f32, 0f32, -1f32) * self.focal_length();
        Ray {
            origin: self.origin(),
            direction: direction,
        }
    }

    pub fn to_image_v(&self, image_y: i32) -> f32 {
        assert!(image_y >= 0 && image_y < self.image_height());
        image_y as f32 / self.image_height() as f32 - 0.5
    }

    pub fn to_image_u(&self, image_x: i32) -> f32 {
        assert!(image_x >= 0 && image_x < self.image_width());
        image_x as f32 / self.image_width() as f32 - 0.5
    }

    pub fn image_width(&self) -> i32 {
        self.width
    }

    pub fn image_height(&self) -> i32 {
        self.height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn viewport_height(&self) -> f32 {
        1.0
    }

    pub fn viewport_width(&self) -> f32 {
        self.aspect_ratio() * self.viewport_height()
    }

    pub fn focal_length(&self) -> f32 {
        1.0
    }

    pub fn origin(&self) -> Vector3f {
        Vector3f::new(0f32, 0f32, 0f32)
    }

    pub fn horizontal(&self) -> Vector3f {
        Vector3f::new(self.viewport_width(), 0f32, 0f32)
    }

    pub fn vertical(&self) -> Vector3f {
        Vector3f::new(0f32, self.viewport_height(), 0f32)
    }

    pub fn lower_left_corner(&self) -> Vector3f {
        self.origin()
            - self.horizontal() / 2f32
            - self.vertical() / 2f32
            - Vector3f::new(0f32, 0f32, self.focal_length())
    }

    pub fn new_by_width(aspect_ratio: f32, width: i32) -> Viewport {
        let image_width = width;
        let image_height = (image_width as f32 / aspect_ratio) as i32;

        Viewport {
            width: image_width,
            height: image_height,
        }
    }
}

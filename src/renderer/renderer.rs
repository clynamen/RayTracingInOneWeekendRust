use crate::types::{Size2i, Vector3f};

struct Renderer {}

impl Renderer {
    fn run(&self) -> Vec<u8> {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 384;
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let image_size = Size2i::new(image_height, image_width);

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vector3f::new(0f32, 0f32, 0f32);
        let horizontal = Vector3f::new(viewport_width, 0f32, 0f32);
        let vertical = Vector3f::new(0f32, viewport_height, 0f32);
        let lower_left_corner =
            origin - horizontal / 2f32 - vertical / 2f32 - Vector3f::new(0f32, 0f32, focal_length);

        let image_buffer = Vec::<u8>::new();

        // for (int j = image_height-1; j >= 0; --j) {
        //     std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        //     for (int i = 0; i < image_width; ++i) {
        //         auto u = double(i) / (image_width-1);
        //         auto v = double(j) / (image_height-1);
        //         ray r(origin, lower_left_corner + u*horizontal + v*vertical - origin);
        //         color pixel_color = ray_color(r);
        //         write_color(std::cout, pixel_color);
        //     }
        // }

        image_buffer
    }
}

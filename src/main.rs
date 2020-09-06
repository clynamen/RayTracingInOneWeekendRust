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
use crate::types::{Vector3f, Vector2i};
use crate::renderer::camera::Camera;
use crate::material::material::{Lambertian, Metal, Dielectric};

extern crate piston_window;
extern crate image as piston_image;

use graphics::rectangle::rectangle_by_corners;


use piston_window::*;
// use image::Image as PistonImage;
use piston_image::ImageBuffer;
use piston_image::buffer::ConvertBuffer;
use std::thread;
use std::sync::mpsc::channel;

type RgbImageU8Vec = ImageBuffer::<piston_image::Rgb<u8>, std::vec::Vec<u8>>;
type RgbaImageU8Vec = ImageBuffer::<piston_image::Rgba<u8>, std::vec::Vec<u8>>;

fn renderer_image_to_piston_imagebuffer(src: image::image::Image) -> RgbImageU8Vec{
    let dest = RgbImageU8Vec::from_raw(
        src.size.width() as u32, src.size.height() as u32, src.data).unwrap();
    dest
}


fn rgb2rgba(src: RgbImageU8Vec) -> RgbaImageU8Vec {
    let dest =    src.convert();
    dest
}

fn main() {
    let camera_width : u32 = 200;
    let camera_height: u32 = 120;
    let camera_size = Vector2i::new(camera_width as i32, camera_height as i32);

    let mut window: PistonWindow = 
        WindowSettings::new("renderer", [camera_width, camera_height])
        .exit_on_esc(true).build().unwrap();

    let game: Game = Game::new(camera_size);
    // Create a simple streaming channel
    let (renderer_tx, renderer_rx) = channel();

    let renderer_thread = thread::spawn( move || {
        while true {
            let rendered_image = game.render();
            println!("rendered!");
            let image_buffer = renderer_image_to_piston_imagebuffer(rendered_image);
            let image_buffer_rgba = rgb2rgba(image_buffer);
            renderer_tx.send(image_buffer_rgba);
        }
    });

    
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    while let Some(e) = window.next() {
        // let rendered_image = game.render();
        window.draw_2d(&e, |c, g, _device| {
            // rectangle([1.0, 0.0, 0.0, 1.0], // red
            //           [0.0, 0.0, 100.0, 100.0],
            //           c.transform, g);
            match renderer_rx.try_recv() {
                Ok(image) => {
                    println!("received rendered image");
                    clear([1.0; 4], g);
                    let img = Image::new().rect(rectangle_by_corners(0.0, 0.0,
                        image.width().into(), 
                        image.height().into()));
                    let texture_settings = TextureSettings::new();
                    let texture = Texture::from_image(&mut texture_context, &image, &texture_settings ).unwrap();
                    img.draw(&texture, &c.draw_state, c.transform.scale(5.0, 5.0), g);
                },
                Err(e) => {

                }
            }
        });
    }
}

struct Game {
    renderer: renderer::renderer::Renderer,
    camera: Camera
}

impl Game {

    pub fn new(camera_size: Vector2i) -> Game {
        let renderer = renderer::renderer::Renderer::new();
        let camera = Camera::new(camera_size.x, camera_size.y);

        Game {
            renderer,
            camera
        }
    }

    pub fn get_hittables(&self) -> Vec<Box<dyn Hittable>>  {

        let material_ground = Lambertian::new(Vector3f::new(0.8, 0.8, 0.0));
        let material_center = Lambertian::new(Vector3f::new(0.7, 0.3, 0.3));
        // let material_left   = Metal::new(Vector3f::new(0.8, 0.8, 0.8), 0.3);
        // let material_center = Dielectric::new(1.5f32);
        let material_left   = Dielectric::new(1.5f32);
        let material_right  = Metal::new(Vector3f::new(0.8, 0.6, 0.2), 0.05);

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

        hittables
    }

    pub fn render(&self) -> image::image::Image {
        let hittables = self.get_hittables();
        let image = self.renderer.run(&self.camera, &hittables);
        ppm::save_image_to_ppm(
            image.data.as_slice(),
            image.size.width(),
            image.size.height(),
            "output.ppm",
        );
        image
    }
}

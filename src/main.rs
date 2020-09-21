extern crate color;
extern crate nalgebra;
extern crate rand;
extern crate arc_swap;
extern crate dyn_clone;

mod geom;
mod image;
mod ppm;
mod raycasting;
mod renderer;
mod types;
mod material;

use arc_swap::ArcSwap;
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
use std::sync::mpsc::{channel, Sender, Receiver};

type RgbImageU8Vec = ImageBuffer::<piston_image::Rgb<u8>, std::vec::Vec<u8>>;
type RgbaImageU8Vec = ImageBuffer::<piston_image::Rgba<u8>, std::vec::Vec<u8>>;
type MainTextureContext = piston_window::TextureContext<
    gfx_device_gl::Factory, 
    gfx_device_gl::Resources, 
    gfx_device_gl::CommandBuffer>;

fn renderer_image_to_piston_imagebuffer(src: image::image::Image) -> RgbImageU8Vec{
    let dest = RgbImageU8Vec::from_raw(
        src.size.width() as u32, src.size.height() as u32, src.data).unwrap();
    dest
}


fn rgb2rgba(src: RgbImageU8Vec) -> RgbaImageU8Vec {
    let dest =    src.convert();
    dest
}

use std::collections::HashSet;

type ArcMutex<T> = std::sync::Arc<std::sync::Mutex<T>>;

fn make_arc_mutex<T>(value: T) -> ArcMutex<T>{
    std::sync::Arc::new(std::sync::Mutex::new(value))
}

struct UserInput {
    pressed_keys: HashSet<Key>,
    exit_requested: bool
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            pressed_keys: HashSet::new(),
            exit_requested: false
        }
    }
}

fn start_render_thread(user_input_rx: Receiver<UserInput>,
        renderer_framebuffer_tx: Sender<RgbaImageU8Vec>,
        game_conf: GameConf,
        game_state: ArcSwap<GameState>) -> std::thread::JoinHandle<()> {

    let game: Game = Game::new();

    let renderer_thread = thread::spawn( move || {
        let mut running = true;
        while running {
            if let Ok(user_input) = user_input_rx.try_recv() {
                if user_input.exit_requested {
                    running = false;
                    break;
                }
            }



            println!("{:?}", game_state.load().camera.origin);
            let rendered_image = game.render(game_state.load().as_ref());
            println!("rendered!");
            let image_buffer = renderer_image_to_piston_imagebuffer(rendered_image);
            let image_buffer_rgba = rgb2rgba(image_buffer);
            renderer_framebuffer_tx.send(image_buffer_rgba);
        }
        println!("exit from render thread");
    });
    renderer_thread
}


struct GameConf{
    camera_size: Vector2i
}

fn generate_user_input(e: &Event) -> UserInput {
    let mut user_input = UserInput::new();

    if let Some(Button::Keyboard(key)) = e.press_args() {
        user_input.pressed_keys.insert(key);
    }

    if user_input.pressed_keys.contains(&Key::Escape) {
        println!("exiting from main thread");
        user_input.exit_requested = true;
    }

    user_input
}



fn draw_window(window: &mut PistonWindow, e: &Event, 
        renderer_framebuffer_rx: &Receiver<RgbaImageU8Vec>,
        texture_context: &mut MainTextureContext) {

    window.draw_2d(e, |c, g, _device| {
        match renderer_framebuffer_rx.try_recv() {
            Ok(image) => {
                println!("received rendered image");
                clear([1.0; 4], g);
                let img = Image::new().rect(rectangle_by_corners(0.0, 0.0,
                    image.width().into(), 
                    image.height().into()));
                let texture_settings = TextureSettings::new();
                let texture = Texture::from_image(texture_context, &image, &texture_settings ).unwrap();
                img.draw(&texture, &c.draw_state, c.transform.scale(5.0, 5.0), g);
            },
            Err(e) => {

            }
        }
    });
}

fn make_wasd_vector(user_input: &UserInput) -> Vector3f {
    let mut wasd_vector = Vector3f::new(0.0, 0.0, 0.0);
    if user_input.pressed_keys.contains(&Key::W) {
        wasd_vector.z += 1.0;
    } else if user_input.pressed_keys.contains(&Key::S) {
        wasd_vector.z -= 1.0;
    }

    if(user_input.pressed_keys.contains(&Key::A)) {
        wasd_vector.x += 1.0;
    } else if (user_input.pressed_keys.contains(&Key::D)) {
        wasd_vector.x -= 1.0;
    }

    wasd_vector
}

fn update_camera(user_input: &UserInput, camera: Camera) -> Camera {
    let mut new_camera = camera;
    let mut wasd_vector = make_wasd_vector(user_input);
    new_camera.fps_move(wasd_vector);
    new_camera
}

fn update_game_state(user_input: &UserInput, previous_game_state: &GameState) -> GameState {
    let mut new_game_state = dyn_clone::clone(previous_game_state);
    new_game_state.camera = update_camera(user_input, new_game_state.camera);
    new_game_state
}

fn main_thread(mut window: PistonWindow,
    user_input_tx: Sender<UserInput>,
    renderer_framebuffer_rx: Receiver<RgbaImageU8Vec>,
    game_state: ArcSwap<GameState>) {

    let mut running = true;
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    while let Some(e) = window.next() {
        if !running  {
            break
        }

        let user_input = generate_user_input(&e);

        let previous_game_state = game_state.load();
        let new_game_state = update_game_state(&user_input, previous_game_state.as_ref()); 
        game_state.store(std::sync::Arc::new(new_game_state));

        if user_input.exit_requested {
            println!("exiting from main thread");
            running = false;
            user_input_tx.send(user_input);
        }

        draw_window(&mut window, &e, &renderer_framebuffer_rx, &mut texture_context);
    }

}

fn get_camera_size() -> Vector2i {
    let camera_width : u32 = 200;
    let camera_height: u32 = 120;
    let camera_size = Vector2i::new(camera_width as i32, camera_height as i32);
    camera_size
}

fn make_initial_game_state(camera_size: Vector2i) -> GameState {
    GameState {
        camera: Camera::new(camera_size.x, camera_size.y),
        hittables: make_default_hittables()
    }
}

fn main() {
    let camera_size = get_camera_size();

    let window: PistonWindow = 
        WindowSettings::new("renderer",
        [camera_size.x as u32, camera_size.y as u32])
        .exit_on_esc(true).build().unwrap();

    let game_conf = GameConf { camera_size };

    let (renderer_framebuffer_tx, renderer_framebuffer_rx) = channel();
    let (user_input_tx, user_input_rx) = channel::<UserInput>();

    let initial_game_state = make_initial_game_state(camera_size);
    // let game_state = make_arc_mutex(initial_game_state);
    let game_state = ArcSwap::new(std::sync::Arc::new(initial_game_state));

    let renderer_thread = start_render_thread(
        user_input_rx, renderer_framebuffer_tx,
        game_conf, game_state.clone(),
    );

    main_thread(window, user_input_tx, 
        renderer_framebuffer_rx, 
        game_state);

    renderer_thread.join().unwrap();
}

struct Game {
    renderer: renderer::renderer::Renderer
}


#[derive(Clone)]
struct GameState {
    hittables: Vec<Box<dyn Hittable>>,
    camera: Camera
}

impl GameState {
    pub fn new(camera: Camera) -> GameState {
        GameState {
            hittables: Vec::new(),
            camera: camera
        }
    }
}

pub fn make_default_hittables() -> Vec<Box<dyn Hittable>>  {
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


impl Game {

    pub fn new() -> Game {
        let renderer = renderer::renderer::Renderer::new();

        Game {
            renderer
        }
    }

    pub fn render(&self, game_state: &GameState) -> image::image::Image {
        let hittables = &game_state.hittables;
        let camera = &game_state.camera;
        let image = self.renderer.run(camera, hittables);
        ppm::save_image_to_ppm(
            image.data.as_slice(),
            image.size.width(),
            image.size.height(),
            "output.ppm",
        );
        image
    }
}

extern crate nalgebra;
extern crate color;

mod types;
mod ppm;
mod raycasting;
mod geom;
mod renderer;
mod image;



fn main() {
    let renderer = renderer::renderer::Renderer::new();
    let image = renderer.run();
    ppm::save_image_to_ppm(image.data.as_slice(), image.size.width(), image.size.height(), "output.ppm")
}

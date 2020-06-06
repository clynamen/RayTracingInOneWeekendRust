use crate::types::{Size2i, Vector2i, Rgb8};
use color::Rgb;
use std::fmt::*;

#[derive(Debug)]
pub struct Image {
    pub size: Size2i,
    pub data: Vec<u8>
}

impl Image {

    pub fn new(size: Size2i) -> Image {
        let mut data = Vec::<u8>::new();
        let data_size = (size.width()*size.height()*3) as usize;
        data.resize(data_size, 0);
        Image::new_with_data(size, data)
    }

    pub fn new_with_data(size: Size2i, data: Vec<u8>) -> Image {
        Image {
            size,
            data
        }
    }

    pub fn set_pixel(&mut self, position: Vector2i, rgb: Rgb8) {
        assert!(position.y >= 0 && position.y < self.size.height());
        assert!(position.x >= 0 && position.x < self.size.width());
        let image_size = self.size.height() * self.size.width();
        let pixel_index = position.y * self.size.width() + position.x;
        self.data[(0*image_size+pixel_index) as usize] = rgb.r;
        self.data[(1*image_size+pixel_index) as usize] = rgb.g;
        self.data[(2*image_size+pixel_index) as usize] = rgb.b;
    }

}

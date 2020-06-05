use crate::types::Size2f;

#[derive(Debug, Display)]
pub struct Image {
    pub size: Size2f,
    pub data: Vec<u8>
}

impl Image {
    pub fn new(size: Size2f, data: Vec<u8>) -> Image {
        Image {
            size,
            data
        }
    }
}

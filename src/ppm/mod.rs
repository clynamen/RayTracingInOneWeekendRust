use std::io::prelude::*;
use std::fs::File;

pub fn save_image_to_ppm(buffer: &[u8], width: i32, height: i32, fname: &str) {
    let mut file = File::create(fname).unwrap();
    write!(file, "P3\n{} {}\n255\n", width, height).unwrap();
    let image_size : usize = (height*width) as usize;
    for y in 0..height {
        for x in 0..width {
            let i : usize = (y*width+x) as usize;
            let r : u8 = buffer[0*image_size+i];
            let g : u8 = buffer[1*image_size+i];
            let b : u8 = buffer[2*image_size+i];

            write!(file, "{} {} {}\n", r, g, b).unwrap();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_save_works() {
        let bufvec = vec![
            255, 255, 255,
            0,   0,   0,
            0,   0,   0,

            0,   0,   0,
            255, 255, 255,
            0,   0,   0,

            0,   0,   0,
            0,   0,   0,
            255, 255, 255,
        ];
        let buffer = bufvec.as_slice();
        let width = 3;
        let height = 3;
        let fname = "/tmp/output.ppm";

        save_image_to_ppm(buffer, width, height, fname);
        let expected_fname = "test/data/test1.ppm";
        let expected_data = std::fs::read_to_string(expected_fname).unwrap();
        let written_data = std::fs::read_to_string(fname).unwrap();
        assert_eq!(expected_data, written_data);
    }
}

use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Debug, Clone, Copy)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Float3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.z.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }
}

pub type Grid = [[Float3; 64]; 64];

pub fn create_test_image() -> Grid {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;

    let mut image: [[Float3; WIDTH]; HEIGHT] = [[Float3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r = x as f64 / (WIDTH - 1) as f64;
            let g = y as f64 / (HEIGHT - 1) as f64;
            let b = 0.0;

            image[x][y] = Float3::new(r, g, b);
        }
    }

    image
}

pub fn save_as_bmp(image: &[[Float3; 64]; 64], path: &str) -> Result<(), image::ImageError> {
    let width = image[0].len() as u32;
    let height = image.len() as u32;

    let mut img_buffer: RgbImage = ImageBuffer::new(width, height);

    // Fill the image buffer
    for (y, row) in image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let rgb = pixel.to_rgb();
            img_buffer.put_pixel(x as u32, y as u32, Rgb(rgb));
        }
    }

    // Save as BMP
    img_buffer.save(Path::new(path))
}

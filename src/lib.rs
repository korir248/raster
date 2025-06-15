use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};

#[derive(Debug, Copy, Clone)]
pub struct Float3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Float2 {
    pub x: f64,
    pub y: f64,
}

impl Float2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
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

    let a = Float2::new(0.2 * WIDTH as f64, 0.2 * HEIGHT as f64);
    let b = Float2::new(0.7 * WIDTH as f64, 0.4 * HEIGHT as f64);
    let c = Float2::new(0.4 * WIDTH as f64, 0.8 * HEIGHT as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Float2::new(x as f64, y as f64);

            if is_point_in_triangle(p, (a, b, c).into()) {
                image[x][y] = Float3::new(0.0, 0.0, 1.0);
            }
        }
    }

    image
}

pub fn save_as_bmp(image: &[[Float3; 64]; 64], path: &str) -> Result<(), image::ImageError> {
    let width = image[0].len() as u32;
    let height = image.len() as u32;

    let mut img_buffer: RgbImage = ImageBuffer::new(width, height);

    for (y, row) in image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let rgb = pixel.to_rgb();
            img_buffer.put_pixel(x as u32, y as u32, Rgb(rgb));
        }
    }

    img_buffer.save(Path::new(path))
}

fn cross_product(a: Float2, b: Float2, p: Float2) -> f64 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

/// Returns true if the point is inside the triangle.
pub fn is_point_in_triangle(p: Float2, triangle: [Float2; 3]) -> bool {
    let [a, b, c] = triangle;
    let d1 = cross_product(a, b, p);
    let d2 = cross_product(b, c, p);
    let d3 = cross_product(c, a, p);

    (d1 * d2 >= 0.0) && (d2 * d3 >= 0.0)
}

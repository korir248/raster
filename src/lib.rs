use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const TRIANGLE_COUNT: usize = 100;

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

pub type Grid = [[Float3; 256]; 256];

pub fn create_test_image() -> Grid {
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

pub fn save_as_bmp(image: &Grid, path: &str) -> Result<(), image::ImageError> {
    let mut img_buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

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

pub fn render(points: &[Float2], triangle_cols: &[Float3], image: &mut Grid) -> Grid {
    for x in 0..HEIGHT {
        for y in 0..WIDTH {
            for (i, chunk) in points.chunks_exact(3).enumerate() {
                let a = chunk[0];
                let b = chunk[1];
                let c = chunk[2];

                let p = Float2::new(x as f64, y as f64);

                if is_point_in_triangle(p, (a, b, c).into()) {
                    image[x][y] = triangle_cols[i / 3];
                }
            }
        }
    }
    *image
}

pub fn create_test_images() -> Grid {
    let mut rng = rand::rng();
    let mut points = [Float2::new(0.0, 0.0); TRIANGLE_COUNT * 3];
    let mut velocities = [Float2::new(0.0, 0.0); TRIANGLE_COUNT * 3];
    let mut triangle_cols = [Float3::new(0.0, 0.0, 0.0); TRIANGLE_COUNT];

    let mut image: [[Float3; WIDTH]; HEIGHT] = [[Float3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

    let half_size = Float2::new(32.0, 32.0);

    for point in points.iter_mut() {
        *point = Float2::new(
            rng.random_range(-half_size.x..half_size.x),
            rng.random_range(-half_size.y..half_size.y),
        );
    }
    for (i, chunk) in velocities.chunks_exact_mut(3).enumerate() {
        let velocity = Float2::new(4.0, 4.0);

        for v in chunk.iter_mut() {
            *v = velocity;
        }

        triangle_cols[i] = Float3::new(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
        )
    }

    render(&points, &triangle_cols, &mut image)
}

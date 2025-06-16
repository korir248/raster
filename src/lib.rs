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

    pub fn random(rng: &mut impl Rng, min: f64, max: f64) -> Self {
        Self {
            x: rng.random_range(min..max),
            y: rng.random_range(min..max),
        }
    }
}

impl Float3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        Self {
            x: rng.random_range(0.0..1.0),
            y: rng.random_range(0.0..1.0),
            z: rng.random_range(0.0..1.0),
        }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.x.clamp(0.0, 1.0) * 255.0) as u8,
            (self.y.clamp(0.0, 1.0) * 255.0) as u8,
            (self.z.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }
}

pub type Grid = [[Float3; WIDTH]; HEIGHT];

pub fn create_test_image() -> Grid {
    let mut image = [[Float3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];
    let a = Float2::new(0.2 * WIDTH as f64, 0.2 * HEIGHT as f64);
    let b = Float2::new(0.7 * WIDTH as f64, 0.4 * HEIGHT as f64);
    let c = Float2::new(0.4 * WIDTH as f64, 0.8 * HEIGHT as f64);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Float2::new(x as f64, y as f64);
            if is_point_in_triangle(p, [a, b, c]) {
                image[y][x] = Float3::new(0.0, 0.0, 1.0);
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

pub fn render(points: &[Float2], triangle_cols: &[Float3], image: &mut Grid) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = Float2::new(x as f64, y as f64);
            for (i, triangle) in points.chunks_exact(3).enumerate() {
                if is_point_in_triangle(p, [triangle[0], triangle[1], triangle[2]]) {
                    image[y][x] = triangle_cols[i];
                }
            }
        }
    }
}

pub fn create_test_images() -> (Vec<Float2>, Vec<Float2>, Vec<Float3>, Grid) {
    let mut rng = rand::rng();
    let mut points = Vec::with_capacity(TRIANGLE_COUNT * 3);
    let mut velocities = Vec::with_capacity(TRIANGLE_COUNT * 3);
    let mut triangle_cols = Vec::with_capacity(TRIANGLE_COUNT);
    let mut image = [[Float3::new(0.0, 0.0, 0.0); WIDTH]; HEIGHT];

    for _ in 0..TRIANGLE_COUNT {
        let center = Float2::random(&mut rng, 0.0, WIDTH as f64);
        let color = Float3::random(&mut rng);
        let velocity = Float2::random(&mut rng, -2.0, 2.0);

        // Create triangle vertices around the center
        for _ in 0..3 {
            points.push(Float2::new(
                center.x + rng.random_range(-10.0..10.0),
                center.y + rng.random_range(-10.0..10.0),
            ));
            velocities.push(velocity);
        }
        triangle_cols.push(color);
    }

    render(&points, &triangle_cols, &mut image);
    (points, velocities, triangle_cols, image)
}

pub fn update_positions(points: &mut [Float2], velocities: &[Float2]) {
    for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
        point.x += velocity.x;
        point.y += velocity.y;

        if point.x < 0.0 {
            point.x += WIDTH as f64;
        }
        if point.x >= WIDTH as f64 {
            point.x -= WIDTH as f64;
        }
        if point.y < 0.0 {
            point.y += HEIGHT as f64;
        }
        if point.y >= HEIGHT as f64 {
            point.y -= HEIGHT as f64;
        }
    }
}

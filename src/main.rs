use raster::{create_test_images, render, save_as_bmp, update_positions};

fn main() {
    let (mut points, velocities, triangle_cols, mut image) = create_test_images();

    for frame in 0..100 {
        update_positions(&mut points, &velocities);
        render(&points, &triangle_cols, &mut image);

        let path = format!("frame_{:03}.bmp", frame);
        save_as_bmp(&image, &path).unwrap();
    }
}

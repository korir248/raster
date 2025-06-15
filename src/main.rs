use raster::{create_test_image, save_as_bmp};

fn main() {
    let image = create_test_image();
    save_as_bmp(&image, "new.png").unwrap();
}

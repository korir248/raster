use raster::{create_test_image, create_test_images, save_as_bmp};

fn main() {
    let image = create_test_image();
    save_as_bmp(&image, "new.bmp").unwrap();

    let images= create_test_images();
    save_as_bmp(&images, "multiple.bmp").unwrap();
}

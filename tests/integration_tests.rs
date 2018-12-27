extern crate tiny_tiff;

use tiny_tiff::*;

#[test]
fn can_manipulate_image() {
    let tiff = reader_open("./tests/test_data/cell32.tif").unwrap();
    let bits = reader_bits_per_sample(tiff, 0);
    let width = reader_width(tiff);
    let height = reader_height(tiff);
    let size = (width * height) as usize;
    let mut buffer: Vec<f32> = vec![0f32; size];
    reader_sample_data(tiff, &buffer, 0);
    reader_close(tiff);

    for px in &mut buffer {
        *px += 42f32;
    }

    let tiff = writer_open("./tests/test_data/cell32_mod.tif", bits, width, height).unwrap();
    writer_write_image_float(tiff, &buffer);
    writer_close(tiff, "test description");
}

extern crate tiny_tiff;

use tiny_tiff::reader;
use tiny_tiff::writer;

#[test]
fn can_manipulate_image() {
    let tiff = reader::open("./tests/test_data/cell32.tif").unwrap();
    let bits = reader::bits_per_sample(tiff, 0);
    let width = reader::width(tiff);
    let height = reader::height(tiff);
    let size = (width * height) as usize;
    let mut buffer: Vec<f32> = vec![0f32; size];
    reader::sample_data(tiff, &buffer, 0);
    reader::close(tiff);

    for px in &mut buffer {
        *px += 42f32;
    }

    let tiff = writer::open("./tests/test_data/cell32_mod.tif", bits, width, height).unwrap();
    writer::write_image_float(tiff, &buffer);
    writer::close(tiff, "test description");
}

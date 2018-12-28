extern crate tiny_tiff;

use tiny_tiff::reader;
use tiny_tiff::writer;

#[test]
fn can_manipulate_image_8() {
    let tiff = reader::open("./tests/test_data/cell8.tif").unwrap();
    let bits = reader::bits_per_sample(tiff, 0);
    let width = reader::width(tiff);
    let height = reader::height(tiff);
    let size = (width * height) as usize;
    let mut buffer: Vec<u8> = vec![0u8; size];
    reader::sample_data(tiff, &buffer, 0);
    reader::close(tiff);

    for px in &mut buffer {
        *px += 42u8;
    }

    let tiff = writer::open("./tests/test_data/cell8_mod.tif", bits, width, height).unwrap();
    writer::write_image_void(tiff, &buffer);
    writer::close(tiff, "test mod 8bit");
}

#[test]
fn can_manipulate_image_16() {
    let tiff = reader::open("./tests/test_data/cell16.tif").unwrap();
    let bits = reader::bits_per_sample(tiff, 0);
    let width = reader::width(tiff);
    let height = reader::height(tiff);
    let size = (width * height) as usize;
    let mut buffer: Vec<u16> = vec![0u16; size];
    reader::sample_data(tiff, &buffer, 0);
    reader::close(tiff);

    for px in &mut buffer {
        if *px <= 65535 - 42 {
            *px += 42u16;
        }
    }

    let tiff = writer::open("./tests/test_data/cell16_mod.tif", bits, width, height).unwrap();
    writer::write_image_void(tiff, &buffer);
    writer::close(tiff, "test mod 16bit");
}

#[test]
fn can_manipulate_image_32() {
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
    writer::close(tiff, "test mod 32bit");
}

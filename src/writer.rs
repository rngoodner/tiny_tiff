use libc::FILE;
use std::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_float;
use std::os::raw::c_int;
use std::os::raw::c_long;

#[repr(C)]
pub struct TinyTiffFile {
    file: *mut FILE,
    last_ifd_offset_field: u32,
    last_start_pos: c_long,
    last_ifd_data_address: u32,
    last_if_count: u16,
    last_header: *mut u8,
    last_header_size: c_int,
    pos: u32,
    width: u32,
    height: u32,
    bits_per_sample: u16,
    description_offset: u32,
    description_size_offset: u32,
    frames: u64,
    byte_order: u8,
}

#[link(name = "tinytiff")]
extern "C" {
    fn TinyTIFFWriter_open(
        filename: *const c_char,
        bits_per_sample: u16,
        width: u32,
        height: u32,
    ) -> *mut TinyTiffFile;
    fn TinyTIFFWriter_getMaxDescriptionTextSize(tiff: *mut TinyTiffFile) -> c_int;
    fn TinyTIFFWriter_close(tiff: *mut TinyTiffFile, image_description: *const c_char);
    fn TinyTIFFWriter_writeImageVoid(tiff: *mut TinyTiffFile, image_data: *mut c_void);
    fn TinyTIFFWriter_writeImageFloat(tiff: *mut TinyTiffFile, image_data: *mut c_float);
    fn TinyTIFFWriter_writeImageDouble(tiff: *mut TinyTiffFile, image_data: *mut c_double);
}

/// create a new tiff file
pub fn open(
    filename: &str,
    bits_per_sample: u16,
    width: u32,
    height: u32,
) -> Result<*mut TinyTiffFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff = unsafe { TinyTIFFWriter_open(pntr, bits_per_sample, width, height) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename))),
    }
}

/// get max size for image description
pub fn max_description_text_size(tiff: *mut TinyTiffFile) -> u32 {
    let size = unsafe { TinyTIFFWriter_getMaxDescriptionTextSize(tiff) };
    size as u32
}

/// close the tiff and write image description to first frame
pub fn close(tiff: *mut TinyTiffFile, image_description: &str) {
    let image_description = CString::new(image_description).unwrap();
    let image_description = image_description.as_ptr();
    unsafe { TinyTIFFWriter_close(tiff, image_description) };
}

/// writes row-major image data to a tiff file
pub fn write_image_void<T>(tiff: *mut TinyTiffFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_void;
    unsafe { TinyTIFFWriter_writeImageVoid(tiff, pntr) };
}

/// writes row-major image data to a tiff file
pub fn write_image_float<T>(tiff: *mut TinyTiffFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_float;
    unsafe { TinyTIFFWriter_writeImageFloat(tiff, pntr) };
}

/// writes row-major image data to a tiff file
pub fn write_image_double<T>(tiff: *mut TinyTiffFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_double;
    unsafe { TinyTIFFWriter_writeImageDouble(tiff, pntr) };
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn can_open() {
    //let _tiff = open("./tests/test_data/cell2.tif", 8, 100, 100).unwrap();
    //}

    #[test]
    #[should_panic]
    fn open_bad_file_panics() {
        let _tiff = open("./does/not/exist.tif", 8, 100, 100).unwrap();
    }

    //#[test]
    //fn can_max_description_text_size() {
    //let tiff = open("./tests/test_data/cell2.tif", 8, 100, 100).unwrap();
    //let size = max_description_text_size(tiff);
    //assert_ne!(size, 0);
    //}

    #[test]
    fn can_write_image_void8_and_close() {
        let bits: u16 = 8;
        let width: u32 = 100;
        let height: u32 = 100;
        let size = width * height;
        let buffer: Vec<u8> = vec![42u8; size as usize];
        let tiff = open("./tests/test_data/test8.tif", bits, width, height).unwrap();
        write_image_void(tiff, &buffer);
        close(tiff, "test 8bit");
    }

    #[test]
    fn can_write_image_void16_and_close() {
        let bits: u16 = 16;
        let width: u32 = 100;
        let height: u32 = 100;
        let size = width * height;
        let buffer: Vec<u16> = vec![42u16; size as usize];
        let tiff = open("./tests/test_data/test16.tif", bits, width, height).unwrap();
        write_image_void(tiff, &buffer);
        close(tiff, "test 16bit");
    }

    #[test]
    fn can_write_image_float32_and_close() {
        let bits: u16 = 32;
        let width: u32 = 100;
        let height: u32 = 100;
        let size = width * height;
        let buffer: Vec<f32> = vec![42f32; size as usize];
        let tiff = open("./tests/test_data/test32.tif", bits, width, height).unwrap();
        write_image_float(tiff, &buffer);
        close(tiff, "test 32bit");
    }

    #[test]
    fn can_write_image_double64_and_close() {
        let bits: u16 = 64;
        let width: u32 = 100;
        let height: u32 = 100;
        let size = width * height;
        let buffer: Vec<f64> = vec![42f64; size as usize];
        let tiff = open("./tests/test_data/test64.tif", bits, width, height).unwrap();
        write_image_double(tiff, &buffer);
        close(tiff, "test 64bit");
    }
}

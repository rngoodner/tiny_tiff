use libc::FILE;
use std::ffi::c_void;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_float;
use std::os::raw::c_int;
use std::os::raw::c_long;

#[repr(C)]
pub struct TinyTIFFFile {
    file: *mut FILE,
    lastIFDOffsetField: u32,
    lastStartPos: c_long,
    lastIFDDATAAdress: u32,
    lastIFDCount: u16,
    lastHeader: *mut u8,
    lastHeaderSize: c_int,
    pos: u32,
    width: u32,
    height: u32,
    bitspersample: u16,
    descriptionOffset: u32,
    descriptionSizeOffset: u32,
    frames: u64,
    byteorder: u8,
}

#[link(name = "tinytiff")]
extern "C" {
    fn TinyTIFFWriter_open(
        filename: *const c_char,
        bits_per_sample: u16,
        width: u32,
        height: u32,
    ) -> *mut TinyTIFFFile;
    fn TinyTIFFWriter_getMaxDescriptionTextSize(tiff: *mut TinyTIFFFile) -> c_int;
    fn TinyTIFFWriter_close(tiff: *mut TinyTIFFFile, image_description: *const c_char);
    fn TinyTIFFWriter_writeImageVoid(tiff: *mut TinyTIFFFile, image_data: *mut c_void);
    fn TinyTIFFWriter_writeImageFloat(tiff: *mut TinyTIFFFile, image_data: *mut c_float);
    fn TinyTIFFWriter_writeImageDouble(tiff: *mut TinyTIFFFile, image_data: *mut c_double);
}

/// create a new tiff file
pub fn open(
    filename: &str,
    bits_per_sample: usize,
    width: usize,
    height: usize,
) -> Result<*mut TinyTIFFFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff =
        unsafe { TinyTIFFWriter_open(pntr, bits_per_sample as u16, width as u32, height as u32) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename))),
    }
}

/// get max size for image description
pub fn max_description_text_size(tiff: *mut TinyTIFFFile) -> usize {
    let size = unsafe { TinyTIFFWriter_getMaxDescriptionTextSize(tiff) };
    size as usize
}

/// close the tiff and write image description to first frame
pub fn close(tiff: *mut TinyTIFFFile, image_description: &str) {
    let image_description = CString::new(image_description).unwrap();
    let image_description = image_description.as_ptr();
    unsafe { TinyTIFFWriter_close(tiff, image_description) };
}

/// writes row-major image data to a tiff file
pub fn write_image_void<T>(tiff: *mut TinyTIFFFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_void;
    unsafe { TinyTIFFWriter_writeImageVoid(tiff, pntr) };
}

/// writes row-major image data to a tiff file
pub fn write_image_float<T>(tiff: *mut TinyTIFFFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_float;
    unsafe { TinyTIFFWriter_writeImageFloat(tiff, pntr) };
}

/// writes row-major image data to a tiff file
pub fn write_image_double<T>(tiff: *mut TinyTIFFFile, buffer: &Vec<T>) {
    let pntr = buffer.as_ptr() as *mut c_double;
    unsafe { TinyTIFFWriter_writeImageDouble(tiff, pntr) };
}

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
        let bits = 8;
        let width = 100;
        let height = 100;
        let size = width * height;
        let mut buffer: Vec<u8> = vec![42u8; size];
        let tiff = open("./tests/test_data/test8.tif", bits, width, height).unwrap();
        write_image_void(tiff, &buffer);
        close(tiff, "test 8bit");
    }

    #[test]
    fn can_write_image_void16_and_close() {
        let bits = 16;
        let width = 100;
        let height = 100;
        let size = width * height;
        let mut buffer: Vec<u16> = vec![42u16; size];
        let tiff = open("./tests/test_data/test16.tif", bits, width, height).unwrap();
        write_image_void(tiff, &buffer);
        close(tiff, "test 16bit");
    }

    #[test]
    fn can_write_image_float32_and_close() {
        let bits = 32;
        let width = 100;
        let height = 100;
        let size = width * height;
        let mut buffer: Vec<f32> = vec![42f32; size];
        let tiff = open("./tests/test_data/test32.tif", bits, width, height).unwrap();
        write_image_float(tiff, &buffer);
        close(tiff, "test 32bit");
    }

    #[test]
    fn can_write_image_double64_and_close() {
        let bits = 64;
        let width = 100;
        let height = 100;
        let size = width * height;
        let mut buffer: Vec<f64> = vec![42f64; size];
        let tiff = open("./tests/test_data/test64.tif", bits, width, height).unwrap();
        write_image_double(tiff, &buffer);
        close(tiff, "test 64bit");
    }
}

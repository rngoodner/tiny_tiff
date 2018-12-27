extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::FILE;
use std::ffi::CString;
use std::ffi::CStr;
use std::ffi::c_void;
use std::os::raw::c_long;
use std::os::raw::c_float;
use std::os::raw::c_double;

const TIFF_LAST_ERROR_SIZE: usize = 1024;

#[repr(C)]
pub struct TinyTIFFReaderFrame {
    width: u32,
    height: u32,
    compression: u16,
    rowsperstrip: u32,
    stripoffsets: *mut u32,
    stripbytecounts: *mut u32,
    stripcount: u32,
    samplesperpixel: u16,
    bitspersample: *mut u16,
    planarconfiguration: u16,
    sampleformat: u16,
    imagelength: u32,
    description: *mut c_char
}

#[repr(C)]
pub struct TinyTIFFReaderFile {
    file: *mut FILE,
    lastError: [c_char; TIFF_LAST_ERROR_SIZE],
    wasError: c_int,
    systembyteorder: u8,
    filebyteorder: u8,
    firstrecord_offset: u32,
    nextifd_offset: u32,
    filesize: u64,
    currentFrame: TinyTIFFReaderFrame
}

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
    byteorder: u8
}

#[link(name = "tinytiff")]
extern "C" {
    fn TinyTIFFReader_open(filename: *const c_char) -> *mut TinyTIFFReaderFile;
    fn TinyTIFFReader_close(tiff: *mut TinyTIFFReaderFile);
    fn TinyTIFFReader_getBitsPerSample(tiff: *mut TinyTIFFReaderFile, sample: c_int) -> u16;
    fn TinyTIFFReader_getSampleData(tiff: *mut TinyTIFFReaderFile, sample_data: *mut c_void, sample: u16) -> c_int;
    fn TinyTIFFReader_getWidth(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_getHeight(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_countFrames(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_getSampleFormat(tiff: *mut TinyTIFFReaderFile) -> u16;
    fn TinyTIFFReader_getSamplesPerPixel(tiff: *mut TinyTIFFReaderFile) -> u16;
    fn TinyTIFFReader_getImageDescription(tiff: *mut TinyTIFFReaderFile) -> *const c_char;
    fn TinyTIFFReader_hasNext(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_readNext(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_success(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_wasError(tiff: *mut TinyTIFFReaderFile) -> c_int;
    fn TinyTIFFReader_getLastError(tiff: *mut TinyTIFFReaderFile) -> *const c_char;

    fn TinyTIFFWriter_open(filename: *const c_char, bits_per_sample: u16, width: u32, height: u32) -> *mut TinyTIFFFile;
    fn TinyTIFFWriter_getMaxDescriptionTextSize(tiff: *mut TinyTIFFFile) -> c_int;
    fn TinyTIFFWriter_close(tiff: *mut TinyTIFFFile, image_description: *const c_char);
    fn TinyTIFFWriter_writeImageVoid(tiff: *mut TinyTIFFFile, image_data: *mut c_void);
    fn TinyTIFFWriter_writeImageFloat(tiff: *mut TinyTIFFFile, image_data: *mut c_float);
    fn TinyTIFFWriter_writeImageDouble(tiff: *mut TinyTIFFFile, image_data: *mut c_double);
}

pub fn reader_open(filename: &str) -> Result<*mut TinyTIFFReaderFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff = unsafe { TinyTIFFReader_open(pntr) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename)))
    }
}

pub fn reader_close(tiff: *mut TinyTIFFReaderFile) {
    unsafe { TinyTIFFReader_close(tiff) };
}

pub fn reader_bits_per_sample(tiff: *mut TinyTIFFReaderFile, sample: i32) -> u16 {
    let bits = unsafe { TinyTIFFReader_getBitsPerSample(tiff, sample) };
    bits
}

pub fn reader_sample_data<T>(tiff: *mut TinyTIFFReaderFile, buffer: &[T], sample: u16) -> bool {
    let pntr = buffer.as_ptr() as *mut c_void;
    let data = unsafe { TinyTIFFReader_getSampleData(tiff, pntr, sample) };
    data != 0
}

pub fn reader_width(tiff: *mut TinyTIFFReaderFile) -> u32 {
    let width = unsafe { TinyTIFFReader_getWidth(tiff) };
    width as u32
}

pub fn reader_height(tiff: *mut TinyTIFFReaderFile) -> u32 {
    let height = unsafe { TinyTIFFReader_getHeight(tiff) };
    height as u32
}

pub fn reader_count_frames(tiff: *mut TinyTIFFReaderFile) -> i32 {
    let frames = unsafe { TinyTIFFReader_countFrames(tiff) };
    frames
}

pub fn reader_sample_format(tiff: *mut TinyTIFFReaderFile) -> u16 {
    let format = unsafe { TinyTIFFReader_getSampleFormat(tiff) };
    format
}

pub fn reader_samples_per_pixel(tiff: *mut TinyTIFFReaderFile) -> u16 {
    let format = unsafe { TinyTIFFReader_getSamplesPerPixel(tiff) };
    format
}

pub fn reader_image_description(tiff: *mut TinyTIFFReaderFile) -> String {
    let desc = unsafe { TinyTIFFReader_getImageDescription(tiff) };
    let desc = unsafe { CStr::from_ptr(desc) };
    let desc = desc.to_str().unwrap();
    let desc = String::from(desc);
    desc
}

pub fn reader_has_next(tiff: *mut TinyTIFFReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_hasNext(tiff) };
    result != 0
}

pub fn reader_read_next(tiff: *mut TinyTIFFReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_readNext(tiff) };
    result != 0
}

pub fn reader_success(tiff: *mut TinyTIFFReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_success(tiff) };
    result != 0
}

pub fn reader_was_error(tiff: *mut TinyTIFFReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_wasError(tiff) };
    result != 0
}

pub fn reader_last_error(tiff: *mut TinyTIFFReaderFile) -> String {
    let error = unsafe { TinyTIFFReader_getLastError(tiff) };
    let error = unsafe { CStr::from_ptr(error) };
    let error = error.to_str().unwrap();
    let error = String::from(error);
    error
}

pub fn writer_open(filename: &str, bits_per_sample: u16, width: u32, height: u32) -> Result<*mut TinyTIFFFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff = unsafe { TinyTIFFWriter_open(pntr, bits_per_sample, width, height) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename)))
    }
}

pub fn writer_max_description_text_size(tiff: *mut TinyTIFFFile) -> i32 {
    let size = unsafe { TinyTIFFWriter_getMaxDescriptionTextSize(tiff) };
    size
}

pub fn writer_close(tiff: *mut TinyTIFFFile, image_description: &str) {
    let image_description = CString::new(image_description).unwrap();
    let image_description = image_description.as_ptr();
    unsafe { TinyTIFFWriter_close(tiff, image_description) };
}

pub fn writer_write_image_void<T>(tiff: *mut TinyTIFFFile, buffer: &[T]) {
    let pntr = buffer.as_ptr() as *mut c_void;
    unsafe { TinyTIFFWriter_writeImageVoid(tiff, pntr) };
}

pub fn writer_write_image_float<T>(tiff: *mut TinyTIFFFile, buffer: &[T]) {
    let pntr = buffer.as_ptr() as *mut c_float;
    unsafe { TinyTIFFWriter_writeImageFloat(tiff, pntr) };
}

pub fn writer_write_image_double<T>(tiff: *mut TinyTIFFFile, buffer: &[T]) {
    let pntr = buffer.as_ptr() as *mut c_double;
    unsafe { TinyTIFFWriter_writeImageDouble(tiff, pntr) };
}

mod tests {
    use super::*;

    #[test]
    fn can_reader_open() {
        let _tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
    }

    #[test]
    #[should_panic]
    fn reader_open_bad_file_panics() {
        let _tiff = reader_open("./does/not/exist.tif").unwrap();
    }

    #[test]
    fn can_reader_close() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        reader_close(tiff);
    }

    #[test]
    fn can_reader_bits_per_sample() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_bits_per_sample(tiff, 0), 8);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_sample_data() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        let mut buffer = [0u8; 191 * 159];
        let result = reader_sample_data(tiff, &buffer, 0);
        reader_close(tiff);
        assert!(result);
        assert_eq!(buffer[2], 112);
    }

    #[test]
    fn can_reader_width() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_width(tiff), 191);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_height() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_height(tiff), 159);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_count_frames() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_count_frames(tiff), 1);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_sample_format() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_sample_format(tiff), 1);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_samples_per_pixel() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_samples_per_pixel(tiff), 1);
        reader_close(tiff);
    }

    #[test]
    fn can_reader_image_description() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_image_description(tiff), "image description");
        reader_close(tiff);
    }

    #[test]
    fn can_reader_has_next() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert!(!reader_has_next(tiff));
        reader_close(tiff);
    }

    #[test]
    fn can_reader_read_next() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert!(!reader_read_next(tiff));
        reader_close(tiff);
    }

    #[test]
    fn can_reader_success() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert!(reader_success(tiff));
        reader_close(tiff);
    }

    #[test]
    fn can_reader_was_error() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert!(!reader_was_error(tiff));
        reader_close(tiff);
    }

    #[test]
    fn can_reader_last_error() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(reader_last_error(tiff), "");
        reader_close(tiff);
    }

    //#[test]
    //fn can_writer_open() {
        //let _tiff = writer_open("./tests/test_data/cell2.tif", 8, 100, 100).unwrap();
    //}

    #[test]
    #[should_panic]
    fn writer_open_bad_file_panics() {
        let _tiff = writer_open("./does/not/exist.tif", 8, 100, 100).unwrap();
    }

    //#[test]
    //fn can_writer_max_description_text_size() {
        //let tiff = writer_open("./tests/test_data/cell2.tif", 8, 100, 100).unwrap();
        //let size = writer_max_description_text_size(tiff);
        //assert_ne!(size, 0);
    //}

    #[test]
    fn can_writer_write_image_void8_and_close() {
        let tiff = reader_open("./tests/test_data/cell8.tif").unwrap();
        let bits = reader_bits_per_sample(tiff, 0);
        let width = reader_width(tiff);
        let height = reader_height(tiff);
        let mut buffer = [0u8; 191 * 159];
        reader_sample_data(tiff, &buffer, 0);
        reader_close(tiff);

        let tiff = writer_open("./tests/test_data/cell8_2.tif", bits, width, height).unwrap();
        writer_write_image_void(tiff, &buffer);
        writer_close(tiff, "test description");
    }

    #[test]
    fn can_writer_write_image_void16_and_close() {
        let tiff = reader_open("./tests/test_data/cell16.tif").unwrap();
        let bits = reader_bits_per_sample(tiff, 0);
        let width = reader_width(tiff);
        let height = reader_height(tiff);
        let mut buffer = [0u16; 191 * 159];
        reader_sample_data(tiff, &buffer, 0);
        reader_close(tiff);

        let tiff = writer_open("./tests/test_data/cell16_2.tif", bits, width, height).unwrap();
        writer_write_image_void(tiff, &buffer);
        writer_close(tiff, "test description");
    }

    #[test]
    fn can_writer_write_image_float_and_close() {
        let tiff = reader_open("./tests/test_data/cell32.tif").unwrap();
        let bits = reader_bits_per_sample(tiff, 0);
        let width = reader_width(tiff);
        let height = reader_height(tiff);
        let mut buffer = [0f32; 191 * 159];
        reader_sample_data(tiff, &buffer, 0);
        reader_close(tiff);

        let tiff = writer_open("./tests/test_data/cell32_2.tif", bits, width, height).unwrap();
        writer_write_image_float(tiff, &buffer);
        writer_close(tiff, "test description");
    }

    #[test]
    fn can_writer_write_image_double_and_close() {
        let tiff = reader_open("./tests/test_data/cell32.tif").unwrap();
        let bits = reader_bits_per_sample(tiff, 0);
        let width = reader_width(tiff);
        let height = reader_height(tiff);
        let mut buffer = [0f32; 191 * 159];
        reader_sample_data(tiff, &buffer, 0);
        reader_close(tiff);

        let tiff = writer_open("./tests/test_data/cell32_3.tif", bits, width, height).unwrap();
        writer_write_image_double(tiff, &buffer);
        writer_close(tiff, "test description");
    }
}

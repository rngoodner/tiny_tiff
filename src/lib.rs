extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::FILE;
use std::ffi::CString;
use std::ffi::c_void;

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

#[link(name = "tinytiff")]
extern "C" {
    fn TinyTIFFReader_open(filename: *const c_char) -> *mut TinyTIFFReaderFile;
    fn TinyTIFFReader_close(tiff: *mut TinyTIFFReaderFile);
    fn TinyTIFFReader_getBitsPerSample(tiff: *mut TinyTIFFReaderFile, sample: c_int) -> u16;
    fn TinyTIFFReader_getSampleData(tiff: *mut TinyTIFFReaderFile, sample_data: *mut c_void, sample: u16) -> c_int;
}

pub fn reader_open(filename: &str) -> Result<*mut TinyTIFFReaderFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff = unsafe { TinyTIFFReader_open(pntr) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename)))
        //false => Err(String::from("Could not open file!"))
    }
}

pub fn reader_close(tiff: *mut TinyTIFFReaderFile) {
    unsafe { TinyTIFFReader_close(tiff) };
}

pub fn reader_bits_per_sample(tiff: *mut TinyTIFFReaderFile, sample: i32) -> u16 {
    let bits = unsafe { TinyTIFFReader_getBitsPerSample(tiff, sample) };
    bits
}

pub fn reader_sample_data(tiff: *mut TinyTIFFReaderFile, buffer: *mut c_void, sample: u16) -> c_int {
    let data = unsafe { TinyTIFFReader_getSampleData(tiff, buffer, sample) };
    data
}

mod tests {
    use super::*;
    use std::ffi::c_void;

    #[test]
    fn reader_open_ok() {
        let _tiff = reader_open("./tests/test_data/cell.tif").unwrap();
    }

    #[test]
    #[should_panic]
    fn reader_open_bad_file_panics() {
        let _tiff = reader_open("./does/not/exist.tif").unwrap();
    }

    #[test]
    fn reader_close_ok() {
        let tiff = reader_open("./tests/test_data/cell.tif").unwrap();
        reader_close(tiff);
    }

    #[test]
    fn reader_bits_per_sample_ok() {
        let tiff = reader_open("./tests/test_data/cell.tif").unwrap();
        assert_eq!(reader_bits_per_sample(tiff, 0), 8);
        reader_close(tiff);
    }

    #[test]
    fn reader_sample_data_ok() {
        let tiff = reader_open("./tests/test_data/cell.tif").unwrap();
        let data = [0u8; 191 * 159];
        let pntr = data.as_ptr() as *mut c_void;
        reader_sample_data(tiff, pntr, 0);
        reader_close(tiff);
        assert_eq!(data[2], 112);
    }
}

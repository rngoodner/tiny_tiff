use libc::FILE;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_int;

const TIFF_LAST_ERROR_SIZE: usize = 1024;

#[repr(C)]
pub struct TinyTiffReaderFrame {
    width: u32,
    height: u32,
    compression: u16,
    rows_per_strip: u32,
    strip_offsets: *mut u32,
    strip_byte_counts: *mut u32,
    strip_count: u32,
    samples_per_pixel: u16,
    bits_per_sample: *mut u16,
    planar_configuration: u16,
    sample_format: u16,
    image_length: u32,
    description: *mut c_char,
}

#[repr(C)]
pub struct TinyTiffReaderFile {
    file: *mut FILE,
    last_error: [c_char; TIFF_LAST_ERROR_SIZE],
    was_error: c_int,
    system_byte_order: u8,
    file_byte_order: u8,
    first_record_offset: u32,
    next_ifd_offset: u32,
    file_size: u64,
    current_frame: TinyTiffReaderFrame,
}

#[link(name = "tinytiff")]
extern "C" {
    fn TinyTIFFReader_open(filename: *const c_char) -> *mut TinyTiffReaderFile;
    fn TinyTIFFReader_close(tiff: *mut TinyTiffReaderFile);
    fn TinyTIFFReader_getBitsPerSample(tiff: *mut TinyTiffReaderFile, sample: c_int) -> u16;
    fn TinyTIFFReader_getSampleData(
        tiff: *mut TinyTiffReaderFile,
        sample_data: *mut c_void,
        sample: u16,
    ) -> c_int;
    fn TinyTIFFReader_getWidth(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_getHeight(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_countFrames(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_getSampleFormat(tiff: *mut TinyTiffReaderFile) -> u16;
    fn TinyTIFFReader_getSamplesPerPixel(tiff: *mut TinyTiffReaderFile) -> u16;
    fn TinyTIFFReader_getImageDescription(tiff: *mut TinyTiffReaderFile) -> *const c_char;
    fn TinyTIFFReader_hasNext(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_readNext(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_success(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_wasError(tiff: *mut TinyTiffReaderFile) -> c_int;
    fn TinyTIFFReader_getLastError(tiff: *mut TinyTiffReaderFile) -> *const c_char;
}

/// open tiff file for reading
pub fn open(filename: &str) -> Result<*mut TinyTiffReaderFile, String> {
    let cfilename = CString::new(filename).unwrap();
    let pntr = cfilename.as_ptr();
    let tiff = unsafe { TinyTIFFReader_open(pntr) };
    match tiff.is_null() {
        false => Ok(tiff),
        true => Err(format!("Could not open file: {}", String::from(filename))),
    }
}

/// close tiff file
pub fn close(tiff: *mut TinyTiffReaderFile) {
    unsafe { TinyTIFFReader_close(tiff) };
}

/// get bits per sample of current frame
pub fn bits_per_sample(tiff: *mut TinyTiffReaderFile, sample: u16) -> u16 {
    let bits = unsafe { TinyTIFFReader_getBitsPerSample(tiff, sample as i32) };
    bits
}

/// read data from current frame into supplied buffer
pub fn sample_data<T>(tiff: *mut TinyTiffReaderFile, buffer: &Vec<T>, sample: u16) -> bool {
    let pntr = buffer.as_ptr() as *mut c_void;
    let data = unsafe { TinyTIFFReader_getSampleData(tiff, pntr, sample) };
    data != 0
}

/// get width of current frame
pub fn width(tiff: *mut TinyTiffReaderFile) -> u32 {
    let width = unsafe { TinyTIFFReader_getWidth(tiff) };
    width as u32
}

/// get height of current frame
pub fn height(tiff: *mut TinyTiffReaderFile) -> u32 {
    let height = unsafe { TinyTIFFReader_getHeight(tiff) };
    height as u32
}

/// get number of frames
pub fn count_frames(tiff: *mut TinyTiffReaderFile) -> u32 {
    let frames = unsafe { TinyTIFFReader_countFrames(tiff) };
    frames as u32
}

/// get sample format of current frame
pub fn sample_format(tiff: *mut TinyTiffReaderFile) -> u16 {
    let format = unsafe { TinyTIFFReader_getSampleFormat(tiff) };
    format
}

/// get samples per pixel of current from
pub fn samples_per_pixel(tiff: *mut TinyTiffReaderFile) -> u16 {
    let format = unsafe { TinyTIFFReader_getSamplesPerPixel(tiff) };
    format
}

/// get image description of current frame
pub fn image_description(tiff: *mut TinyTiffReaderFile) -> String {
    let desc = unsafe { TinyTIFFReader_getImageDescription(tiff) };
    let desc = unsafe { CStr::from_ptr(desc) };
    let desc = desc.to_str().unwrap();
    let desc = String::from(desc);
    desc
}

/// true if another frame exists
pub fn has_next(tiff: *mut TinyTiffReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_hasNext(tiff) };
    result != 0
}

/// read next frame from a multi-frame tiff
pub fn read_next(tiff: *mut TinyTiffReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_readNext(tiff) };
    result != 0
}

/// true if no error in last function call
pub fn success(tiff: *mut TinyTiffReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_success(tiff) };
    result != 0
}

/// true if error in last function call
pub fn was_error(tiff: *mut TinyTiffReaderFile) -> bool {
    let result = unsafe { TinyTIFFReader_wasError(tiff) };
    result != 0
}

/// get last error messsage
pub fn last_error(tiff: *mut TinyTiffReaderFile) -> String {
    let error = unsafe { TinyTIFFReader_getLastError(tiff) };
    let error = unsafe { CStr::from_ptr(error) };
    let error = error.to_str().unwrap();
    let error = String::from(error);
    error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_open() {
        let _tiff = open("./tests/test_data/cell8.tif").unwrap();
    }

    #[test]
    #[should_panic]
    fn open_bad_file_panics() {
        let _tiff = open("./does/not/exist.tif").unwrap();
    }

    #[test]
    fn can_close() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        close(tiff);
    }

    #[test]
    fn can_bits_per_sample() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(bits_per_sample(tiff, 0), 8);
        close(tiff);
    }

    #[test]
    fn can_sample_data() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        let width = width(tiff);
        let height = height(tiff);
        let size = (width * height) as usize;
        let buffer: Vec<u8> = vec![0u8; size];
        let result = sample_data(tiff, &buffer, 0);
        close(tiff);
        assert!(result);
        assert_eq!(buffer[2], 112 as u8);
    }

    #[test]
    fn can_width() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(width(tiff), 191);
        close(tiff);
    }

    #[test]
    fn can_height() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(height(tiff), 159);
        close(tiff);
    }

    #[test]
    fn can_count_frames() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(count_frames(tiff), 1);
        close(tiff);
    }

    #[test]
    fn can_sample_format() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(sample_format(tiff), 1);
        close(tiff);
    }

    #[test]
    fn can_samples_per_pixel() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(samples_per_pixel(tiff), 1);
        close(tiff);
    }

    #[test]
    fn can_image_description() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(image_description(tiff), "image description");
        close(tiff);
    }

    #[test]
    fn can_has_next() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert!(!has_next(tiff));
        close(tiff);
    }

    #[test]
    fn can_read_next() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert!(!read_next(tiff));
        close(tiff);
    }

    #[test]
    fn can_success() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert!(success(tiff));
        close(tiff);
    }

    #[test]
    fn can_was_error() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert!(!was_error(tiff));
        close(tiff);
    }

    #[test]
    fn can_last_error() {
        let tiff = open("./tests/test_data/cell8.tif").unwrap();
        assert_eq!(last_error(tiff), "");
        close(tiff);
    }
}

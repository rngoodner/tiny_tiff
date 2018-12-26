extern crate tiny_tiff;

use std::ffi::CString;
use tiny_tiff::*;

fn main() {
    let filename = CString::new("/home/ryn1x/Documents/twix32.tif").unwrap();
    let filename = filename.as_ptr();
    let _tiff = unsafe { TinyTIFFReader_open(filename) };
}

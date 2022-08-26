//! # tiny_tiff
//!
//! `tiny_tiff` is a wrapper for the TinyTIFF C++ library. It enables easy reading and writing of
//! uncompressed TIFF images with uint, int, or float data types.
//!
//! DEPENDANCIES
//! ============
//!
//! - `git clone https://github.com/rngoodner/TinyTIFF.git`
//! - `cd TinyTIFF`
//! - `mkdir build`
//! - `cd build`
//! - `cmake ..`
//! - `make -j`
//! - `sudo make install`
//!
//! SYNOPSIS
//! ========
//!
//! ```
//! extern crate tiny_tiff;
//!
//! use tiny_tiff::reader;
//! use tiny_tiff::writer;
//!
//! // read
//! let tiff = reader::open("./tests/test_data/cell32.tif").unwrap();
//! let bits = reader::bits_per_sample(tiff, 0);
//! let width = reader::width(tiff);
//! let height = reader::height(tiff);
//! let size = width * height;
//! let mut buffer: Vec<f32> = vec![0f32; size as usize];
//! reader::sample_data(tiff, &mut buffer, 0);
//! reader::close(tiff);
//!
//! // manipulate
//! for px in &mut buffer {
//!     *px += 42f32;
//! }
//!
//! // write
//! let tiff = writer::open("./tests/test_data/cell32_example.tif", bits, width, height).unwrap();
//! writer::write_image_float(tiff, &buffer);
//! writer::close(tiff, "cell32 + 42!");
//! ```

extern crate libc;

pub mod reader;
pub mod writer;

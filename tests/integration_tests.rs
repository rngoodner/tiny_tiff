extern crate tiny_tiff;

use tiny_tiff::*;

#[test]
fn reader_write_ok() {
    let _tiff = reader_open("./test/test_data/cell.tif");
}

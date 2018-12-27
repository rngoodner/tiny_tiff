# tiny_tiff

`tiny_tiff` is a wrapper for the TinyTIFF C++ library. It enables easy reading and writing of
uncompressed TIFF images with uint, int, and float data types.

DEPENDANCIES
============

## Unix
```
git clone https://github.com/ryn1x/TinyTIFF.git
cd TinyTIFF
mkdir build
cd build
cmake ..
make
sudo make install
```

## Windows
```
git clone https://github.com/ryn1x/TinyTIFF.git
cd TinyTIFF
mkdir build
cd build
cmake ..
cmake -DCMAKE_WINDOWS_EXPORT_ALL_SYMBOLS=TRUE -DBUILD_SHARED_LIBS=TRUE -G "Visual Studio 15 2017 win64" ..
build generated ".sln" file with visual studio
```

SYNOPSIS
========

```
// read
let tiff = reader_open("./tests/test_data/cell32.tif").unwrap();
let bits = reader_bits_per_sample(tiff, 0);
let width = reader_width(tiff);
let height = reader_height(tiff);
let size = (width * height) as usize;
let mut buffer: Vec<f32> = vec![0f32; size];
reader_sample_data(tiff, &buffer, 0);
reader_close(tiff);

// manuipulate
for px in &mut buffer {
    *px += 42f32;
}

// write
let tiff = writer_open("./tests/test_data/cell32_mod.tif", bits, width, height).unwrap();
writer_write_image_float(tiff, &buffer);
writer_close(tiff, "test description");


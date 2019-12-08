# tiny_tiff

`tiny_tiff` is a wrapper for a lightly modified version of the TinyTIFF C++ library. It enables easy reading and writing of uncompressed TIFF images with uint, int, or float data types.

DEPENDANCIES
============

## Unix

Dependancies will be automatically installed if Linux or macOS is detected. Note that you will be prompted for your password when "sudo make install" is attempted. Manual instructions just in case the build script fails:

- `git clone https://github.com/ryn1x/TinyTIFF.git`
- `cd TinyTIFF`
- `mkdir build`
- `cd build`
- `cmake ..`
- `make`
- `sudo make install`

## Windows

- `git clone https://github.com/ryn1x/TinyTIFF.git`
- `cd TinyTIFF`
- `mkdir build`
- `cd build`
- `cmake ..`
- `cmake -DCMAKE_WINDOWS_EXPORT_ALL_SYMBOLS=TRUE -DBUILD_SHARED_LIBS=TRUE -G "Visual Studio 15 2017 win64" ..`
- build generated ".sln" file with visual studio

DOCUMENTATION
============
`cargo doc --open`

COPYRIGHT AND LICENSE
=====================

Copyright 2018 ryn1x

This library is free software; you can redistribute it and/or modify it under the MIT or APACHE-2.0 licenses.

# tiny_tiff

`tiny_tiff` is a wrapper for the TinyTIFF C++ library. It enables easy reading and writing of
uncompressed TIFF images with uint, int, or float data types.

DEPENDANCIES
============

- `git clone https://github.com/rngoodner/TinyTIFF.git`
- `cd TinyTIFF`
- `mkdir build`
- `cd build`
- `cmake ..`
- `make -j`
- `sudo make install`

DOCUMENTATION
============
`cargo doc --open`

COPYRIGHT AND LICENSE
=====================

Copyright 2022 rngoodner

This library is free software; you can redistribute it and/or modify it under the MIT or APACHE-2.0 licenses.

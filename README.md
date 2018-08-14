[![License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

slice_queue
===========
Welcome to my `slice_queue`-library 🎊


What this library is:
---------------------
This library provides a optimized queue for efficient working with (byte-)slices. It allows you to
 - efficiently push an arbitrary amount of elements by either consuming them or by cloning them from a slice (if the
   type supports the `Clone` trait)
 - efficiently popping an arbitrary amount of elements from the front
 - direct access to the underlying buffer by either using `peek*` methods or by using (range-)indices
 - dereferencing the `SliceDeque<T>` like it's a `Vec<T>` (which usually results in a slice)

_Important: To be as efficient as possible it uses some raw pointer access. If this is a no-go for you, please either
use another crate or provide some patches 😇_ 


Build Documentation and Library:
--------------------------------
To build and open the documentation, go into the project's root-directory and run `cargo doc --release --open`

To build this library, change into the projects root-directory and run `cargo build --release`; you can find the build
in `target/release`.
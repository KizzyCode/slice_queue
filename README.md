[![License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

slice_queue
===========
Welcome to my `slice_queue`-library 🎊


What this library is:
---------------------
This library provides a optimized queue for efficient working with (byte-)slices. It allows you to
 - efficiently push an arbitrary amount of elements to the back by either consuming them or by cloning/copying them from
   a slice (if the type supports the `Clone`/`Copy` trait)
 - efficiently pop an arbitrary amount of elements from the front (optionally into a to avoid uneccessary reallocations)
 - access the underlying buffer directly by either using `peek*` methods or (range-)indices
 - dereference the `SliceQueue<T>` by propagating the `deref()`-call to the underlying `Vec<T>` (see
   [Feature `deref`](#feature-deref))


Feature `deref`
---------------
This feature allows you to deref the `SliceQueue<T>` by propagating any `deref()`-call to the underlying `Vec<T>` (which
usually results in a slice). Because in some projects this could be considered as "bad practice", it is possible to
disable this behaviour. _This feature is enabled by default_


Feature `fast_unsafe_code`
--------------------------
To get even more performance, you can use the feature `fast_unsafe_code`. This replaces some safe operations with raw
pointer access and manual memory management, which can improve the performance dramatically; especially if you work with
a lot of `Copy`-types. _This feature is disabled by default._


Build Documentation and Library:
--------------------------------
To build and open the documentation, go into the project's root-directory and run `cargo doc --release --open`

To build this library, change into the projects root-directory and run `cargo build --release`; you can find the build
in `target/release`.
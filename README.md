[![License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# slice_queue
Welcome to my `slice_queue`-library 🎊


## About This Library
This library provides a optimized queue for efficient working with (byte-)slices. It allows you to
 - efficiently push an arbitrary amount of elements to the back by either consuming them or by cloning/copying them from
   a slice (if the type supports the `Clone`/`Copy` trait)
 - communicate and enforce a limit on the amount of elements to store
 - efficiently pop an arbitrary amount of elements from the front (optionally into a slice to avoid uneccessary
   reallocations)
 - access the underlying buffer directly by using either `peek*` methods or (range-)indices
 - dereference the `SliceQueue<T>` by propagating the `deref()`-call to the underlying `Vec<T>` (can be disabled; see
   [Feature-Gates](#feature-gates))


## Feature-Gates
 - `deref`: This feature allows you to deref the `SliceQueue<T>` by propagating any `deref()`-call to the underlying
   `Vec<T>` (which usually results in a slice). Because in some projects this could be considered as "bad practice", it
   is possible to disable this behaviour. __This feature is enabled by default.__
 - `unsafe_fast_code`: Because the main goal of this library is performance, we use raw pointer access and manual memory
   managementin some places. Especially for `Copy`-types like `u8`, this improves the performance dramatically. Since
   this requires unsafe code which may be not acceptible in your case, it is possible to replace the unsafe code with
   safe `Vec`-operations by disabling this feature. __This feature is enabled by default.__


## Build Documentation and Library:
To build and open the documentation, go into the project's root-directory and run `cargo doc --release --open`

To build this library, change into the projects root-directory and run `cargo build --release` (or
`cargo build --release --features ...` to manually specify the features to use); you can find the build in
`target/release`.

If you use this library for the first time or after an update, we recomment you to run `cargo test --release` (or
`cargo test --release --features ...` to manually specify the features to use).
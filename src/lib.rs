//! This library provides `SliceQueue`, a optimized queue for efficient working with (byte-)slices.
//! It allows you to
//!  - efficiently push an arbitrary amount of elements to the back by either consuming them or by
//!    cloning/copying them from a slice (if the type supports the `Clone`/`Copy` trait)
//!  - communicate and enforce a limit on the amount of elements to store
//!  - efficiently pop an arbitrary amount of elements from the front (optionally into a slice to
//!    avoid uneccessary reallocations)
//!  - access the underlying buffer directly by using (range-)indices
//!  - dereference the `SliceQueue<T>` by propagating the `deref()`-call to the underlying `Vec<T>`
//!  - access it using the `io::Read` and `io::Write` traits

mod mem;
mod queue;
mod traits;

pub use queue::{ SliceQueue, AutoShrinkMode };
pub use traits::{ ReadableSliceQueue, WriteableSliceQueue };
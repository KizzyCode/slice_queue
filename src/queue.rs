use super::{ mem, ReadableSliceQueue, WriteableSliceQueue };
use std::{
	cmp::min, usize, io::{ Read, Write, Result as IoResult },
	fmt::{ Debug, Formatter, Result as FmtResult },
	ops::{ Index, IndexMut, Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive }
};


#[derive(Copy, Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum AutoShrinkMode {
	/// Shrinks the `SliceQueue` in 50% steps using `self.shrink_opportunistic`
	///
	/// __This mode is the default value__
	Opportunistic,
	/// Immediately shrinks the `SliceQueue` to the amount of bytes used using `self.shrink_to_fit`
	///
	/// This method is potentially inefficient but can be useful in certain scenarios
	Aggressive,
	/// Disables auto-shrink
	///
	/// If this mode is set, you must take care to use the `self.shrink_opportunistic` or
	/// `self.shrink_to_fit` methods accordingly if necessary.
	Disabled
}
impl Default for AutoShrinkMode {
	fn default() -> Self {
		AutoShrinkMode::Opportunistic
	}
}


#[derive(Default)]
pub struct SliceQueue<T> {
	backing: Vec<T>,
	limit: usize,
	auto_shrink_mode: AutoShrinkMode
}
impl<T> SliceQueue<T> {
	/// Creates a new `SliceQueue`
	///
	/// Returns __the new `SliceQueue`__
	pub fn new() -> Self {
		SliceQueue{ backing: Vec::new(), limit: usize::MAX, auto_shrink_mode: Default::default() }
	}
	/// Creates a new `SliceQueue` with a preallocated capacity `n`
	///
	/// Parameters:
	///  - `n`: The capacity to preallocate
	///
	/// Returns __the new `SliceQueue`__
	pub fn with_capacity(n: usize) -> Self {
		SliceQueue{ backing: Vec::with_capacity(n), limit: usize::MAX, auto_shrink_mode: Default::default() }
	}
	/// Creates a new `SliceQueue` with a predefined `limit` (the default limit is `usize::MAX`)
	///
	/// __Warning: Panics if `limit` is `0`__
	///
	/// Parameters:
	///  - `limit`: The limit to enforce. The limit indicates the maximum amount of elements that
	///    can be stored by `self`.
	///
	/// Returns __the new `SliceQueue`__
	pub fn with_limit(limit: usize) -> Self {
		assert!(limit > 0, "`limit` is `0`");
		SliceQueue{ backing: Vec::new(), limit, auto_shrink_mode: Default::default() }
	}
	
	
	/// Sets the auto-shrink mode
	///
	/// This mode specifies how the `SliceQueue` should behave if elements are consumed
	///
	/// Parameters:
	///  - `auto_shrink`: The auto-shrink mode to use
	pub fn set_auto_shrink_mode(&mut self, mode: AutoShrinkMode) {
		self.auto_shrink_mode = mode
	}
	/// The auto-shrink mode currently used
	///
	/// Returns _`true`_ if auto-shrink is enabled (which is the default state) or _`false`_ if it
	/// was disabled
	pub fn auto_shrink_mode(&self) -> AutoShrinkMode {
		self.auto_shrink_mode
	}
	
	
	/// Sets a new limit (the default limit is `usize::MAX`)
	///
	/// _Info: The limit is only enforced during the `push*`-calls. If the current length exceeds
	/// the new limit, nothing happens until a `push*`-call would exceed the limit._
	///
	/// __Warning: Panics if `limit` is `0`__
	///
	/// Parameters:
	///  - `limit`: The new limit to enforce. The limit indicates the maximum amount of elements
	///    that can be stored by `self`.
	pub fn set_limit(&mut self, limit: usize) {
		assert!(limit > 0, "`limit` is `0`");
		self.limit = limit
	}
	/// The current limit
	///
	/// Returns __the current size-limit of `self`__
	pub fn limit(&self) -> usize {
		self.limit
	}
	
	
	/// Shrinks the allocated capacity if less than it's half is used or the allocated capacity is
	/// greater than `self.limit`
	pub fn shrink_opportunistic(&mut self) {
		// Compute the half capacity
		let half_capacity = if self.backing.capacity() == 0 { 0 }
			else { self.backing.capacity() / 2 };
		
		// Resize the backing if the used space is smaller than the half capacity
		if self.len() > 4 && (self.len() <= half_capacity || self.backing.capacity() > self.limit) { self.backing.shrink_to_fit() }
	}
	/// Shrinks the allocated capacity as much as possible
	pub fn shrink_to_fit(&mut self) {
		self.backing.shrink_to_fit()
	}
	/// Performs the auto-shrink action specified by `self.auto_shrink_mode`
	pub fn auto_shrink(&mut self) {
		match self.auto_shrink_mode {
			AutoShrinkMode::Opportunistic => self.shrink_opportunistic(),
			AutoShrinkMode::Aggressive => self.shrink_to_fit(),
			AutoShrinkMode::Disabled => ()
		}
	}
}


impl<T> ReadableSliceQueue<T> for SliceQueue<T> {
	/// The amount of elements stored
	///
	/// Returns __the amount of elements stored in `self`__
	fn len(&self) -> usize {
		self.backing.len()
	}
	/// Checks if there are __no__ elements stored
	///
	/// Returns either __`true`__ if `self` is empty or __`false`__ otherwise
	fn is_empty(&self) -> bool {
		self.backing.is_empty()
	}
	
	/// Take a look at the first element __without__ consuming it
	///
	/// Returns either _`Some(element_ref)`_ if we have a first element or _`None`_ otherwise
	fn peek(&self) -> Option<&T> {
		self.backing.first()
	}
	/// Take a look at the first `n` elements __without__ consuming them
	///
	/// Parameters:
	///  - `n`: The amount of elements to peek at
	///
	/// Returns either __`Ok(element_refs)`__ if there were `n` elements avaliable to peek at or
	/// __`Err(element_refs)`__ if less elements were available
	fn peek_n(&self, n: usize) -> Result<&[T], &[T]> {
		if n <= self.len() { Ok(&self.backing[..n]) }
			else { Err(&self.backing) }
	}
	
	/// Consumes the first element and returns it
	///
	/// Returns either __`Ok(element)`__ if there was an element to consume or __`Err(())`__
	/// otherwise
	fn pop(&mut self) -> Result<T, ()> {
		match self.is_empty() {
			true => Err(()),
			false => {
				let element = self.backing.remove(0);
				self.auto_shrink();
				Ok(element)
			}
		}
	}
	/// Consumes the first `n` elements and returns them
	///
	/// Parameters:
	///  - `n`: The amount of elements to consume
	///
	/// Returns either __`Ok(elements)`__ if there were `n` elements avaliable to consume or
	/// __`Err(elements)`__ if less elements were available
	fn pop_n(&mut self, n: usize) -> Result<Vec<T>, Vec<T>> {
		// Move elements into `elements`
		let to_consume = min(self.len(), n);
		let elements = mem::drain_n(&mut self.backing, to_consume);
		
		// Shrink and return result
		self.auto_shrink();
		if to_consume == n { Ok(elements) }
			else { Err(elements) }
	}
	/// Consumes the first `dst.len()` and moves them into `dst`
	///
	/// Parameters:
	///  - `dst`: The target to move the elements into
	///
	/// Returns either __`Ok(())`__ if `dst` was filled completely or __`Err(element_count)`__ if
	/// only `element_count` elements were moved
	fn pop_into(&mut self, dst: &mut[T]) -> Result<(), usize> {
		// Move elements
		let to_move = min(self.len(), dst.len());
		mem::drain_into(&mut self.backing, &mut dst[..to_move]);
		
		// Shrink and return result
		self.auto_shrink();
		if to_move == dst.len() { Ok(()) }
			else { Err(to_move) }
	}
	
	/// Discards the first `n` elements
	///
	/// Parameters:
	///  - `n`: The amount of elements to discard
	///
	/// Returns either __`Ok(())`__ if `n` elements were discarded or __`Err(element_count)`__ if
	/// only `element_count` elements were discarded
	fn drop_n(&mut self, n: usize) -> Result<(), usize> {
		// Drop `n` elements and copy the remaining elements to the front
		let to_drop = min(self.len(), n);
		mem::drop_n(&mut self.backing, to_drop);
		
		// Shrink and return result
		self.auto_shrink();
		if to_drop == n { Ok(()) }
			else { Err(to_drop) }
	}
}
impl Read for SliceQueue<u8> {
	/// Pull some bytes from this source into the specified buffer, returning how many bytes were
	/// read.
    ///
    /// It is guaranteed that for [`Ok(n)`] `0 <= n <= buf.len()` is always true. A nonzero `n`
    /// value indicates that the buffer `buf` has been filled in with `n` bytes of data from this
    /// source. If n is 0, then it can indicate one of two scenarios:
    ///
    /// 1. This reader has reached its "end of file" and will likely no longer be able to produce
    ///    bytes. Note that this does not mean that the reader will *always* no longer be able to
    ///    produce bytes.
    /// 2. The buffer specified was 0 bytes in length.
    ///
    /// __This call never fails; the result is only used for trait-compatibility__
	fn read(&mut self, buf: &mut[u8]) -> IoResult<usize> {
		match self.pop_into(buf) {
			Ok(_) => Ok(buf.len()),
			Err(popped) => Ok(popped)
		}
	}
}


impl<T> WriteableSliceQueue<T> for SliceQueue<T> {
	/// The amount of space remaining until `self.limit` is reached
	///
	/// Returns __the amount of space remaining in `self` until `self.limit` is reached__
	fn remaining(&self) -> usize {
		self.limit.checked_sub(self.len()).unwrap_or_default()
	}
	
	/// Reserves an additional amount of memory to append `n` elements without reallocating
	///
	/// Does nothing if `self.reserved` is greater or equal `n`
	///
	/// Parameters:
	///  - `n`: The amount of elements that we should be able to append without reallocating
	///
	/// Returns either _nothing_ if the space for `n` elements could be reserved or _the amount of
	/// elements reserved_ if `n` was greater than `self.remaining`.
	fn reserve_n(&mut self, n: usize) -> Result<(), usize> {
		// Reserve elements
		let to_reserve = min(self.limit.checked_sub(self.backing.capacity()).unwrap_or_default(), n);
		self.backing.reserve_exact(to_reserve);
		
		if to_reserve == n { Ok(()) }
			else { Err(to_reserve) }
	}
	/// The amount of elements that can be appended with out reallocating
	///
	/// Returns __the amount of elements that can be appended with out reallocating__
	fn reserved(&self) -> usize {
		self.backing.capacity() - self.len()
	}
	
	/// Appends `element` at the end
	///
	/// Parameters:
	///  - `element`: The element to append at the end
	///
	/// Returns either __`Ok(())`__ if the element was pushed successfully or __`Err(element)`__ if
	/// `element` was not appended because `self.limit` would have been exceeded
	fn push(&mut self, element: T) -> Result<(), T> {
		if self.remaining() >= 1 { Ok(self.backing.push(element)) }
			else { Err(element) }
	}
	/// Appends `n` at the end
	///
	/// Parameters:
	///  - `n`: The n elements to append at the end
	///
	/// Returns either __`Ok(())`__ if `n` was appended completely or __`Err(remaining_elements)`__
	/// if `n` was only appended partially because `self.limit` would have been exceeded
	fn push_n(&mut self, mut n: Vec<T>) -> Result<(), Vec<T>> {
		if self.remaining() >= n.len() {
			self.backing.append(&mut n);
			Ok(())
		} else {
			let remaining = n.split_off(self.remaining());
			self.backing.append(&mut n);
			Err(remaining)
		}
	}
	/// Clones and appends the elements in `src` at the end
	///
	/// Parameters:
	///  - `src`: A slice containing the elements to clone and append
	///
	/// Returns either __`Ok(())`__ if `src` was appended completely or
	/// __`Err(remaining_element_count)`__ if `src` was only appended partially because `self.limit`
	/// would have been exceeded
	fn push_from(&mut self, src: &[T]) -> Result<(), usize> where T: Clone {
		let to_append = min(self.remaining(), src.len());
		self.backing.extend_from_slice(&src[..to_append]);
		
		if to_append == src.len() { Ok(()) }
			else { Err(to_append) }
	}
	/// Calls `push_fn` to push up to `n` elements in place
	///
	/// __Warning: This function panics if `self.limit` is exceeded__
	///
	/// The function works like this:
	///  1. `n` default elements are inserted at the end
	///  2. `push_fn` is called with a mutable slice referencing the new elements and returns either
	///     the amount of elements pushed or an error
	///  3. If the amount of elements pushed is smaller than `n` or an error occurred, the unused
	///     default elements are removed again
	///
	/// Parameters:
	///  - `n`: The amount of bytes to reserve
	///  - `push_fn`: The pushing callback
	///
	/// Returns either _the amount of elements pushed_ or _the error `push_fn` returned_
	///
	/// Example:
	/// ```
	/// # extern crate slice_queue;
	/// # use slice_queue::*;
	///	let mut slice_queue = SliceQueue::new();
	///
	/// // Successful push
	///	slice_queue.push_in_place(7, |buffer: &mut[usize]| -> Result<usize, ()> {
	/// 	(0..4).for_each(|i| buffer[i] = i);
	/// 	Ok(4)
	/// });
	/// assert_eq!(slice_queue.len(), 4);
	/// (0..4).for_each(|i| assert_eq!(slice_queue[i], i));
	///
	/// // Failed push
	/// slice_queue.push_in_place(7, |buffer: &mut[usize]| -> Result<usize, ()> {
	/// 	(0..4).for_each(|i| buffer[i] = i + 7);
	/// 	Err(())
	/// });
	/// assert_eq!(slice_queue.len(), 4);
	/// (0..4).for_each(|i| assert_eq!(slice_queue[i], i));
	///	```
	fn push_in_place<E>(&mut self, n: usize, mut push_fn: impl FnMut(&mut[T]) -> Result<usize, E>) -> Result<usize, E> where T: Default {
		assert!(self.limit >= self.len() + n, "`self.len() + n` is larger than `self.limit`");
		let old_len = self.len();
		
		// Append `n` default elements
		self.backing.reserve(n);
		(0..n).for_each(|_| self.backing.push(T::default()));
		
		// Call `push_fn` and truncate the length to the amount of elements pushed
		let pushed = push_fn(&mut self.backing[old_len..]);
		self.backing.truncate(old_len + match pushed.as_ref() {
			Ok(pushed) if *pushed > n => panic!("`push_fn` must not claim that it pushed more elements than `n`"),
			Ok(pushed) => *pushed,
			Err(_) => 0
		});
		self.shrink_opportunistic();
		
		pushed
	}
}
impl Write for SliceQueue<u8> {
	/// Write a buffer into this object, returning how many bytes were written.
    ///
    /// This function will attempt to write the entire contents of `buf`, but the entire write may
    /// not succeed. A call to `write` represents *at most one* attempt to write to any wrapped
    /// object.
    ///
    /// It is guaranteed that for [`Ok(n)`] `0 <= n <= buf.len()` is always true. If n is 0, then it
    /// can indicate one of two scenarios:
	///  1. The limit was reached so that the `SliceQueue` cannot accept any more bytes. Note that
	///     this does not mean that the `SliceQueue` will always no longer be able to accept bytes.
	///  2. The buffer specified was 0 bytes in length.
    ///
    /// __This call never fails; the result is only used for trait-compatibility__
	fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
		match self.push_from(buf) {
			Ok(_) => Ok(buf.len()),
			Err(pushed) => Ok(pushed)
		}
	}
	/// __This call does nothing (and thus never fails); it is only provided for
	/// trait-compatibility__
	fn flush(&mut self) -> IoResult<()> {
		Ok(())
	}
}


impl<T: Debug> Debug for SliceQueue<T> {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		f.debug_struct("SliceQueue").field("backing", &self.backing).finish()
	}
}


impl<'a, T> From<&'a[T]> for SliceQueue<T> where T: Clone {
	fn from(slice: &[T]) -> Self {
		SliceQueue{ backing: slice.to_vec(), limit: usize::MAX, auto_shrink_mode: Default::default() }
	}
}
impl<T> From<Vec<T>> for SliceQueue<T> {
	fn from(vec: Vec<T>) -> Self {
		SliceQueue{ backing: vec, limit: usize::MAX, auto_shrink_mode: Default::default() }
	}
}
impl<T> Into<Vec<T>> for SliceQueue<T> {
	fn into(self) -> Vec<T> {
		self.backing
	}
}


impl<T> Clone for SliceQueue<T> where T: Clone {
	fn clone(&self) -> Self {
		SliceQueue{ backing: self.backing.clone(), limit: self.limit, auto_shrink_mode: Default::default() }
	}
}


macro_rules! index_impl {
    ($range_ty:path) => {
    	impl<T> ::std::ops::Index<$range_ty> for SliceQueue<T> {
			type Output = [T];
			fn index(&self, range: $range_ty) -> &[T] {
				&self.backing[range]
			}
		}
		impl<T> ::std::ops::IndexMut<$range_ty> for SliceQueue<T> {
			fn index_mut(&mut self, range: $range_ty) -> &mut[T] {
				&mut self.backing[range]
			}
		}
    };
}
index_impl!(Range<usize>);
index_impl!(RangeFrom<usize>);
index_impl!(RangeTo<usize>);
index_impl!(RangeFull);
index_impl!(RangeInclusive<usize>);
index_impl!(RangeToInclusive<usize>);


impl<T> Index<usize> for SliceQueue<T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.backing[i]
	}
}
impl<T> IndexMut<usize> for SliceQueue<T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		&mut self.backing[i]
	}
}


#[cfg(feature = "deref")]
use std::ops::{ Deref, DerefMut };
#[cfg(feature = "deref")]
impl<T> Deref for SliceQueue<T> {
	type Target = <Vec<T> as Deref>::Target;
	fn deref(&self) -> &Self::Target {
		self.backing.deref()
	}
}
#[cfg(feature = "deref")]
impl<T> DerefMut for SliceQueue<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.backing.deref_mut()
	}
}
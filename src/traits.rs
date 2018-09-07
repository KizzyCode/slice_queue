pub trait ReadableSliceQueue<T> {
	/// The amount of elements stored
	///
	/// Returns __the amount of elements stored in `self`__
	fn len(&self) -> usize;
	/// Checks if there are __no__ elements stored
	///
	/// Returns either __`true`__ if `self` is empty or __`false`__ otherwise
	fn is_empty(&self) -> bool;
	
	/// Consumes the first element and returns it
	///
	/// Returns either __`Ok(element)`__ if there was an element to consume or __`Err(())`__
	/// otherwise
	fn pop(&mut self) -> Result<T, ()>;
	/// Consumes the first `n` elements and returns them
	///
	/// Parameters:
	///  - `n`: The amount of elements to consume
	///
	/// Returns either __`Ok(elements)`__ if there were `n` elements avaliable to consume or
	/// __`Err(elements)`__ if less elements were available
	fn pop_n(&mut self, n: usize) -> Result<Vec<T>, Vec<T>>;
	/// Consumes the first `dst.len()` and moves them into `dst`
	///
	/// Parameters:
	///  - `dst`: The target to move the elements into
	///
	/// Returns either __`Ok(())`__ if `dst` was filled completely or __`Err(element_count)`__ if
	/// only `element_count` elements were moved
	fn pop_into(&mut self, dst: &mut[T]) -> Result<(), usize>;
	
	/// Discards the first `n` elements
	///
	/// Parameters:
	///  - `n`: The amount of elements to discard
	///
	/// Returns either __`Ok(())`__ if `n` elements were discarded or __`Err(element_count)`__ if
	/// only `element_count` elements were discarded
	fn drop_n(&mut self, n: usize) -> Result<(), usize>;
}


pub trait WriteableSliceQueue<T> {
	/// The amount of space remaining until `self.limit` is reached
	///
	/// Returns __the amount of space remaining in `self` until `self.limit` is reached__
	fn remaining(&self) -> usize;
	
	/// Reserves an additional amount of memory to append `n` elements without reallocating
	///
	/// Does nothing if `self.reserved` is greater or equal `n`
	///
	/// Parameters:
	///  - `n`: The amount of elements that we should be able to append without reallocating
	///
	/// Returns either _nothing_ if the space for `n` elements could be reserved or _the amount of
	/// elements reserved_ if `n` was greater than `self.remaining`.
	fn reserve_n(&mut self, n: usize) -> Result<(), usize>;
	/// The amount of elements that can be appended with out reallocating
	///
	/// Returns __the amount of elements that can be appended with out reallocating__
	fn reserved(&self) -> usize;
	
	/// Appends `element` at the end
	///
	/// Parameters:
	///  - `element`: The element to append at the end
	///
	/// Returns either __`Ok(())`__ if the element was pushed successfully or __`Err(element)`__ if
	/// `element` was not appended because `self.limit` would have been exceeded
	fn push(&mut self, element: T) -> Result<(), T>;
	/// Appends `n` at the end
	///
	/// Parameters:
	///  - `n`: The n elements to append at the end
	///
	/// Returns either __`Ok(())`__ if `n` was appended completely or __`Err(remaining_elements)`__
	/// if `n` was only appended partially because `self.limit` would have been exceeded
	fn push_n(&mut self, n: Vec<T>) -> Result<(), Vec<T>>;
	/// Clones and appends the elements in `src` at the end
	///
	/// Parameters:
	///  - `src`: A slice containing the elements to clone and append
	///
	/// Returns either __`Ok(())`__ if `src` was appended completely or
	/// __`Err(remaining_element_count)`__ if `src` was only appended partially because `self.limit`
	/// would have been exceeded
	fn push_from(&mut self, src: &[T]) -> Result<(), usize> where T: Clone;
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
	fn push_in_place<E>(&mut self, n: usize, push_fn: impl FnMut(&mut[T]) -> Result<usize, E>) -> Result<usize, E> where T: Default;
}
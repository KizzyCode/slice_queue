use std::{
	usize, ptr, fmt::{ Debug, Formatter, Result as FmtResult },
	ops::{
		Index, IndexMut,
		Range, RangeFrom, RangeTo, RangeInclusive, RangeToInclusive, RangeBounds, Bound,
		Deref, DerefMut
	}
};

#[derive(Default)]
pub struct SliceDeque<T> {
	backing: Vec<T>
}
impl<T> SliceDeque<T> {
	/// Creates a new `SliceDeque`
	///
	/// Returns _the new `SliceDeque`_
	pub fn new() -> Self {
		SliceDeque{ backing: Vec::new() }
	}
	/// Creates a new `SliceDeque` with a preallocated capacity `n`
	///
	/// Parameters:
	///  - `n`: The capacity to preallocate
	///
	/// Returns _the new `SliceDeque`_
	pub fn with_capacity(n: usize) -> Self {
		SliceDeque{ backing: Vec::with_capacity(n) }
	}
	
	
	/// The amount of elements stored
	///
	/// Returns _the amount of elements stored in `self`_
	pub fn len(&self) -> usize {
		self.backing.len()
	}
	/// Checks if there are __no__ elements stored
	///
	/// Returns either _`true`_ if `self` is empty or _`false`_ otherwise
	pub fn is_empty(&self) -> bool {
		self.backing.is_empty()
	}
	/// Returns the allocated capacity
	///
	/// Returns _the allocated capacity of `self`_
	pub fn capacity(&self) -> usize {
		self.backing.capacity()
	}
	/// Reserves an additional amount of memory to push `additional_element_count` elements without
	/// reallocating
	///
	/// Parameters:
	///  - `additional_element_count`: The amount of elements that we should be able to append
	///    without reallocating
	pub fn reserve(&mut self, additional_element_count: usize) {
		self.backing.reserve(additional_element_count)
	}
	/// Shrinks the allocated capacity if less than it's half is used. This basically mirrors `Vec`'s
	/// allocation strategy.
	pub fn shrink_opportunistic(&mut self) {
		// Compute the half capacity
		let half_capacity = if self.capacity() == 0 { 0 }
			else { self.capacity() / 2 };
		
		// Resize the backing if the used space is smaller than the half capacity
		if self.len() > 4 && self.len() <= half_capacity { self.backing.shrink_to_fit() }
	}
	/// Shrinks the allocated capacity as much as possible
	pub fn shrink_to_fit(&mut self) {
		self.backing.shrink_to_fit()
	}
	
	
	/// Consumes the first element and returns it
	///
	/// Returns either _`Some(element)`_ if there was an element to consume or _`None`_ otherwise
	pub fn pop(&mut self) -> Option<T> {
		match self.is_empty() {
			true => None,
			false => {
				let element = self.backing.remove(0);
				self.shrink_opportunistic();
				Some(element)
			}
		}
	}
	/// Consumes the first `n` elements and returns them
	///
	/// Parameters:
	///  - `n`: The amount of elements to consume
	///
	/// Returns either _`Some(elements)`_ if there were enough elements to consume or _`None`_
	/// otherwise
	pub fn pop_n(&mut self, n: usize) -> Option<Vec<T>> {
		if self.len() < n { return None }
		
		// Copy elements into a new vector
		let mut elements = Vec::with_capacity(n);
		let remaining = self.len() - n;
		unsafe {
			// Copy stored elements to the new vector
			ptr::copy_nonoverlapping(self.backing.as_ptr(), elements.as_mut_ptr(), n);
			// Move the remaining stored elements to the front
			ptr::copy(self.backing[n..].as_ptr(), self.backing.as_mut_ptr(), remaining);
			// Adjust the lengths
			elements.set_len(n);
			self.backing.set_len(remaining);
		}
		self.shrink_opportunistic();
		
		Some(elements)
	}
	/// Consumes the first `target.len()` and moves them into `target`
	///
	/// __Warning: This function panics if there are not enough elements stored to fill `target`
	/// completely__
	///
	/// Parameters:
	///  - `target`: The target to move the elements into
	pub fn pop_into(&mut self, target: &mut[T]) {
		assert!(self.len() >= target.len(), "`target` is larger than `self`");
		
		// Copy raw data
		let to_copy = target.len();
		let remaining = self.len() - to_copy;
		unsafe {
			// Deallocate the elements in `target`
			Self::drop_elements(target.as_mut_ptr(), to_copy);
			// Copy stored elements to `target`
			ptr::copy_nonoverlapping(self.backing.as_ptr(), target.as_mut_ptr(), to_copy);
			// Move the remaining stored elements to the front
			ptr::copy(self.backing[to_copy..].as_ptr(), self.backing.as_mut_ptr(), remaining);
			// Adjust the length
			self.backing.set_len(remaining);
		}
		self.shrink_opportunistic();
	}
	
	
	/// Discards the first `n` elements
	///
	/// __Warning: This function panics if there are less than `n` elements stored in `self`__
	///
	/// Parameters:
	///  - `n`: The amount of elements to discard
	pub fn discard_n(&mut self, n: usize) {
		assert!(self.len() >= n, "`n` is larger than the amount of elements in `self`");
		
		// Drop `n` elements and copy the remaining elements to the front
		let remaining = self.len() - n;
		unsafe {
			// Deallocate the elements to discard
			Self::drop_elements(self.as_mut_ptr(), n);
			// Move the remaining stored elements to the front
			ptr::copy(self.backing[n..].as_ptr(), self.backing.as_mut_ptr(), remaining);
			// Adjust the length
			self.backing.set_len(remaining);
		}
		self.shrink_opportunistic();
	}
	
	
	/// Returns a reference to the first element
	///
	/// Returns either _`Some(element_reference)`_ if there is an element to reference or _`None`_
	/// otherwise
	pub fn peek(&self) -> Option<&T> {
		self.backing.first()
	}
	/// Returns a mutable reference to the first element
	///
	/// Returns either _`Some(element_reference)`_ if there is an element to reference or _`None`_
	/// otherwise
	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.backing.first_mut()
	}
	/// Returns a reference to the first `n` elements
	///
	/// Parameters:
	///  - `n`: The amount of elements to reference
	///
	/// Returns either _`Some(element_references)`_ if there are enough elements to reference or
	/// _`None`_ otherwise
	pub fn peek_n(&self, n: usize) -> Option<&[T]> {
		if self.len() < n { None }
			else { Some(&self.backing[..n]) }
	}
	/// Returns a mutable reference to the first `n` elements
	///
	/// Parameters:
	///  - `n`: The amount of elements to reference
	///
	/// Returns either _`Some(element_references)`_ if there are enough elements to reference or
	/// _`None`_ otherwise
	pub fn peek_n_mut(&mut self, n: usize) -> Option<&mut[T]> {
		if self.len() < n { None }
			else { Some(&mut self.backing[..n]) }
	}
	
	
	/// Appends `element` at the end
	///
	/// Parameters:
	///  - `element`: The element to append at the end
	pub fn push(&mut self, element: T) {
		self.backing.push(element)
	}
	/// Appends `n` at the end
	///
	/// Parameters:
	///  - `n`: The n elements to append at the end
	pub fn push_n(&mut self, mut n: Vec<T>) {
		self.backing.append(&mut n);
	}
	/// Clones and appends all elements in `source` at the end
	///
	/// Parameters:
	///  - `source`: A slice containing the elements to clone and append
	pub fn push_from(&mut self, source: &[T]) where T: Clone {
		self.backing.extend_from_slice(source)
	}
	
	
	/// A private helper function to translate `RangeBounds` into ranges relative to `self`
	///
	/// __Warning: This function panics if an exclusive range over- or underflows `usize` limits__
	///
	/// Parameters:
	///  - `bounds`: The `RangeBounds` to translate
	///
	/// Returns _the translated range_
	fn range_from_bounds(&self, bounds: impl RangeBounds<usize>) -> Range<usize> {
		let start_included = match bounds.start_bound() {
			Bound::Included(b) => *b,
			Bound::Excluded(_) => unreachable!(),
			Bound::Unbounded => 0
		};
		let end_excluded = match bounds.end_bound() {
			Bound::Included(b) => if *b > usize::MIN { *b - 1 }
					else { panic!("Index usize::MIN - 1 is invalid") },
			Bound::Excluded(b) => *b,
			Bound::Unbounded => self.backing.len()
		};
		start_included..end_excluded
	}
	/// A private helper that drops `length` elements referenced by `ptr`
	///
	/// Parameters:
	///  - `ptr`: The pointer referencing the elements to drop
	///  - `length`: The amount of elements to drop
	unsafe fn drop_elements(mut ptr: *mut T, length: usize) {
		(0..length).for_each(|_| {
			ptr.drop_in_place();
			ptr = ptr.offset(1);
		})
	}
}
impl<T: Debug> Debug for SliceDeque<T> {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		Debug::fmt(&**self, f)
	}
}
impl<T> From<Vec<T>> for SliceDeque<T> {
	fn from(vec: Vec<T>) -> Self {
		SliceDeque{ backing: vec }
	}
}
impl<T> Clone for SliceDeque<T> where T: Clone {
	fn clone(&self) -> Self {
		SliceDeque{ backing: self.backing.clone() }
	}
}

macro_rules! impl_range_index {
    ($b:ty) => {
    	impl<T> Index<$b> for SliceDeque<T> {
    		type Output = [T];
			fn index(&self, bounds: $b) -> &[T] {
				&self.backing[self.range_from_bounds(bounds)]
			}
    	}
    	impl<T> IndexMut<$b> for SliceDeque<T> {
			fn index_mut(&mut self, bounds: $b) -> &mut [T] {
				let range = self.range_from_bounds(bounds);
				&mut self.backing[range]
			}
    	}
    };
}
impl_range_index!(Range<usize>);
impl_range_index!(RangeFrom<usize>);
impl_range_index!(RangeTo<usize>);
impl_range_index!(RangeInclusive<usize>);
impl_range_index!(RangeToInclusive<usize>);

impl<T> Index<usize> for SliceDeque<T> {
	type Output = T;
	fn index(&self, i: usize) -> &T {
		&self.backing[i]
	}
}
impl<T> IndexMut<usize> for SliceDeque<T> {
	fn index_mut(&mut self, i: usize) -> &mut T {
		&mut self.backing[i]
	}
}

impl<T> Deref for SliceDeque<T> {
	type Target = <Vec<T> as Deref>::Target;
	fn deref(&self) -> &Self::Target {
		self.backing.deref()
	}
}
impl<T> DerefMut for SliceDeque<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.backing.deref_mut()
	}
}


#[cfg(test)]
mod tests {
	include!("tests.rs");
}
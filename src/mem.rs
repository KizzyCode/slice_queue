
#[cfg(feature = "unsafe_fast_code")]
pub use self::usafe::{ drop_n, drain_n, drain_into };

#[cfg(not(feature = "unsafe_fast_code"))]
pub use self::safe::{ drop_n, drain_n, drain_into };


#[cfg(feature = "unsafe_fast_code")]
mod usafe {
	use std::{ ptr, mem };
	
	/// Drops/deallocates all elements in `slice`
	///
	/// __Warning: The slice's size are not invalidated, so it's possible to "access" an already
	/// deallocated element without a panic (which results in undefined behaviour).__
	///
	/// Parameters:
	///  - `slice`: The slice containing the elements to deallocate
	unsafe fn drop_in_place<T>(slice: &mut[T]) {
		if mem::needs_drop::<T>() {
			let mut ptr = slice.as_mut_ptr();
			(0..slice.len()).for_each(|_| {
				ptr.drop_in_place();
				ptr = ptr.offset(1);
			})
		}
	}
	
	/// Removes `n` elements from `vec`'s beginning __without deallocating them__
	///
	/// Parameters:
	///  - `vec`: The vector to remove the elements from
	///  - `n`: The amount of elements to remove
	unsafe fn discard_n<T>(vec: &mut Vec<T>, n: usize) {
		assert!(n <= vec.len(), "`n` is greater than `vec.len()`");
		
		let remaining = vec.len() - n;
		ptr::copy(vec[n..].as_ptr(), vec.as_mut_ptr(), remaining);
		vec.set_len(remaining);
	}
	
	pub fn drop_n<T>(vec: &mut Vec<T>, n: usize) {
		assert!(n <= vec.len(), "`n` is greater than `vec.len()`");
		
		// Drop the elements and discard them in `vec`
		unsafe{ drop_in_place(&mut vec[..n]) }
		unsafe{ discard_n(vec, n) }
	}
	
	pub fn drain_n<T>(src: &mut Vec<T>, n: usize) -> Vec<T> {
		assert!(n <= src.len(), "`n` is greater than `src.len()`");
		
		// Create new vector
		let mut dst = Vec::with_capacity(n);
		unsafe{ dst.set_len(n) }
		
		// Copy elements and discard them in `src`
		unsafe{ ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), n) }
		unsafe{ discard_n(src, n) }
		
		dst
	}
	
	pub fn drain_into<T>(src: &mut Vec<T>, dst: &mut[T]) {
		assert!(dst.len() <= src.len());
		
		// Drop all elements in `dst`
		unsafe{ drop_in_place(dst) }
		
		// Copy elements and discard them in `src`
		unsafe{ ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), dst.len()) }
		unsafe{ discard_n(src, dst.len()) }
	}
}


#[cfg(not(feature = "unsafe_fast_code"))]
mod safe {
	pub fn drop_n<T>(src: &mut Vec<T>, n: usize) {
		src.drain(..n);
	}
	
	pub fn drain_n<T>(src: &mut Vec<T>, n: usize) -> Vec<T> {
		src.drain(..n).collect()
	}
	
	pub fn drain_into<T>(src: &mut Vec<T>, dst: &mut[T]) {
		let (mut src, dst) = (src.drain(..dst.len()), dst.iter_mut());
		dst.for_each(|t| *t = src.next().unwrap());
	}
}


#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use super::{ drop_n, drain_n, drain_into };
	
	fn rc_vec(n: usize) -> Vec<Rc<usize>> {
		let mut vec = Vec::new();
		(0..n).for_each(|i| vec.push(Rc::new(i)));
		vec
	}
	
	#[test]
	fn test_drop_n() {
		// Create RC-counted elements and clone them and test that the ref-count equals two
		let base = rc_vec(42);
		let mut cloned = base.clone();
		base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
		
		// Drop 7 elements in `cloned` and test the length and ref-counts
		drop_n(&mut cloned, 7);
		assert_eq!(cloned.len(), base.len() - 7);
		base[..7].iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 1));
		base[7..].iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
	}
	
	#[test]
	fn test_drain_n() {
		// Create RC-counted elements and cloned them and test that the ref-count equals two
		let base = rc_vec(42);
		let mut cloned = base.clone();
		base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
		
		// Drain 7 elements and validate them and the remaining elements and the ref-counts
		let drained = drain_n(&mut cloned, 7);
		assert_eq!(drained.len(), 7);
		assert_eq!(cloned.len(), base.len() - 7);
		
		(0..7).for_each(|i| assert_eq!(*drained[i], i));
		(7..base.len()).for_each(|i| assert_eq!(*cloned[i - 7], i));
		
		base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
	}
	
	#[test]
	fn test_drain_into() {
		// Create RC-counted elements and cloned them and test that the ref-count equals two
		let src_base = rc_vec(42);
		let dst_base = rc_vec(7);
		
		let mut src = src_base.clone();
		let mut dst = dst_base.clone();
		
		src_base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
		dst_base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
		
		// Drain 7 elements into `dst` and validate them and the remaining elements and the ref-counts
		drain_into(&mut src, &mut dst);
		
		assert_eq!(dst.len(), dst_base.len());
		assert_eq!(src.len(), src_base.len() - 7);
		
		(0..dst_base.len()).for_each(|i| assert_eq!(*dst[i], i));
		(dst_base.len()..src_base.len()).for_each(|i| assert_eq!(*src[i - 7], i));
		
		src_base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 2));
		dst_base.iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), 1));
	}
}
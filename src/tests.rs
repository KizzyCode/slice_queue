use super::*;
use std::rc::Rc;

struct RcVec(Vec<Rc<usize>>);
impl RcVec {
	pub fn new(n: usize) -> Self {
		RcVec((0..n).map(|i| Rc::new(i)).collect())
	}
	pub fn validate(&self, range: Range<usize>, expected: usize) {
		self.0[range].iter().for_each(|rc| assert_eq!(Rc::strong_count(rc), expected))
	}
}


#[test]
fn test_shrink_opportunistic() {
	// Create slice
	let mut slice_queue = SliceQueue::from(vec![0u8; 14]);
	
	// Discard 6 elements
	slice_queue.discard_n(6);
	assert_eq!(slice_queue.len(), 8);
	assert_eq!(slice_queue.backing.capacity(), 14);
	
	// Discard one more element
	slice_queue.pop().unwrap();
	assert_eq!(slice_queue.len(), 7);
	assert_eq!(slice_queue.backing.capacity(), 7);
}

#[test]
fn test_pop_n() {
	// Create elements and slice
	let base = RcVec::new(14);
	let mut slice_queue = SliceQueue::from(base.0.clone());
	
	// Validate ref-counts
	base.validate(0..14, 2);
	
	// Pop the first 7 elements and validate the popped and remaining elements
	let popped = slice_queue.pop_n(7).unwrap();
	assert_eq!(slice_queue.len(), 7);
	(0..7).for_each(|i| assert_eq!(*popped[i], i));
	(0..7).for_each(|i| assert_eq!(*slice_queue[i], i + 7));
	
	// Validate ref-counts
	base.validate(0..14, 2);
}

#[test]
fn test_pop_into() {
	// Create buffer and base and slice
	let (buffer_base, base) = (RcVec::new(7), RcVec::new(14));
	let (mut buffer, mut slice_queue) =
		(buffer_base.0.clone(), SliceQueue::from(base.0.clone()));
	
	// Validate ref-counts
	buffer_base.validate(0..7, 2);
	base.validate(0..14, 2);
	
	// Pop the first 7 elements and validate the popped and remaining elements
	slice_queue.pop_into(&mut buffer);
	assert_eq!(slice_queue.len(), 7);
	(0..7).for_each(|i| assert_eq!(*buffer[i], i));
	(0..7).for_each(|i| assert_eq!(*slice_queue[i], i + 7));
	
	// Validate ref-counts
	buffer_base.validate(0..7, 1);
	base.validate(0..14, 2);
}

#[test]
fn test_discard_n() {
	// Create elements and slice
	let base = RcVec::new(14);
	let mut slice_queue = SliceQueue::from(base.0.clone());
	
	// Validate ref-counts
	base.validate(0..14, 2);
	
	// Discard the first 7 elements and validate the remaining elements
	slice_queue.discard_n(7);
	assert_eq!(slice_queue.len(), 7);
	(0..7).for_each(|i| assert_eq!(*slice_queue[i], i + 7));
	
	// Validate ref-counts
	base.validate(0..7, 1);
	base.validate(7..14, 2);
}

#[test]
fn test_range_from_bounds() {
	// Create slice
	let slice_queue = SliceQueue::from(vec![0u8; 14]);
	// Validate the translated ranges
	assert_eq!(slice_queue.range_from_bounds(0..14), 0..14);
	assert_eq!(slice_queue.range_from_bounds(..14), 0..14);
	assert_eq!(slice_queue.range_from_bounds(0..), 0..14);
	assert_eq!(slice_queue.range_from_bounds(..), 0..14);
}
#[test] #[should_panic]
fn test_range_from_bounds_end_underflow() {
	// Create slice
	let slice_queue = SliceQueue::from(vec![0u8; 14]);
	// Expect panic
	slice_queue.range_from_bounds(0..=0);
}

#[test] #[cfg(feature = "fast_unsafe_code")]
fn test_replace_n() {
	// Create elements and slice
	let base = RcVec::new(14);
	let mut clone = base.0.clone();
	
	// "Forget" the elements in clone by "emptying" the vector
	unsafe{ clone.set_len(7); }
	// Validate that the elements have not been dropped (the Rcs' ref-counts still are 2)
	base.validate(0..14, 2);
	
	// Replace the first 7 elements
	unsafe{ SliceQueue::replace_n(clone[7..].as_ptr(), clone.as_mut_ptr(), 7) }
	// Validate that the elements have been dropped
	base.validate(0..7, 1);
	base.validate(7..14, 2);
}
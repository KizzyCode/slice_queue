extern crate slice_queue;
use { slice_queue::*, std::{ rc::Rc, ops::Range } };


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
fn test_limit() {
	// Create a slice-queue with a predefined limit and verify it
	let mut slice_queue = SliceQueue::with_limit(9);
	assert_eq!(slice_queue.len(), 0);
	assert_eq!(slice_queue.limit(), 9);
	assert_eq!(slice_queue.remaining(), 9);
	
	// Push some data and verify the remaining free space
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(slice_queue.remaining(), 0);
	
	// Adjust the limit and verify it and the remaining free space
	slice_queue.set_limit(4);
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(slice_queue.limit(), 4);
	assert_eq!(slice_queue.remaining(), 0);
	
	// Drop the data and verify the free space
	slice_queue.drop_n(8).unwrap();
	assert_eq!(slice_queue.len(), 1);
	assert_eq!(slice_queue.remaining(), 3);
}


#[test]
fn test_reserve() {
	// Create a slice-queue with a predefined capacity and verify it
	let mut slice_queue = SliceQueue::with_capacity(42);
	assert_eq!(slice_queue.reserved(), 42);
	
	// Push some data and verify the remaining free space
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(slice_queue.reserved(), 33);
	
	// Reserve capacity for 9 elements and verify that nothing happened (because we already have anough space)
	slice_queue.reserve_n(9).unwrap();
	assert_eq!(slice_queue.reserved(), 33);
	
	// Reserve capacity for 42 elements and verify that we have enough space for 42 elements
	slice_queue.reserve_n(42).unwrap();
	assert_eq!(slice_queue.reserved(), 42);
	
	// Drop the data and verify that we have space for 9 more elements
	slice_queue.drop_n(9).unwrap();
	assert!(slice_queue.is_empty());
	assert_eq!(slice_queue.reserved(), 51);
}


#[test]
fn test_shrink_opportunistic() {
	let mut slice_queue = SliceQueue::from(vec![0u8; 14]);
	assert_eq!(slice_queue.auto_shrink_mode(), AutoShrinkMode::Opportunistic);
	
	// Discard 6 elements
	slice_queue.drop_n(6).unwrap();
	assert_eq!(slice_queue.len(), 8);
	assert_eq!(slice_queue.reserved(), 6);
	
	// Discard one more element
	slice_queue.pop().unwrap();
	assert_eq!(slice_queue.len(), 7);
	assert_eq!(slice_queue.reserved(), 0);
}
#[test]
fn test_shrink_to_fit() {
	let mut slice_queue = SliceQueue::from(vec![0u8; 14]);
	slice_queue.set_auto_shrink_mode(AutoShrinkMode::Aggressive);
	
	// Discard 6 elements
	slice_queue.drop_n(6).unwrap();
	assert_eq!(slice_queue.len(), 8);
	assert_eq!(slice_queue.reserved(), 0);
	
	// Discard one more element
	slice_queue.pop().unwrap();
	assert_eq!(slice_queue.len(), 7);
	assert_eq!(slice_queue.reserved(), 0);
}


#[test]
fn test_pop() {
	let mut slice_queue = SliceQueue::from(vec![7; 14]);
	assert_eq!(slice_queue.len(), 14);
	
	// Pop the first 7 elements and validate the popped elements and remaining length
	(0..7).for_each(|_| assert_eq!(slice_queue.pop().unwrap(), 7));
	assert_eq!(slice_queue.len(), 7);
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
	slice_queue.pop_into(&mut buffer).unwrap();
	assert_eq!(slice_queue.len(), 7);
	(0..7).for_each(|i| assert_eq!(*buffer[i], i));
	(0..7).for_each(|i| assert_eq!(*slice_queue[i], i + 7));
	
	// Validate ref-counts
	buffer_base.validate(0..7, 1);
	base.validate(0..14, 2);
}
#[test]
fn test_drop_n() {
	// Create elements and slice
	let base = RcVec::new(14);
	let mut slice_queue = SliceQueue::from(base.0.clone());
	
	// Validate ref-counts
	base.validate(0..14, 2);
	
	// Discard the first 7 elements and validate the remaining elements
	slice_queue.drop_n(7).unwrap();
	assert_eq!(slice_queue.len(), 7);
	(0..7).for_each(|i| assert_eq!(*slice_queue[i], i + 7));
	
	// Validate ref-counts
	base.validate(0..7, 1);
	base.validate(7..14, 2);
}


#[test]
fn test_push() {
	let mut slice_queue = SliceQueue::new();
	assert!(slice_queue.is_empty());
	
	(0..7).for_each(|i| slice_queue.push(i).unwrap());
	assert_eq!(slice_queue.len(), 7);
	
	(0..7).for_each(|i| assert_eq!(slice_queue[i], i));
}
#[test]
fn test_push_n() {
	let mut slice_queue = SliceQueue::new();
	assert!(slice_queue.is_empty());
	
	// Push data and verify it
	slice_queue.push_n(b"Testolope".to_vec()).unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// Empty push
	slice_queue.push_n(Vec::new()).unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// And a last push
	slice_queue.push_n(b"!!".to_vec()).unwrap();
	assert_eq!(slice_queue.len(), 11);
	assert_eq!(&slice_queue[..], b"Testolope!!");
}
#[test]
fn test_push_from() {
	let mut slice_queue = SliceQueue::new();
	assert!(slice_queue.is_empty());
	
	// Push data and verify it
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// Empty push
	slice_queue.push_from(b"").unwrap();
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// And a last push
	slice_queue.push_from(b"!!").unwrap();
	assert_eq!(slice_queue.len(), 11);
	assert_eq!(&slice_queue[..], b"Testolope!!");
}
#[test]
fn test_push_in_place() {
	let mut slice_queue = SliceQueue::new();
	assert!(slice_queue.is_empty());
	
	// Push data and verify it
	assert_eq!(slice_queue.push_in_place(9, |s: &mut[u8]| -> Result<usize, &'static str> {
		assert_eq!(s.len(), 9);
		s.copy_from_slice(b"Testolope");
		Ok(9)
	}).unwrap(), 9);
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// Empty push
	assert_eq!(slice_queue.push_in_place(9, |s: &mut[u8]| -> Result<usize, &'static str> {
		assert_eq!(s.len(), 9);
		Ok(0)
	}).unwrap(), 0);
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// Error push
	assert_eq!(slice_queue.push_in_place(9, |s: &mut[u8]| -> Result<usize, &'static str> {
		assert_eq!(s.len(), 9);
		s.copy_from_slice(b"Testolope");
		Err("Some test error")
	}).unwrap_err(), "Some test error");
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(&slice_queue[..], b"Testolope");
	
	// Another non-empty push
	assert_eq!(slice_queue.push_in_place(42, |s: &mut[u8]| -> Result<usize, &'static str> {
		assert_eq!(s.len(), 42);
		s[..2].copy_from_slice(b"!!");
		Ok(2)
	}).unwrap(), 2);
	assert_eq!(slice_queue.len(), 11);
	assert_eq!(&slice_queue[..], b"Testolope!!");
}


#[test]
fn test_index() {
	let slice_queue = SliceQueue::from(vec![0, 1, 2, 3, 4, 5, 6, 7]);
	(0..=7).for_each(|i| assert_eq!(slice_queue[i], i));
}
#[test]
fn test_index_mut() {
	let mut slice_queue = SliceQueue::from(vec![0; 7]);
	(0..7).for_each(|i| slice_queue[i] = i);
	(0..7).for_each(|i| assert_eq!(slice_queue[i], i));
}
#[test]
fn test_index_slice() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	
	// Test possible ranges
	assert_eq!(&slice_queue[..], b"Testolope");
	assert_eq!(&slice_queue[..9], b"Testolope");
	assert_eq!(&slice_queue[0..], b"Testolope");
	assert_eq!(&slice_queue[0..9], b"Testolope");
	assert_eq!(&slice_queue[0..=8], b"Testolope");
	assert_eq!(&slice_queue[..=8], b"Testolope");
	
	// Test possible partial ranges
	assert_eq!(&slice_queue[..7], b"Testolo");
	assert_eq!(&slice_queue[4..], b"olope");
	assert_eq!(&slice_queue[4..7], b"olo");
	assert_eq!(&slice_queue[4..=6], b"olo");
	assert_eq!(&slice_queue[..=6], b"Testolo");
}
#[test]
fn test_index_slice_mut() {
	let mut slice_queue = SliceQueue::from(b"*********".as_ref());
	macro_rules! copy_test_reset {
		($slice:expr, $value:expr) => ({
			$slice.copy_from_slice($value);
			assert_eq!($slice, $value);
			
			slice_queue.drop_n(9).unwrap();
			slice_queue.push_from(b"*********").unwrap();
			assert_eq!(&slice_queue[..], b"*********");
		});
	}
	
	// Test possible ranges
	copy_test_reset!(&mut slice_queue[..], b"Testolope");
	copy_test_reset!(&mut slice_queue[..9], b"Testolope");
	copy_test_reset!(&mut slice_queue[0..], b"Testolope");
	copy_test_reset!(&mut slice_queue[0..9], b"Testolope");
	copy_test_reset!(&mut slice_queue[0..=8], b"Testolope");
	copy_test_reset!(&mut slice_queue[..=8], b"Testolope");
	
	// Test possible partial ranges
	copy_test_reset!(&mut slice_queue[..7], b"Testolo");
	copy_test_reset!(&mut slice_queue[4..], b"olope");
	copy_test_reset!(&mut slice_queue[4..7], b"olo");
	copy_test_reset!(&mut slice_queue[4..=6], b"olo");
	copy_test_reset!(&mut slice_queue[..=6], b"Testolo");
}

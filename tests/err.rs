extern crate slice_queue;
use slice_queue::*;


#[test]
fn test_limit() {
	// Create a slice-queue with a predefined limit and verify it
	let mut slice_queue = SliceQueue::with_limit(9);
	assert_eq!(slice_queue.limit(), 9);
	assert_eq!(slice_queue.remaining(), 9);
	
	// Overpush some data and verify the remaining free space
	assert_eq!(slice_queue.push_from(b"Testolope!!").unwrap_err(), 9);
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(slice_queue.remaining(), 0);
	
	// Adjust the limit and verify it
	slice_queue.set_limit(4);
	assert_eq!(slice_queue.limit(), 4);
	
	// Overpush again
	assert_eq!(slice_queue.push_from(b"!!").unwrap_err(), 0);
	assert_eq!(slice_queue.len(), 9);
	assert_eq!(slice_queue.remaining(), 0);
	
	// Drop the data and verify the free space
	slice_queue.drop_n(8).unwrap();
	assert_eq!(slice_queue.len(), 1);
	assert_eq!(slice_queue.remaining(), 3);
	
	// Overpush and validate
	assert_eq!(slice_queue.push_from(b"!!!XXX").unwrap_err(), 3);
	assert_eq!(slice_queue.len(), 4);
	assert_eq!(slice_queue.remaining(), 0);
	assert_eq!(&slice_queue[..], b"e!!!");
}
#[test] #[should_panic(expected = "`limit` is `0`")]
fn test_zero_limit_init() {
	SliceQueue::<u8>::with_limit(0);
}
#[test] #[should_panic(expected = "`limit` is `0`")]
fn test_zero_limit_set() {
	let mut slice_queue = SliceQueue::<u8>::new();
	slice_queue.set_limit(0);
}


#[test]
fn test_reserve() {
	// Create a slice-queue with a predefined limit and verify it
	let mut slice_queue = SliceQueue::<u8>::with_limit(9);
	assert_eq!(slice_queue.limit(), 9);
	assert_eq!(slice_queue.remaining(), 9);
	
	// Reserve 42 slots
	assert_eq!(slice_queue.reserve_n(42).unwrap_err(), 9);
	assert_eq!(slice_queue.reserved(), 9);
}


#[test]
fn test_peek() {
	let slice_queue = SliceQueue::<u8>::new();
	assert!(slice_queue.peek().is_none())
}
#[test]
fn test_peek_n() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(slice_queue.peek_n(11).unwrap_err(), b"Testolope");
}


#[test]
fn test_pop() {
	let mut slice_queue = SliceQueue::new();
	assert_eq!(slice_queue.pop().unwrap_err(), ());
	
	// Push element and consume two
	slice_queue.push(7).unwrap();
	assert_eq!(slice_queue.pop().unwrap(), 7);
	assert_eq!(slice_queue.pop().unwrap_err(), ());
}
#[test]
fn test_pop_n() {
	let mut slice_queue = SliceQueue::new();
	assert!(slice_queue.pop_n(1).unwrap_err().is_empty());
	
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.pop_n(11).unwrap_err(), b"Testolope");
}
#[test]
fn test_pop_into() {
	let (mut slice_queue, mut target) = (SliceQueue::new(), [0u8; 11]);
	assert_eq!(slice_queue.pop_into(&mut target).unwrap_err(), 0);
	assert_eq!(target, [0u8; 11]);
	
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.pop_into(&mut target).unwrap_err(), 9);
	assert_eq!(&target, b"Testolope\x00\x00");
}
#[test]
fn test_drop_n() {
	let mut slice_queue = SliceQueue::new();
	assert_eq!(slice_queue.drop_n(1).unwrap_err(), 0);
	
	slice_queue.push_from(b"Testolope").unwrap();
	assert_eq!(slice_queue.drop_n(11).unwrap_err(), 9);
	assert_eq!(&slice_queue[..], &[]);
}


#[test]
fn test_push() {
	let mut slice_queue = SliceQueue::with_limit(1);
	assert_eq!(slice_queue.push(7).unwrap(), ());
	assert_eq!(slice_queue.push(4).unwrap_err(), 4);
	assert_eq!(&slice_queue[..], [7]);
}
#[test]
fn test_push_n() {
	let mut slice_queue = SliceQueue::with_limit(7);
	assert_eq!(slice_queue.push_n(b"Test".to_vec()).unwrap(), ());
	assert_eq!(slice_queue.push_n(b"olope".to_vec()).unwrap_err(), b"pe");
	assert_eq!(&slice_queue[..], b"Testolo");
}
#[test]
fn test_push_from() {
	let mut slice_queue = SliceQueue::with_limit(7);
	assert_eq!(slice_queue.push_from(b"Test").unwrap(), ());
	assert_eq!(slice_queue.push_from(b"olope").unwrap_err(), 3);
	assert_eq!(&slice_queue[..], b"Testolo");
}
#[test] #[should_panic(expected = "`self.len() + n` is larger than `self.limit`")]
fn test_push_in_place_overpush() {
	let mut slice_queue = SliceQueue::with_limit(7);
	slice_queue.push_in_place(9, |s: &mut[u8]| -> Result<usize, &'static str> {
		s.copy_from_slice(b"Testolope");
		Ok(9)
	}).unwrap();
}
#[test] #[should_panic(expected = "`push_fn` must not claim that it pushed more elements than `n`")]
fn test_push_in_place_invalid_retval() {
	let mut slice_queue = SliceQueue::with_limit(7);
	slice_queue.push_in_place(4, |s: &mut[u8]| -> Result<usize, &'static str> {
		s.copy_from_slice(b"Test");
		Ok(9)
	}).unwrap();
}


#[test] #[should_panic(expected = "index out of bounds: the len is 8 but the index is 8")]
fn test_index() {
	let slice_queue = SliceQueue::from(vec![0, 1, 2, 3, 4, 5, 6, 7]);
	(0..9).for_each(|i| assert_eq!(slice_queue[i], i));
}
#[test] #[should_panic(expected = "index out of bounds: the len is 7 but the index is 7")]
fn test_index_mut() {
	let mut slice_queue = SliceQueue::from(vec![0; 7]);
	(0..9).for_each(|i| slice_queue[i] = i);
}


#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_range_begin() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[9..10], b"!");
}
#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_range_end() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[0..10], b"Testolope!");
}
#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_range_incl_begin() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[9..=9], b"!");
}
#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_range_incl_end() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[0..=9], b"Testolope!");
}
#[test] #[should_panic(expected = "slice index starts at 10 but ends at 9")]
fn test_index_slice_range_from() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[10..], b"!");
}
#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_range_to() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[..10], b"Testolope!");
}
#[test] #[should_panic(expected = "index 10 out of range for slice of length 9")]
fn test_index_slice_to_incl() {
	let slice_queue = SliceQueue::from(b"Testolope".as_ref());
	assert_eq!(&slice_queue[..=9], b"Testolope!");
}
pub fn merge<T: Ord>(arr: &mut [T]) {}

fn _merge<T: Ord>(arr: &mut [T], left: isize, mid: isize, right: isize) {}

fn sort<T: Ord>(arr: &mut [T], start: isize, end: isize) {}

#[cfg(test)]
mod test {
	use super::*;
	use rand;
	use rand::seq::SliceRandom;

	#[test]
	fn test_quick() {
		let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
		let expect = arr.clone();

		arr.shuffle(&mut rand::thread_rng());
		merge(&mut arr);

		assert_eq!(arr, expect);
	}
}
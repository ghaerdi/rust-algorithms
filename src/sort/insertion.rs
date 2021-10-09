pub fn insertion<T: Ord>(arr: &mut [T]) {
	let len = arr.len();
	for i in 1..len {
		let mut j = i;
		while j > 0 && arr[j] < arr[j - 1] {
			arr.swap(j, j - 1);
			j -= 1;
		}
	}
}

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
		insertion(&mut arr);

		assert_eq!(arr, expect);
	}
}

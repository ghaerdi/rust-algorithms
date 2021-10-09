pub fn selection<T: Ord>(arr: &mut [T]) {
	let len = arr.len();
	for i in 0..len {
		let mut min = i;
		for j in i..len {
			if arr[j] < arr[min] {
				min = j;
			}
		}
		if min != i {
			arr.swap(i, min);
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
		selection(&mut arr);

		assert_eq!(arr, expect);
	}
}
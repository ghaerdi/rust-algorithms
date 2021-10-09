pub fn quick<T: Ord>(arr: &mut [T]) {
	let len = arr.len();
	quick_sort(arr, 0, (len - 1) as isize);
}

fn quick_sort<T: Ord>(arr: &mut [T], min: isize, max: isize) {
	if min < max {
		let pivot = partition(arr, min as usize, max as usize);
		println!("{}", pivot);
		quick_sort(arr, min, pivot - 1);
		quick_sort(arr, pivot + 1, max);
	}
}

fn partition<T: Ord>(arr: &mut [T], min: usize, max: usize) -> isize {
	let pivot = max;
	let mut i = min;
	for j in min..max {
		if arr[j] < arr[pivot] {
			if arr[i] != arr[j] {
				arr.swap(i, j);
			}
			i += 1;
		}
	}
	arr.swap(i, max);
	return i as isize;
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
		quick(&mut arr);

		assert_eq!(arr, expect);
	}
}

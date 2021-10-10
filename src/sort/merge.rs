pub fn merge<T: Clone + Ord>(arr: &mut [T]) {
  let len = arr.len();
  merge_sort(arr, 0, len - 1);
}

fn merge_sort<T: Clone + Ord>(arr: &mut [T], start: usize, end: usize) {
  if start >= end {
    return;
  }

  let mid = (start + end) / 2;
  merge_sort(arr, start, mid);
  merge_sort(arr, mid + 1, end);
  sort(arr, start, mid, end);
}

fn sort<T: Clone + Ord>(arr: &mut [T], left: usize, mid: usize, right: usize) {
  let n1 = mid - left + 1;
  let n2 = right - mid;

  let left_arr = arr[left..mid + 1].to_vec();
  let right_arr = arr[mid + 1..right + 1].to_vec();

  let mut first_subarray_index = 0;
  let mut second_subarray_index = 0;
  let mut merged_array_index = left;

  while first_subarray_index < n1 && second_subarray_index < n2 {
    if left_arr[first_subarray_index] <= right_arr[second_subarray_index] {
      arr[merged_array_index] = left_arr[first_subarray_index].clone();
      first_subarray_index += 1;
    } else {
      arr[merged_array_index] = right_arr[second_subarray_index].clone();
      second_subarray_index += 1;
    }
    merged_array_index += 1;
  }

  while first_subarray_index < n1 {
    arr[merged_array_index] = left_arr[first_subarray_index].clone();
    first_subarray_index += 1;
    merged_array_index += 1;
  }

  while second_subarray_index < n2 {
    arr[merged_array_index] = right_arr[second_subarray_index].clone();
    second_subarray_index += 1;
    merged_array_index += 1;
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
    merge(&mut arr);

    assert_eq!(arr, expect);
  }
}

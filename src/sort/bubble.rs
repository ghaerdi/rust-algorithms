pub fn bubble<T: Ord>(arr: &mut [T]) {
  let len = arr.len();
  for i in 0..len {
    for j in 0..len - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use rand;
  use rand::seq::SliceRandom;

  #[test]
  fn test_bubble() {
    let mut arr: [i8; 5] = [1, 2, 3, 4, 5];
    let expect = arr.clone();

    arr.shuffle(&mut rand::thread_rng());
    bubble(&mut arr);

    assert_eq!(arr, expect);
  }
}

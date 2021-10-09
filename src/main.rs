#![allow(dead_code)]
mod other;
mod sort;
use rand::seq::SliceRandom;

fn main() {
  let mut arr = (0..5).collect::<Vec<i32>>();
  arr.shuffle(&mut rand::thread_rng());
  sort::merge(&mut arr);

  println!("array after merge sorted: {:?}", arr);
}

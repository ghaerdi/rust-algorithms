#![allow(dead_code)]
mod sort;
mod other;

fn main() {
  let result = other::fibonacci(10);
  println!("{}", result);
}

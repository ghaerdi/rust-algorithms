#![allow(dead_code)]
mod other;
mod sort;
// use std::collections::LinkedList;
mod collections;

#[derive(Debug, Clone)]
struct Person {
  name: String,
  age: u8,
  gender: String,
}

impl Person {
  fn new(name: &str, age: u8, gender: &str) -> Self {
    return Person {
      name: name.to_string(),
      gender: gender.to_string(),
      age,
    };
  }
  fn birthday(&mut self) {
    self.age += 1;
  }
}

fn main() {
  let mut numbers = collections::LinkedList::<u32>::new();
  numbers.push(10);
  numbers.push(11);
  println!("{:#?}", numbers);
}

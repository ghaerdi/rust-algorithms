use std::collections::HashMap;

pub fn fibonacci(number: u32) -> usize {
  fn memo_fib(number: u32, memo: &mut HashMap<u32, usize>) -> usize {
    match number {
      0 => 0,
      1 => 1,
      _ => {
        let value = *memo.get(&number).unwrap_or(&0);
        if value != 0 {
          return value;
        }

        let result = memo_fib(number - 1, memo) + memo_fib(number - 2, memo);
        memo.insert(number, result);

        return result;
      }
    }
  }

  return memo_fib(number, &mut HashMap::new());
}

#[cfg(test)]
mod testts {
  use super::*;

  #[test]
  fn fibonacci_test() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(10), 55);
  }
}

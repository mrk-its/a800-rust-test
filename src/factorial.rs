pub fn factorial(n: u64) -> u64 {
  match n {
    0 => 1,
    _ => n * factorial(n-1),
  }
}


pub fn factorial2(n: u64) -> u64 {
  (1..n+1).fold(1, |acc, v| acc * v)
}


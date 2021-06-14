pub fn factorial(n: u32) -> u32 {
  match n {
    0 => 1,
    _ => n * factorial(n-1),
  }
}


pub fn factorial2(n: u32) -> u32 {
  (1..n+1).fold(1, |acc, v| acc * v)
}


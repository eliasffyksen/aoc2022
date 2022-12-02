use std::fs::read_to_string;

pub fn p2() -> i32 {
  read_to_string("./data/day2.txt").unwrap()
    .trim()
    .split("\n")
    .map(|l| {
      let a: i32 = l.bytes().nth(0).unwrap() as i32 - 65;
      let b: i32 = l.bytes().nth(2).unwrap() as i32 - 88;
      b * 3 + (a + 2 + b) % 3 + 1
    })
    .sum::<i32>()
}

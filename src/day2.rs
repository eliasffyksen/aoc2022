use std::{fs::read_to_string, rc::Rc, cell::RefCell};


pub fn p() -> i32 {
  read_to_string("./data/day2.txt").unwrap()
    .trim()
    .split("\n")
    .map(|l| {
      let a: i32 = l.bytes().nth(0).unwrap() as i32 - 65;
      let b: i32 = l.bytes().nth(2).unwrap() as i32 - 88;
      (b - a + 4) % 3 * 3 + b + 1
    })
    .sum::<i32>()
}

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

#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod utils;

mod day11;
use day11 as day;

fn main() {
    let input_test = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
    ".to_string();

    let input_real = read_to_string(day::DATA_PATH).unwrap();

    println!("PART 1:");
    println!("\tTEST: {}", day::p1(input_test.clone()));
    println!("\tREAL: {}", day::p1(input_real.clone()));

    println!("PART 2:");
    println!("\tTEST: {}", day::p2(input_test.clone()));
    println!("\tREAL: {}", day::p2(input_real.clone()));
}

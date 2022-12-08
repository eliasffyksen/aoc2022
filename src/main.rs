#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod day8_alt;
use day8_alt as day;

fn main() {
    let input_test = "
30373
25512
65332
33549
35390
    ".to_string();

    let input_real = read_to_string(day::DATA_PATH).unwrap();

    println!("PART 1:");
    println!("\tTEST: {:?}", day::p1(input_test.clone()));
    println!("\tREAL: {:?}", day::p1(input_real.clone()));

    println!("PART 2:");
    println!("\tTEST: {:?}", day::p2(input_test.clone()));
    println!("\tREAL: {:?}", day::p2(input_real.clone()));
}

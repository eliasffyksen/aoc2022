#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod day9;
use day9 as day;

fn main() {
    let input_test = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    ".to_string();

    let input_real = read_to_string(day::DATA_PATH).unwrap();

    println!("PART 1:");
    println!("\tTEST: {:?}", day::p1(input_test.clone()));
    println!("\tREAL: {:?}", day::p1(input_real.clone()));

    println!("PART 2:");
    println!("\tTEST: {:?}", day::p2(input_test.clone()));
    println!("\tREAL: {:?}", day::p2(input_real.clone()));
}

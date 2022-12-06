#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod day5;
use day5 as day;

fn main() {
    let input_test = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();

    let input_real = read_to_string(day::DATA_PATH).unwrap();

    println!("PART 1:");
    println!("\tTEST: {:?}", day::p1(input_test.clone()));
    println!("\tREAL: {:?}", day::p1(input_real.clone()));

    println!("PART 2:");
    println!("\tTEST: {:?}", day::p2(input_test.clone()));
    println!("\tREAL: {:?}", day::p2(input_real.clone()));
}

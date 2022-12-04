#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    println!("{:?}", day4::p2(
        read_to_string("./data/day4.txt").unwrap()
    ));
}

#![feature(iter_array_chunks)]

use std::{fs::read_to_string,};

pub mod day5;


fn main() {
    println!("{:?}", day5::p2(
        read_to_string("./data/day5.txt").unwrap()
    ));
}

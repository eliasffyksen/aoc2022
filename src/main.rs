use std::fs::read_to_string;

pub mod day2;
pub mod day3;

fn main() {
    println!("{:?}", day3::p2(
        read_to_string("./data/day3.txt").unwrap()
    ));
}

#![feature(iter_array_chunks)]

use std::fs::read_to_string;

pub mod day7;
use day7 as day;

fn main() {
    let input_test = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();

    let input_real = read_to_string(day::DATA_PATH).unwrap();

    println!("PART 1:");
    println!("\tTEST: {:?}", day::p1(input_test.clone()));
    println!("\tREAL: {:?}", day::p1(input_real.clone()));

    println!("PART 2:");
    println!("\tTEST: {:?}", day::p2(input_test.clone()));
    println!("\tREAL: {:?}", day::p2(input_real.clone()));
}

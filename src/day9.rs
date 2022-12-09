use std::{iter, collections::HashSet, cmp::max};

use regex::Regex;

pub const DATA_PATH: &str = "data/day9.txt";

pub fn parse(input: &String) -> Vec<(i32, i32)> {
  let re_l = Regex::new(r"^(U|D|L|R) (\d+)").unwrap();
  let data = input.trim().lines()
    .map(|line| {
      let cap = re_l.captures(line).unwrap();
      let dir = match cap.get(1).unwrap().as_str().chars().nth(0).unwrap() {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!()
      };
      let count = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
      iter::repeat(dir).take(count)
    }).flatten().collect();

  data
}

pub fn p1(input: String) -> usize {
  let re_l = Regex::new(r"^(U|D|L|R) (\d+)").unwrap();
  let data = parse(&input);

  let mut pos_set = HashSet::new();

  let mut head = (0, 0);
  let mut tail = (0, 0);
  pos_set.insert(tail);

  for (x, y) in data {
    head.0 += x;
    head.1 += y;

    let mh = max(num::abs(head.0 - tail.0), num::abs(head.1 - tail.1));
    if mh > 1 {
      tail.0 += num::clamp(head.0 - tail.0, -1, 1);
      tail.1 += num::clamp(head.1 - tail.1, -1, 1);
    }

    pos_set.insert(tail);
  }

  pos_set.len()
}

pub fn p2(input: String) -> usize {
  let data = parse(&input);

  let mut pos_set = HashSet::new();

  let mut head = (0, 0);
  let mut tail = (0..9).map(|_| (0, 0)).collect::<Vec<_>>();
  pos_set.insert(tail[0]);

  for (x, y) in data {
    head.0 += x;
    head.1 += y;
    let mut target = head;

    for link in tail.iter_mut() {
      let mh = max(num::abs(target.0 - link.0), num::abs(target.1 - link.1));
      if mh > 1 {
        link.0 += num::clamp(target.0 - link.0, -1, 1);
        link.1 += num::clamp(target.1 - link.1, -1, 1);
      }
      target = link.clone();
    }

    pos_set.insert(tail.iter().last().unwrap().clone());
  }

  pos_set.len()
}


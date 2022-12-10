use crate::utils::AccMapExtender;

pub const DATA_PATH: &str = "data/day10.txt";

fn parse(input: &String) -> impl Iterator<Item = i32> + '_ {
  input.trim().lines().flat_map(|line|
      if line == "noop" { return vec![0].into_iter(); }
      else { vec![0, line.split(" ").nth(1).unwrap().parse::<i32>().unwrap()].into_iter() }
    ).acc_map(1, |acc, x| (acc + x, acc))
}

pub fn p1(input: String) -> i32 {
  parse(&input).skip(19).enumerate()
    .filter(|(i,_)| i % 40 == 0)
    .map(|(i, x)| (i + 20) as i32 * x)
    .sum()
}

pub fn p2(input: String) -> String {
  parse(&input).enumerate()
    .map(|(i, x)|
      if num::abs(x - i as i32 % 40) > 1 { (i,'.') }
      else { (i,'#') }
    ).flat_map(|(i,x)|
      if i % 40 == 0 { vec!['\n', x ].into_iter() }
      else { vec![x].into_iter() }
    ).collect()
}

use std::{collections::VecDeque};

use regex::Regex;

pub const DATA_PATH: &str = "data/day11.txt";

struct Monkey {
  items: VecDeque<usize>,
  operation: Box<dyn Fn(usize) -> usize>,
  test: Box<dyn Fn(usize) -> usize>,
  inspections: usize,
}

fn parse(input: String) -> (Vec<Monkey>, usize) {
  let mut super_divi = 1;
  let re_monkey = Regex::new(r"Monkey.*").unwrap();
  (re_monkey.split(&input).map(|monkey| monkey.trim())
    .filter(|s| s.len() > 0)
    .map(|monkey_data| {
      let operation_cap = Regex::new(r"(\+|\*) (old)?(\d+)?").unwrap().captures(
          Regex::new(r"new = old (.*)").unwrap()
          .captures(monkey_data).unwrap().get(1).unwrap().as_str()
        ).unwrap();

      let opp_sym = operation_cap.get(1).unwrap().as_str();
      let opp_combine: Box<dyn Fn(usize, usize) -> usize> = if opp_sym == "+" {
        Box::new(|x:usize, y:usize| x + y)
      } else {
        Box::new(|x: usize, y: usize| x * y)
      };

      let opp_data = operation_cap.get(2);
      let opp: Box<dyn Fn(usize) -> usize> = if let Some(_) = opp_data {
        Box::new(move |x| (opp_combine)(x, x))
      } else {
        let val = operation_cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Box::new(move |x| (opp_combine)(x, val))
      };

      let divi = Regex::new(r"divisible by (\d+)").unwrap().captures(monkey_data)
        .unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
      super_divi *= divi;
      let if_true = Regex::new(r"If true: throw to monkey (\d+)").unwrap().captures(monkey_data)
        .unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
      let if_false = Regex::new(r"If false: throw to monkey (\d+)").unwrap().captures(monkey_data)
        .unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();

      let test_fn: Box<dyn Fn(usize) -> usize> = Box::new(move |x|
        if x % divi == 0  { if_true }
        else { if_false }
      );

      Monkey {
        items: Regex::new(r"items: (.*)").unwrap()
          .captures(monkey_data).unwrap().get(1).unwrap().as_str()
          .split(", ").map(|d| d.parse::<usize>().unwrap()).collect(),
        operation: opp,
        test: test_fn,
        inspections: 0,
      }
    }).collect::<Vec<_>>(), super_divi)
}

fn monkey_pass<F>(monkeys: &mut Vec<Monkey>, i: usize, reduction: F)
where
  F: Fn(usize)->usize
{
  if let Some(monkey) = monkeys.get_mut(i) {
    let mut throws = vec![];
    while let Some(item) = monkey.items.pop_front() {
      monkey.inspections += 1;
      let mut new_value = (monkey.operation)(item);
      new_value = reduction(new_value);
      let to_monkey = (monkey.test)(new_value);

      throws.push((to_monkey, new_value));
    }

    for (to_monkey, new_value) in throws {
      monkeys.get_mut(to_monkey).unwrap().items.push_back(new_value);
    }

    monkey_pass(monkeys, i + 1, reduction);
  }
}

pub fn p1(input: String) -> usize {
  let (mut monkeys, _) = parse(input);
  for _ in 0..20 {
    monkey_pass(&mut monkeys, 0, |x| x / 3);
  }
  for (i,monkey) in monkeys.iter().enumerate() {
    println!("Monkey {i} inspected {}", monkey.inspections);
  }
  0
}

pub fn p2(input: String) -> usize {
  let (mut monkeys, super_divi) = parse(input);
  for _ in 0..10000 {
    monkey_pass(&mut monkeys, 0, |x| x % super_divi);
  }
  for (i,monkey) in monkeys.iter().enumerate() {
    println!("Monkey {i} inspected {}", monkey.inspections);
  }
  0
}

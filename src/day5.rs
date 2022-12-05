fn parse_input(input: &String) -> (Vec<Vec<char>>, usize) {
  let mut piles: Vec<Vec<char>> = Vec::new();
  let mut line_start: usize = 0;

  for (i, line) in input.lines().enumerate() {
    let mut j = 0;

    if loop {
      let in_crate = &line[(j * 4)..((j * 4) + 3)];

      if in_crate == "   " {
      } else if in_crate == " 1 " {
        line_start = i;
        break true;
      } else {
        for _ in piles.len()..(j + 1) {
          piles.push(Vec::new());
        }
        let b = in_crate.chars().nth(1).unwrap();
        piles[j].insert(0, b);
      }

      j += 1;
      if line.len() <= j * 4 {
        break false;
      }
    } {
      break;
    }
  };

  return (piles, line_start);
}

pub fn p1(input: String) -> String {

  let (mut piles, line_start) = parse_input(&input);

  for line in input.lines().skip(line_start + 2) {
    let mut words = line.split(" ");
    let amount = words.nth(1).unwrap().parse::<usize>().unwrap();

    let from_pile = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;

    let to_pile = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;

    for _ in 0..amount {
      let from_pile_len = piles[from_pile].len();
      let moving = piles[from_pile].remove(from_pile_len - 1);
      piles[to_pile].push(moving);
    }
  }

  let mut res = String::new();
  for pile in piles {
    res.push(*pile.last().unwrap());
  }
  res
}

pub fn p2(input: String) -> String {

  let (mut piles, line_start) = parse_input(&input);

  for line in input.lines().skip(line_start + 2) {
    let mut words = line.split(" ");
    let amount = words.nth(1).unwrap().parse::<usize>().unwrap();

    let from_pile = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let from_pile_len = piles[from_pile].len();

    let to_pile = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;

    let mut moving = piles[from_pile].split_off(from_pile_len - amount);
    piles[to_pile].append(&mut moving);
  }

  let mut res = String::new();
  for pile in piles {
    res.push(*pile.last().unwrap());
  }
  res
}

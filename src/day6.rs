
pub const DATA_PATH: &str = "data/day6.txt";

pub fn p1(input: String) -> i32 {
  let mut dups = 0;
  let mut buffer = String::new();

  for (i, c) in input.chars().enumerate() {
    if buffer.find(c).is_some() {
      dups += 1;
    }
    buffer.push(c);

    if buffer.len() > 4 {
      let c = buffer.remove(0);
      if buffer.find(c).is_some() {
        dups -= 1;
      }
    }

    if buffer.len() == 4 {
      if dups == 0 {
        return i as i32 + 1;
      }
    }
  }
  panic!()
}

pub fn p2(input: String) -> i32 {
  let mut dups = 0;
  let mut buffer = String::new();

  for (i, c) in input.chars().enumerate() {
    if buffer.find(c).is_some() {
      dups += 1;
    }
    buffer.push(c);

    if buffer.len() > 14 {
      let c = buffer.remove(0);
      if buffer.find(c).is_some() {
        dups -= 1;
      }
    }

    if buffer.len() == 14 {
      if dups == 0 {
        return i as i32 + 1;
      }
    }
  }
  panic!()
}
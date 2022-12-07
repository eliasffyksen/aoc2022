
fn get_pri(b: u8) -> u8 {
  if b >= b'a' {
    b - b'a' + 1
  } else {
    b - b'A' + 27
  }
}

fn to_bmap(s: &str) -> u64 {
  s.bytes()
    .map(|b| (1 as u64) << get_pri(b))
    .reduce(|a, b| a | b).unwrap()
}

fn what_bit(mut mask: u64) -> i32 {
  let mut sum = 0;
  while mask > 1 {
    sum += 1;
    mask >>= 1;
  }
  sum
}

pub fn p1(input: String) -> i32 {
  input.trim().lines()
    .map(|line| line.split_at(line.len() / 2))
    .map(|(a, b)| what_bit(to_bmap(a) & to_bmap(b)))
    .sum()
}

pub fn p2(input: String) -> i32 {
  input.trim().lines()
    .map(to_bmap)
    .array_chunks::<3>()
    .map(|group| group.into_iter()
      .reduce(|group, elf| (group & elf)).unwrap()
    ).map(what_bit)
    .sum()
}
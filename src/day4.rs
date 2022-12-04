
fn parse_elf(input: &str) -> (i32, i32) {
  let (a, b) = input.split_once("-").unwrap();
  (a.parse().unwrap(), b.parse().unwrap())
}

pub fn p1(input: String) -> i32 {
  input.trim().lines().map(|line| {
    let (elf1, elf2) = line.split_once(",").unwrap();
    let (elf1_s, elf1_e) = parse_elf(elf1);
    let (elf2_s, elf2_e) = parse_elf(elf2);
    if (elf1_s <= elf2_s && elf1_e >= elf2_e)
      || (elf2_s <= elf1_s && elf2_e >= elf1_e)
    {
      1 as i32
    } else {
      0 as i32
    }
  }).sum::<i32>()
}

pub fn p2(input: String) -> i32 {
  input.trim().lines().map(|line| {
    let (elf1, elf2) = line.split_once(",").unwrap();
    let (elf1_s, elf1_e) = parse_elf(elf1);
    let (elf2_s, elf2_e) = parse_elf(elf2);
    if (elf2_s <= elf1_e && elf2_e >= elf1_s)
      || (elf1_s <= elf2_e && elf1_e >= elf2_s) {
      1 as i32
    } else {
      0 as i32
    }
  }).sum::<i32>()
}

fn getpri(b: u8) -> u8 {
  if b >= b'a' {
    b - b'a' + 1
  } else {
    b - b'A' + 27
  }
}

fn whatbit(mut mask: u64) -> i32 {
  let mut sum = 0;
  while mask > 1 {
    sum += 1;
    mask >>= 1;
  }
  sum
}

pub fn p1(input: String) -> u64 {
  input.trim().split("\n")
    .map(|line| {
      let mid = line.len() / 2;
      let a = &line[..mid];
      let b = &line[mid..];
      let mut bmap: u64 = 0;

      for x in a.bytes() {
        bmap |= 1 << getpri(x)
      }

      for x in b.bytes() {
        let pri = getpri(x);
        if bmap & (1 << pri) != 0 {
          return pri as u64
        }
      }

      return 0
    })
    .sum::<u64>()
}

pub fn p2(input: String) -> i32 {
  input.trim().lines()
    .array_chunks::<3>()
    .map(|group| group.iter()
      .map(|elf| elf.bytes()
        .map(|b| (1 as u64) << getpri(b))
        .reduce(|elf, c| elf | c).unwrap()
      ).reduce(|group, elf| (group & elf)).unwrap()
    ).map(whatbit).sum::<i32>()
}
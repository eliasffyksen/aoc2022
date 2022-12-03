
fn getpri(b: u8) -> u8 {
  if b >= b'a' {
    b - b'a' + 1
  } else {
    b - b'A' + 27
  }
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
  let mut sum = 0;
  let mut group_mask = 0;
  for (i, line) in input.trim().split("\n").enumerate() {
    let mut elf_mask: u64 = 0;

    for b in line.bytes() {
      elf_mask |= 1 << getpri(b)
    }

    if i % 3 == 0 {
      group_mask = elf_mask;
    } else {
      group_mask &= elf_mask;

      if i % 3 == 2 {
        for j in 0..53 {
          if group_mask == 1 {
            sum += j;
            break;
          }
          group_mask >>= 1;
        }
      }
    }
  }
  sum
}
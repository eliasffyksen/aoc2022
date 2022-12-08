
pub const DATA_PATH: &str = "data/day8.txt";

fn walker(
  data: &Vec<Vec<i32>>,
  line: &mut Vec<Vec<bool>>,
  x: usize, y: usize,
  dx: usize, dy: usize,
  mut max: i32
) -> i32 {
  if data[x][y] > max {
    max = data[x][y];
    line[x][y] = true;
  }

  let nx = x + dx;
  let ny = y + dy;

  if nx == data.len() || ny == data.len() {
    line[x][y] = true;
    return data[x][y];
  }

  max = walker(data, line, nx, ny, dx, dy, max);

  if data[x][y] > max {
    line[x][y] = true;
    max = data[x][y];
  }

  max
}

pub fn p1(input: String) -> usize {
  let grid = input.trim().lines().map(|line| {
    line.bytes().map(|c| (c - b'0') as i32)
      .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

  let mut tb = grid.iter()
    .map(|x| x.iter().map(|_| false).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut lr = grid.iter()
    .map(|x| x.iter().map(|_| false).collect::<Vec<_>>())
    .collect::<Vec<_>>();

  for i in 0..grid.len() {
    walker(&grid, &mut tb, 0, i, 1, 0, -1);
    walker(&grid, &mut lr, i, 0, 0, 1, -1);
  }

  tb.iter().map(|x| x.iter())
    .flatten()
    .zip(lr.iter().map(|x| x.iter()).flatten())
    .map(|(a,b)| if *a || *b { 1 } else { 0 })
    .sum()
}

pub fn p2(input: String) -> usize {
  unimplemented!()
}

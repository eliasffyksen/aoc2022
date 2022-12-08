
pub const DATA_PATH: &str = "data/day8.txt";

pub fn get_max(
  data: &Vec<Vec<i32>>,
  x: i32, y: i32,
  dx: i32, dy: i32,
) -> i32 {

  let nx = x + dx;
  let ny = y + dy;

  if nx < 0 || nx >= data.len() as i32
    || ny < 0 || ny >= data[0].len() as i32
  {
    return -1;
  }

  let nxi: usize = nx.try_into().unwrap();
  let nyi: usize = ny.try_into().unwrap();

  let max = get_max(data, nx, ny, dx, dy);
  let next = data[nxi][nyi];
  if next > max {
    return next;
  }
  return max;
}

pub fn p1(input: String) -> usize {
  let grid = input.trim().lines().map(|line| {
    line.bytes().map(|c| (c - b'0') as i32)
      .collect::<Vec<_>>()
  }).collect::<Vec<_>>();

  let mut count = 0;
  for x in 0..grid.len() {
    for y in 0..grid[x].len() {
      let xi:i32 = x.try_into().unwrap();
      let yi:i32 = y.try_into().unwrap();

      if grid[x][y] > get_max(&grid, xi, yi, 1, 0)
        || grid[x][y] > get_max(&grid, xi, yi, -1, 0)
        || grid[x][y] > get_max(&grid, xi, yi, 0, 1)
        || grid[x][y] > get_max(&grid, xi, yi, 0, -1)
      {
        count += 1;
      } else {
      }
    }
  }

  count
}

pub fn scenic_score(
  data: &Vec<Vec<i32>>,
  x: i32, y: i32,
  dx: i32, dy: i32,
  h: i32,
) -> i32 {

  let nx = x + dx;
  let ny = y + dy;

  if nx < 0 || nx >= data.len() as i32
    || ny < 0 || ny >= data[0].len() as i32
  {
    return 0;
  }
  let nxi: usize = nx.try_into().unwrap();
  let nyi: usize = ny.try_into().unwrap();

  if data[nxi][nyi] >= h {
    return 1;
  } else {
    return 1 + scenic_score(data, nx, ny, dx, dy, h);
  }
}

pub fn p2(input: String) -> usize {
  let grid = input.trim().lines().map(|line| {
  line.bytes().map(|c| (c - b'0') as i32)
      .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

  let mut best = 0;

  for x in 0..grid.len() {
    for y in 0..grid[x].len() {
      let xi:i32 = x.try_into().unwrap();
      let yi:i32 = y.try_into().unwrap();

      let a = scenic_score(&grid, xi, yi, 1, 0, grid[x][y]);
      let b = scenic_score(&grid, xi, yi, -1, 0, grid[x][y]);
      let c = scenic_score(&grid, xi, yi, 0, 1, grid[x][y]);
      let d = scenic_score(&grid, xi, yi, 0, -1, grid[x][y]);

      let score = a * b * c * d;
      if score > best {
        best = score;
      }
    }
  }

  best as usize
}

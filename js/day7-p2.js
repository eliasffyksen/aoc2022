let fs = require('fs');

let input = `$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`;

input = fs.readFileSync('data/day7.txt', {encoding: 'utf-8'});
input = input.trim();
console.log('INPUT:\n', input);


let root = {};
let cwd = root;

let re_cd = /^\$ cd (.*)/;
let re_file = /^(\d+) (.+)/;

for (let x of input.split("\n")) {
  let match = x.match(re_cd);
  if (match) {
    let name = match[1];
    console.log('Entering',name);

    if (name == '/') {
      cwd = root;
    } else {
      if (!cwd[name]) {
        console.log("Creating dir", name);
        cwd[name] = {
          '..': cwd
        };
      }
      cwd = cwd[name];
    }
    continue;
  }

  match = x.match(re_file);
  if (match) {
    let size = parseInt(match[1]);
    let name = match[2];
    console.log("Reading file", name, size);
    cwd[name] = size;
  }
}

let THRESHOLD = 100000;

function rec_count(dir, path='') {
  let size = 0;
  let count = 0;

  for (let k in dir) {
    if (k == '..') {
      continue;
    }
    let v  = dir[k];

    if (typeof v == 'number') {
      console.log(`${path}/${k}`, size);
      size += v;
    } else {
      let res = rec_count(v, `${path}/${k}`);

      size += res.size;
      count += res.count;

      if (res.size < THRESHOLD) {
        count += res.size;
      }
    }
  }
  console.log(path, size);

  return {count, size};
}

let TOTAL_SPACE = 70000000;
let SPACE_NEEDED = 30000000;

let summary = rec_count(root);
let space_left = TOTAL_SPACE - summary.size;
let space_to_free = SPACE_NEEDED - space_left;

console.log('summary:', summary);
console.log('space left:', space_left);
console.log('space to free:', space_to_free);

let best = null;

function find_rec(dir, path='') {
  let size = 0;
  let count = 0;

  for (let k in dir) {
    if (k == '..') {
      continue;
    }
    let v  = dir[k];

    if (typeof v == 'number') {
      console.log(`${path}/${k}`, size);
      size += v;
    } else {
      let res = find_rec(v, `${path}/${k}`);

      size += res.size;
      count += res.count;

      if (res.size < THRESHOLD) {
        count += res.size;
      }
    }
  }
  console.log(path, size);

  if (size > space_to_free) {
    if (!best || size < best) {
      console.log('Best canidate', path, size);
      best = size;
    }
  }

  return {count, size};
}

find_rec(root);
console.log(best);

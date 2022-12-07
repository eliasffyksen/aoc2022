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
console.log('INPUT:\n', input);

input = input.trim();

let root = {};
let cwd = root;

let re_cd = /^\$ cd (.*)/;
let re_file = /^(\d+) (\w+)/;

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

function rec_count(dir, root=false) {
  let size = 0;
  let count = 0;

  for (let k in dir) {
    if (k == '..') {
      continue;
    }
    let v  = dir[k];

    if (typeof v == 'number') {
      size += v;
    } else {
      let res = rec_count(v);

      size += res.size;
      count += res.count;

      if (res.size < THRESHOLD) {
        count += res.size;
      }
    }
  }

  return {count, size};
}


console.log(rec_count(root, true));

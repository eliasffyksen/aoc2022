use std::{collections::HashMap, rc::Rc, cell::RefCell, borrow::BorrowMut};

use regex::Regex;


pub const DATA_PATH: &str = "data/day7.txt";

type Directory = Rc<RefCell<HashMap<String, Entry>>>;

enum Entry {
  File(usize),
  Dir(Directory),
}

pub fn p1(input: String) -> usize {
  let re_cd = Regex::new(r"$ cd (.*)").unwrap();
  let re_ls = Regex::new(r"$ ls").unwrap();
  let re_file = Regex::new(r"(\d+) (.*)").unwrap();

  let root: Directory = Default::default();

  let mut path = vec![];

  let mut input = input.trim().lines();

  for line in input {
    if let Some(cap) = re_cd.captures(line) {
      let dir = cap.get(1).unwrap().as_str();
      println!("Entring {:?}", dir);

      if dir == "/" {
        path = vec![];
      } if dir == ".." {
        path.pop();
      } else {
        path.push(dir.to_string());
      }

    } else if let Some(cap) = re_file.captures(line) {
      let size = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
      let name = cap.get(1).unwrap().as_str();
      println!("FILE: {:?} {name} {size}", path);

      let mut path = path.clone();
      let mut current = root.as_ref();

      while path.len() > 0 {
        let key = path.remove(0);
        
        if let Entry::Dir(dir) = current.entry(key).or_insert(Entry::Dir(Default::default())) {
          tmp = dir.borrow_mut();

        } else {
          panic!("Not a directory");
        }
      }
    }
  }
  unimplemented!()
}

pub fn p2(input: String) -> usize {
  unimplemented!()
}

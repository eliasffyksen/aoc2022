use std::collections::HashMap;
use regex::Regex;

pub const DATA_PATH: &str = "data/day7.txt";

#[derive(Debug, Default)]
struct Node {
    size: usize,
    children: HashMap<String, Node>
}

fn enter<'a>(dir: &mut Node, lines: &mut impl Iterator<Item = &'a str>) -> bool {
    let re_cd = Regex::new(r"^\$ cd (.+)").unwrap();
    let re_file = Regex::new(r"^(\d+) (.+)").unwrap();

    let mut next = lines.next();
    let mut exit_all = false;

    while let Some(line) = next {
        if let Some(cap) = re_cd.captures(line) {
            let dir_name = cap.get(1).unwrap().as_str();

            if dir_name == ".." {
                break;
            } else if dir_name == "/" || enter(dir.children.entry(dir_name.to_string()).or_default(), lines)
            {
                exit_all = true;
                break;
            }

        } else if let Some(cap) = re_file.captures(line) {
            let size = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let file_name = cap.get(2).unwrap().as_str();
            dir.children.entry(file_name.to_string()).or_default().size = size;
        }
        next = lines.next();
    }

    dir.size = dir.children.iter().map(|(_, v)| v.size).sum();

    exit_all
}

fn parse(input: String) -> Node {
    let mut reader = input.trim().lines().peekable();
    let mut tree: Node = Default::default();

    while let Some(_) = reader.peek() {
        enter(&mut tree, &mut reader);
    }

    tree
}

pub fn p1(input: String) -> usize {
    parse(input).size
}

fn find_space(tree: &Node, target_size: usize) -> usize {
    if tree.children.len() == 0 {
        return usize::MAX;
    }

    tree.children.iter()
        .map(|(_, node)| find_space(node, target_size))
        .chain(vec![tree.size,usize::MAX])
        .filter(|v| *v > target_size)
        .min().unwrap()
}

pub fn p2(input: String) -> usize {
    let tree = parse(input);
    let unused_space = 70000000 - tree.size;
    let space_to_free = 30000000 - unused_space;

    find_space(&tree, space_to_free)
}

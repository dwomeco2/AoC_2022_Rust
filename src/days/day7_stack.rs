#![allow(warnings, unused)]
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day7");

#[derive(Debug)]
struct FsEntry {
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|f| f.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self.clone()).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|f| f.all_dirs()),
            ),
        )
    }
}

pub fn part1() {
    let dir_tree = construct_dir_tree();
    let total_size = dir_tree
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|s| s <= &100_000u64)
        .sum::<u64>();
    println!("{total_size}");
}

fn construct_dir_tree() -> FsEntry {
    let v = input_parsing();

    let root = FsEntry {
        size: 0,
        children: vec![],
    };

    let mut stack = vec![root];

    for l in v {
        match l {
            Line::Command(cmd) => match cmd {
                Command::Cd(path) if path == ".." => {
                    let node = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(node);
                }
                Command::Cd(_) => {
                    let node = FsEntry {
                        size: 0,
                        children: vec![],
                    };
                    stack.push(node);
                }
                Command::Ls => continue,
            },
            Line::Entry(ent) => match ent {
                Entry::Dir(_) => continue,
                Entry::File(size, _) => {
                    let node = FsEntry {
                        size: size as u64,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    let mut root = stack.pop().unwrap();
    while !stack.is_empty() {
        let mut node = stack.pop().unwrap();
        node.children.push(root);
        root = node;
    }
    root
}

fn input_parsing() -> Vec<Line> {
    INPUT
        .lines()
        .skip(1)
        .map(
            |l| match l.split_ascii_whitespace().collect_vec().as_slice() {
                ["$", "cd", ".."] => Line::Command(Command::Cd(String::from(".."))),
                ["$", "cd", path] => Line::Command(Command::Cd(String::from(*path))),
                ["$", "ls"] => Line::Command(Command::Ls),
                ["dir", name] => Line::Entry(Entry::Dir(String::from(*name))),
                [size, name] => Line::Entry(Entry::File(
                    size.parse::<usize>().unwrap(),
                    String::from(*name),
                )),
                _ => unreachable!(),
            },
        )
        .collect::<Vec<_>>()
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Entry {
    Dir(String),
    File(usize, String),
}

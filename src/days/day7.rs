use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day7");

// https://fasterthanli.me/series/advent-of-code-2022/part-7

pub fn part1() {
    let root = construct_dir_tree();
    let sum = Node::all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    println!("{sum}");
}

pub fn part2() {
    let root = construct_dir_tree();

    let total = 70_000_000u64;
    let total_used = root.borrow().total_size();
    let unused_total = total - total_used;
    let needed = 30_000_000u64 - unused_total;

    let result = Node::all_dirs(root)
        .map(|d| d.borrow().total_size())
        .sorted_unstable()
        .find(|s| s >= &needed)
        .unwrap();
    println!("{result}");
}

fn construct_dir_tree() -> Rc<RefCell<Node>> {
    let v = input_parsing();

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for l in v {
        match l {
            Line::Command(cmd) => match cmd {
                Command::Cd(path) if path == ".." => {
                    let parent = node.borrow().parent.clone().unwrap();
                    node = parent;
                }
                Command::Cd(path) => {
                    let child = node.borrow_mut().children.entry(path).or_default().clone();
                    node = child;
                }
                Command::Ls => continue,
            },
            Line::Entry(ent) => match ent {
                Entry::Dir(name) => {
                    let entry = node.borrow_mut().children.entry(name).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, name) => {
                    let entry = node.borrow_mut().children.entry(name).or_default().clone();
                    entry.borrow_mut().size = size;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            },
        }
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

impl fmt::Debug for PrettyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            for (index, line) in format!("{:?}", PrettyNode(child.clone()))
                .lines()
                .enumerate()
            {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }
    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|v| v.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
    fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
        #[allow(clippy::needless_collect)]
        let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

        Box::new(
            std::iter::once(n).chain(
                children
                    .into_iter()
                    .filter_map(|c| {
                        if c.borrow().is_dir() {
                            Some(Node::all_dirs(c))
                        } else {
                            None
                        }
                    })
                    .flatten(),
            ),
        )
    }
}

struct PrettyNode(NodeHandle);

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Debug, Default)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
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

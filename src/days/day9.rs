use std::{iter, collections::HashSet};

const INPUT: &str = include_str!("../../input/day9");

pub fn part1() {
    let mut rope = Rope::new(2);
    let result = tail_visited_count(rope);
    println!("{result}");
}

pub fn part2() {
    let mut rope = Rope::new(10);
    let result = tail_visited_count(rope);
    println!("{result}");
}

fn tail_visited_count(rope: Rope) -> usize {
    let mut hs: HashSet<Cor> = HashSet::new();
    
    for l in INPUT.lines() {
        let mut iter = l.split_whitespace();
        let dir: Cor = match iter.next().unwrap() {
            "U" => (0, 1).into(),
            "D" => (0, -1).into(),
            "R" => (1, 0).into(),
            "L" => (-1, 0).into(),
            _ => unreachable!(),
        };
        let steps = iter.next().unwrap().parse::<usize>().unwrap();
        
        for d in iter::repeat(dir).take(steps) {
            rope.move_head(d);
            hs.insert(rope.tail().unwrap());
        }
    }
    hs.iter().count()
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Default)]
struct Cor {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Cor {
    fn from(cor: (isize, isize)) -> Self {
        Self { x: cor.0, y: cor.1 }
    }
}

impl Cor {
    fn is_touching(&self, other: Cor) -> bool {
        let delta = (self.x - other.x, self.y - other.y);
        delta.0 >= -1 && delta.0 <=1 && delta.1 >= -1 && delta.1 <= 1
    }
    
    fn follow(&self, other: Cor) -> Option<Cor> {
        if self.is_touching(other) {
            return None;
        }
        
        let delta_x = match self.x.cmp(&other.x) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            _ => 0,
        };
        let delta_y = match self.y.cmp(&other.y) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            _ => 0,
        };
        
        Some((other.x + delta_x, other.y + delta_y).into())
    }
}

#[derive(Default)]
struct Rope {
    knots: Vec<Cor>,
}

impl Rope {
    pub fn new(knot_count: usize) -> Self {
        Self {
            knots: vec![Cor::default(); knot_count],
        }
    }
    
    fn move_head(&mut self, cor: Cor) {
        let head = self.knots[0];
        self.knots[0] = Cor { x: head.x + cor.x, y: head.y + cor.y };
        
        for i in 0..self.knots.len() - 1 {
            let result = self.knots[i].follow(self.knots[i + 1]);
            if result.is_some() {
                self.knots[i + 1] = result.unwrap();
            }
        }

    }

    fn tail(&self) -> Option<Cor> {
        self.knots.last().cloned()
    }
}


use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day14");

pub fn part1() {
    let mut grid = parse(1);
    let origin = GridCor { x: 500, y: 0 };
    let mut sand = origin;
    let mut counter = 0;
    loop {
        sand = grid.next_sand_cor(sand);
        if sand.y == grid.height - 1 {
            break;
        }
        counter += 1;
        if let Some(c) = grid.cell_mut(sand) {
            *c = Cell::Rest;
            sand = origin;
        }
    }
    println!("{counter}");
}

pub fn part2() {
    let mut grid = parse(2);
    let origin = GridCor { x: 500, y: 0 };
    let mut sand = origin;
    let mut counter = 0;
    loop {
        sand = grid.next_sand_cor(sand);
        counter += 1;
        if sand == origin {
            break;
        }
        if let Some(c) = grid.cell_mut(sand) {
            *c = Cell::Rest;
            sand = origin;
        }
    }
    println!("{counter}");
}

fn parse(hd: usize) -> Grid {
    let rocks = parse_rock();
    let grid_height = rocks
        .iter()
        .max_by(|&a, &b| a.y.cmp(&b.y))
        .map(|x| x.y)
        .unwrap();
    let grid = Grid::new(1000, grid_height + hd);
    grid.update_rocks(rocks)
}

fn parse_rock() -> Vec<GridCor> {
    INPUT.lines().flat_map(|l| {
        l
        .split(" -> ")
        .map(|cor| {
            let mut a = cor.split(',');
            let x = a.next().unwrap().parse::<usize>().unwrap();
            let y = a.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .tuple_windows()
        .map(|((x1, y1), (x2, y2))| {
            let r = |a: usize, b: usize| if a < b { a..=b } else { b..=a };
            if x1 == x2 {
                r(y1, y2).map(|y| GridCor { x: x1, y }).collect::<Vec<_>>()
            } else {
                r(x1, x2).map(|x| GridCor { x, y: y1 }).collect::<Vec<_>>()
            }
        })
    }).flatten().unique().collect::<Vec<_>>()
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            items: vec![Cell::Air; width * height],
        }
    }

    fn update_rocks(mut self, rocks: Vec<GridCor>) -> Self {
        for c in rocks {
            self.items[c.y * self.width + c.x] = Cell::Rock;
        }
        self
    }

    fn in_bound(&self, c: GridCor) -> bool {
        c.x < self.width && c.y < self.height
    }

    fn cell(&self, c: GridCor) -> Option<&Cell> {
        if !self.in_bound(c) {
            return None
        }
        Some(&self.items[c.y * self.width + c.x])
    }

    fn cell_mut(&mut self, c: GridCor) -> Option<&mut Cell> {
        if !self.in_bound(c) {
            return None
        }
        Some(&mut self.items[c.y * self.width + c.x])
    }

    fn find_unblocked(&self, c: GridCor) -> Option<GridCor> {
        let delta: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)]; // middle, left, right
        
        for d in delta {
            let dd: GridCor = (c.x.checked_add_signed(d.0)?, c.y.checked_add_signed(d.1)?).into();
            if let Some(Cell::Air) = self.cell(dd) {
                return Some(dd);
            }
        }

        None
    }

    fn next_sand_cor(&self, c: GridCor) -> GridCor {
        let mut cc = c;
        while let Some(new_c) = self.find_unblocked(cc) { // loop when can't rest
            cc = new_c;
        }
        cc
    }

}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Rock,
    Rest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct GridCor {
    x: usize,
    y: usize
}

struct Grid {
    width: usize,
    height: usize,
    items: Vec<Cell>,
}

impl From<(usize, usize)> for GridCor {
    fn from(c: (usize, usize)) -> Self {
        Self { x: c.0, y: c.1 }
    }
}
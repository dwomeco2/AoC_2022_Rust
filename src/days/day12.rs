use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day12");

pub fn part1() {
    let grid = create_grid();
    let start = grid.start;
    find_path(grid, start, 26, false);
}

pub fn part2() {
    let grid = create_grid();
    let start = grid.end;
    find_path(grid, start, 1, true);
}

fn find_path(grid: Grid, start: GridCell, target: usize, reverse: bool) {
    let mut visited: HashSet<GridCell> = HashSet::new();
    let mut pathes: Vec<GridCell> = vec![start];

    let mut step_count = 0;

    'outer: loop {
        let mut tmp = vec![];
        while let Some(curr_path) = pathes.pop() {
            if grid.cell(curr_path).unwrap() == target {
                break 'outer;
            }
            for c in grid.walkable_neighbors(curr_path, reverse) {
                if visited.contains(&c) {
                    continue;
                }
                visited.insert(c);
                tmp.push(c);
            }
        }
        pathes.append(&mut tmp);
        step_count += 1;
    }
    println!("{step_count}");
}

fn create_grid() -> Grid {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();
    let mut grid: Grid = Grid::new(width, height);

    let elevation = |c: char| match c {
        'S' => 0,
        'E' => 26,
        c => (c as u8 - b'a') as usize + 1,
    };

    for (x, l) in INPUT.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            if c == 'S' {
                grid.start = (x, y).into();
            } else if c == 'E' {
                grid.end = (x, y).into();
            }
            *grid.cell_mut((x, y).into()) = (elevation)(c);
        }
    }
    grid
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<usize>,
    start: GridCell,
    end: GridCell,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
struct GridCell {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCell {
    fn from(value: (usize, usize)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            cells: vec![Default::default(); width * height],
            start: GridCell::default(),
            end: GridCell::default(),
        }
    }

    fn in_bound(&self, cell: GridCell) -> bool {
        cell.x < self.height && cell.y < self.width
    }

    fn cell(&self, cell: GridCell) -> Option<usize> {
        if !self.in_bound(cell) {
            return None;
        }
        Some(self.cells[cell.x * self.width + cell.y])
    }

    fn cell_mut(&mut self, cell: GridCell) -> &mut usize {
        &mut self.cells[cell.x * self.width + cell.y]
    }

    fn walkable_neighbors(&self, me: GridCell, reverse: bool) -> Vec<GridCell> {
        let step: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        step.iter()
            .filter_map(|(dx, dy)| {
                let x = me.x.checked_add_signed(*dx)?;
                let y = me.y.checked_add_signed(*dy)?;
                self.in_bound((x, y).into()).then_some((x, y).into())
            })
            .filter(|other| {
                let o = self.cell(*other).unwrap();
                let m = self.cell(me).unwrap();
                if !reverse {
                    o <= m + 1
                } else {
                    o >= m - 1
                }
            })
            .collect_vec()
    }
}

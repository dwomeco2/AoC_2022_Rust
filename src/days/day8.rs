
const INPUT: &str = include_str!("../../input/day8");

pub fn part1() {
    let grid = create_grid();
    let mut count = 0;
    for x in 1..grid.height - 1 {
        for y in 1..grid.width - 1 {
            let curr = (x, y).into();
            let step: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let is_visible = step.iter().any(|step| {
                (1..)
                    .map_while(|i| {
                        let cor = (
                            x.checked_add_signed(step.0 * i)?,
                            y.checked_add_signed(step.1 * i)?,
                        );

                        grid.cell(cor.into())
                    })
                    .all(|c| c < grid.cell(curr).unwrap())
            });
            if is_visible {
                count += 1;
            }
        }
    }
    count = count + grid.height * 2 + grid.width * 2 - 4;
    println!("{count}");
}

pub fn part2() {
    let grid = create_grid();
    let mut score_max: usize = 0;
    for x in 1..grid.height - 1 {
        for y in 1..grid.width - 1 {
            let curr = grid.cell((x, y).into()).unwrap();
            let step: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let scenic_score = step.iter().map(|step| {
                let line = (1..)
                    .map_while(|i| {
                        let cor = (
                            x.checked_add_signed(step.0 * i)?,
                            y.checked_add_signed(step.1 * i)?,
                        );

                        grid.cell(cor.into())
                    });
                let mut score = 0;
                for c in line {
                    score += 1;
                    if c >= curr {
                        break;
                    }
                }
                score
            }).product::<usize>();
            score_max = score_max.max(scenic_score);
        }
    }
    println!("{score_max}");
}

fn create_grid() -> Grid<char> {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();
    let mut grid: Grid<char> = Grid::new(width, height);

    for (x, l) in INPUT.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            *grid.cell_mut((x, y).into()) = c;
        }
    }
    grid
}

#[derive(Debug)]
struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

#[derive(Clone, Copy)]
struct GridCell {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCell {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Copy,
{
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            width,
            height,
            cells: vec![Default::default(); width * height],
        }
    }

    fn in_bound(&self, cell: GridCell) -> bool {
        cell.x < self.height && cell.y < self.width
    }

    fn cell(&self, cell: GridCell) -> Option<T> {
        if !self.in_bound(cell) {
            return None;
        }
        Some(self.cells[cell.x * self.width + cell.y])
    }

    fn cell_mut(&mut self, cell: GridCell) -> &mut T {
        &mut self.cells[cell.x * self.width + cell.y]
    }
}

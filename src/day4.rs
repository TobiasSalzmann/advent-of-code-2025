use array2d::Array2D;
use crate::util::{AdventHelper, GridAccess};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid: Array2D<char> = advent.parse_grid_2d();

    advent.part1("Result: {}", part1(&grid));
    advent.part2("Result: {}", part2(&grid));
}

fn part1(grid: &Array2D<char>) -> usize {
    accessible(grid).len()
}
fn part2(grid: &Array2D<char>) -> usize {
    let mut grid = grid.clone();
    let mut removed = 0;
    loop {
        let accessible = accessible(&grid);
        if accessible.is_empty() {
            return removed;
        }
        for (x, y) in accessible {
            grid.set(x, y, '.').unwrap();
            removed += 1;
        }
    }
}

fn accessible(grid: &Array2D<char>) -> Vec<(usize, usize)> {
    let mut accessible = vec![];
    for (x, y) in grid.indices_row_major() {
        let mut num_rolls = 0;
        if let Some('@') = grid.get_i32(x as i32, y as i32) {
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1), (-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
                if let Some(c) = grid.get_i32(x as i32 + dx, y as i32 + dy) && *c == '@' {
                    num_rolls += 1
                }
            }
            if num_rolls < 4 {
                accessible.push((x, y))
            }
        }
    }
    accessible
}
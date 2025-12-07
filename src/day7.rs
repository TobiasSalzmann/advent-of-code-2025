use array2d::Array2D;
use bit_set::BitSet;
use crate::util::AdventHelper;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let grid = advent.parse_grid_2d();
    advent.part1("Result: {}", part1(&grid));
    advent.part2("Result: {}", part2(&grid));
}

fn part1(grid: &Array2D<char>) -> usize {
    let mut beams = BitSet::new();
    let mut count = 0;
    for row in grid.rows_iter() {
        row.enumerate().for_each(|(i, it)| {
            if *it == 'S' {
                beams.insert(i);
            }
            if *it == '^' && beams.contains(i) {
                beams.remove(i);
                beams.insert(i-1);
                beams.insert(i + 1);
                count += 1
            }
        });
    }
    count
}
fn part2(grid: &Array2D<char>) -> usize {
    let mut beams = vec![0; grid.num_columns()];
    for row in grid.rows_iter() {
        row.enumerate().for_each(|(i, it)| {
            if *it == 'S' {
                beams[i] = 1
            }
            if *it == '^' {
                let count = beams[i];
                beams[i] = 0;
                beams[i-1] += count;
                beams[i+1] += count;
            }
        });
    }
    beams.iter().sum()
}
use crate::util::AdventHelper;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let banks: Vec<Vec<i32>> = advent.parse_from_grid().as_rows();

    advent.part1("Result: {}", part(&banks, 2));
    advent.part2("Result: {}", part(&banks, 12));
}

fn part(banks: &[Vec<i32>], length: usize) -> i64 {
    banks.iter().map(|it| joltage(it, length)).sum()
}

fn joltage(bank: &[i32], length: usize) -> i64 {
    let idx = find_first_max_idx(&bank[0..bank.len() - (length - 1)]);

    if length == 1 {
        bank[idx] as i64
    } else {
        10i64.pow(length as u32 - 1) * bank[idx] as i64 + joltage(&bank[idx + 1..], length - 1)
    }
}

fn find_first_max_idx(values: &[i32]) -> usize {
    let mut max_idx = 0;
    for i in 0..values.len() {
        if values[i] > values[max_idx] {
            max_idx = i;
        }
    }
    max_idx
}

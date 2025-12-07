use array2d::Array2D;
use crate::util::AdventHelper;
use itertools::Itertools;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let rows: Vec<Vec<String>> = advent.parse_sequences_from_strings(" ");
    let rows: Vec<Vec<String>> = rows.iter()
        .map(|it| it.iter().filter(|it| !it.is_empty()).cloned().collect_vec()).collect_vec();
    advent.part1("Result: {}", part1(&rows));

    let rows = advent.parse_grid_2d_fill(' ');


    advent.part2("Result: {}", part2(&rows));
}

fn part1(rows: &[Vec<String>]) -> u64 {
    let mut sum = 0;
    for col in 0..rows.first().unwrap().len() {
        let op = rows[rows.len()-1][col].clone();
        let mut agg = if op == "+" { 0 } else { 1 };
        for row in 0..rows.len() - 1 {
           if op == "+" {
               agg += rows[row][col].parse::<u64>().unwrap();
           } else  {
               agg *= rows[row][col].parse::<u64>().unwrap();
           }
       }
        sum += agg;
    }
    sum
}

fn part2(grid: &Array2D<char>) -> u64 {
    let mut sum = 0;
    let mut numbers = vec![];
    grid.as_columns().iter()
        .rev()
        .for_each(|it| {
            let raw: String = it.iter()
                .dropping_back(1)
                .collect();
             if let Ok(n) = raw.trim().parse::<u64>() {
                numbers.push(n);
            }
            match it.last().unwrap() {
                '+' => {
                    sum += numbers.iter().sum::<u64>();
                    numbers.clear();
                }
                '*' => {
                    sum += numbers.iter().product::<u64>();
                    numbers.clear();
                }
                _ => {}
            }
        });
    sum
}
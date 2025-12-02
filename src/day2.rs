use crate::util::AdventHelper;
use lib_advent_macro::ParseFromStr;
use prse::Parse;
use std::collections::HashSet;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let mut ranges: Vec<Input> = advent.parse_sequence(",");
    ranges.sort_by_key(|it| it.min);

    advent.part1("Result: {}", part1(&ranges));
    advent.part2("Result: {}", part2(&ranges));
}

fn part1(ranges: &[Input]) -> u64 {
    invalid_ids(ranges, 2).iter().sum()
}

fn part2(ranges: &[Input]) -> u64 {
    let max = ranges.iter().map(|it| it.max).max().unwrap();

    let ids: HashSet<u64> = (2..max.to_string().len())
        .flat_map(|it| invalid_ids(ranges, it))
        .collect();
    ids.iter().sum()
}

fn invalid_ids(ranges: &[Input], repeats: usize) -> Vec<u64> {
    let mut n: u64 = 1;
    let mut ids = vec![];
    for range in ranges {
        loop {
            let double = (n.to_string().repeat(repeats)).parse::<u64>().unwrap();
            if double > range.max {
                break;
            }
            if double >= range.min {
                ids.push(double);
            }
            n += 1;
        }
    }
    ids
}

#[derive(Parse, ParseFromStr, PartialEq, Eq, Debug, Hash, Clone)]
#[prse = "{min}-{max}"]
struct Input {
    min: u64,
    max: u64,
}

use crate::util::AdventHelper;
use itertools::Itertools;
use lib_advent_macro::ParseFromStr;
use prse::Parse;
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let (ranges, ids): (Vec<Range>, Vec<u64>) = advent.parse_from_strings_split();

    advent.part1("Result: {}", part1(&ranges, &ids));
    advent.part2("Result: {}", part2(&ranges));
}

fn part1(ranges: &[Range], ids: &[u64]) -> usize {
    ids.iter()
        .filter(|it| contained_by_any(**it, ranges))
        .count()
}

fn contained_by_any(it: u64, ranges: &[Range]) -> bool {
    ranges
        .iter()
        .any(|range| range.min <= it && it <= range.max)
}

fn part2(ranges: &[Range]) -> u64 {
    let ranges = ranges.iter().sorted_by_key(|it| it.min).collect_vec();
    let mut current = **ranges.first().unwrap();
    let mut sum = 0;
    for range in ranges {
        if range.min > current.max {
            sum += current.size();
            current = *range;
        } else {
            current.max = range.max.max(current.max);
        }
    }
    sum + current.size()
}

#[derive(Parse, ParseFromStr, PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[prse = "{min}-{max}"]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn size(&self) -> u64 {
        self.max - self.min + 1
    }
}

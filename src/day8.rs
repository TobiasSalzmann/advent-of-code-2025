use std::str::FromStr;
use bit_set::BitSet;
use itertools::Itertools;
use prse::Parse;
use lib_advent_macro::ParseFromStr;
use crate::util::AdventHelper;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let points: Vec<Point3> = advent.parse_from_strings();

    advent.part1("Result: {}", part1(&points, 1000));
    advent.part2("Result: {}", part2(&points));
}

fn part1(points: &[Point3], cutoff: usize) -> usize {
    let mut pairs: Vec<(usize, usize)> = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            pairs.push((i, j));
        }
    }
    pairs.sort_by_key(|(i, j)| (points[*i].x - points[*j].x).pow(2) + (points[*i].y - points[*j].y).pow(2) + (points[*i].z - points[*j].z).pow(2));
    let mut clusters: Vec<BitSet> = vec![];
    pairs = pairs.into_iter().take(cutoff).collect();
    for (i, j) in pairs {
        if let Some((set_i_idx, _set_i)) = clusters.iter().find_position(|it| it.contains(i))
            && let Some((set_j_idx, set_j)) = clusters.iter().find_position(|it| it.contains(j)) {
            if set_i_idx == set_j_idx {
                continue
            }
            let copy = set_j.clone();
            clusters[set_i_idx].union_with(&copy);
            clusters.remove(set_j_idx);
        } else if let Some(set_i) = clusters.iter_mut().find(|it| it.contains(i)) {
            set_i.insert(j);
        } else if let Some(set_j) = clusters.iter_mut().find(|it| it.contains(j)) {
            set_j.insert(i);
        } else {
            clusters.push(BitSet::from_iter(vec![i, j]));
        }
    }

    clusters.iter().map(|it| it.len()).sorted().rev().take(3).product()
}

fn part2(points: &[Point3]) -> i64 {
    let mut pairs: Vec<(usize, usize)> = vec![];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            pairs.push((i, j));
        }
    }
    pairs.sort_by_key(|(i, j)| (points[*i].x - points[*j].x).pow(2) + (points[*i].y - points[*j].y).pow(2) + (points[*i].z - points[*j].z).pow(2));
    let mut clusters: Vec<BitSet> = vec![];
    for (i, j) in pairs {
        if let Some((set_i_idx, _set_i)) = clusters.iter().find_position(|it| it.contains(i))
            && let Some((set_j_idx, set_j)) = clusters.iter().find_position(|it| it.contains(j)) {
            if set_i_idx == set_j_idx {
                continue
            }
            let copy = set_j.clone();
            clusters[set_i_idx].union_with(&copy);
            clusters.remove(set_j_idx);
        } else if let Some(set_i) = clusters.iter_mut().find(|it| it.contains(i)) {
            set_i.insert(j);
        } else if let Some(set_j) = clusters.iter_mut().find(|it| it.contains(j)) {
            set_j.insert(i);
        } else {
            clusters.push(BitSet::from_iter(vec![i, j]));
        }
        if clusters.len() == 1 && clusters[0].len() == points.len() {
            return points[i].x * points[j].x
        }

    }
    panic!("Something went wrong.");
}

#[derive(Parse, ParseFromStr, PartialEq, Eq, Debug, Hash, Clone)]
#[prse = "{x},{y},{z}"]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}
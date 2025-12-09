use crate::util::AdventHelper;
use itertools::Itertools;
use lib_advent_macro::ParseFromStr;
use prse::Parse;
use std::collections::{HashMap};
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let points: Vec<Point2> = advent.parse_from_strings();

    advent.part1("Result: {}", part1(&points));
    advent.part2("Result: {}", part2(&points));
}

fn part1(points: &[Point2]) -> i64 {
    points
        .iter()
        .tuple_combinations::<(&Point2, &Point2)>()
        .map(|(a, b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .max()
        .unwrap()
}

fn part2(points: &[Point2]) -> i64 {
    let points = compress(points);

    let mut borders = vec![];
    for i in 0..points.len() {
        let r = Rectangle::from_points_compressed(&points[i], &points[(i + 1) % points.len()]);
        for y in r.y_min..=r.y_max {
            for x in r.x_min..=r.x_max {
                borders.push(Point2 { x, y });
            }
        }
    }

    let candidates: Vec<(i64, &Point2Compressed, &Point2Compressed)> = points
        .iter()
        .tuple_combinations::<(&Point2Compressed, &Point2Compressed)>()
        .map(|(a, b)| (((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1), a, b))
        .sorted_by_key(|(it, _, _)| -it)
        .collect_vec();

    for (area, a, b) in candidates {
        let r = Rectangle::from_points_compressed(a, b);
        if borders.iter().any(|it| r.contains_inside(it)) {
            continue;
        }
        return area;
    }

    panic!("No solution found")
}

fn compress(points: &[Point2]) -> Vec<Point2Compressed> {
    let xmap: HashMap<i64, i64> = points
        .iter()
        .map(|it| it.x)
        .sorted()
        .enumerate()
        .map(|(i, x)| (x, i as i64))
        .collect();
    let ymap: HashMap<i64, i64> = points
        .iter()
        .map(|it| it.y)
        .sorted()
        .enumerate()
        .map(|(i, y)| (y, i as i64))
        .collect();

    let points: Vec<Point2Compressed> = points
        .into_iter()
        .map(|p| Point2Compressed {
            x: p.x,
            y: p.y,
            x_c: *xmap.get(&p.x).unwrap(),
            y_c: *ymap.get(&p.y).unwrap(),
        })
        .collect();
    points
}

#[derive(Parse, ParseFromStr, PartialEq, Eq, Debug, Hash, Clone)]
#[prse = "{x},{y}"]
struct Point2 {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point2Compressed {
    x: i64,
    y: i64,
    x_c: i64,
    y_c: i64,
}

struct Rectangle {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Rectangle {
    fn contains_inside(&self, p: &Point2) -> bool {
        p.x > self.x_min && p.x < self.x_max && p.y > self.y_min && p.y < self.y_max
    }

    fn from_points_compressed(a: &Point2Compressed, b: &Point2Compressed) -> Self {
        Rectangle {
            x_min: a.x_c.min(b.x_c),
            x_max: a.x_c.max(b.x_c),
            y_min: a.y_c.min(b.y_c),
            y_max: a.y_c.max(b.y_c),
        }
    }
}

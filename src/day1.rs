use crate::util::AdventHelper;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let lines: Vec<i32> = advent
        .parse_from_strings()
        .into_iter()
        .map(|it: String| {
            it.replace("R", "+")
                .replace("L", "-")
                .parse::<i32>()
                .unwrap()
        })
        .collect();

    advent.part1("Result: {}", part1(&lines));

    advent.part2("Result: {}", part2(&lines));
}

fn part1(lines: &[i32]) -> usize {
    let mut count = 0;
    let mut sum = 50;
    for it in lines {
        sum += it;
        if sum.rem_euclid(100) == 0 {
            count += 1;
        }
    }
    count
}

fn part2(lines: &[i32]) -> usize {
    let mut count = 0;
    let mut sum = 50;
    lines.iter().for_each(|it| {
        if sum == 0 && *it < 0 {
            sum = 100
        }
        sum += it;
        while sum > 100 {
            sum -= 100;
            count += 1
        }
        while (sum < 0) {
            sum += 100;
            count += 1
        }
        if sum.rem_euclid(100) == 0 {
            count += 1;
            sum = 0;
        }
    });
    count
}

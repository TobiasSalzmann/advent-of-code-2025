use crate::util::AdventHelper;
use itertools::Itertools;
use lib_advent_macro::ParseFromStr;
use prse::Parse;
use std::collections::{HashMap};
use std::str::FromStr;
use pathfinding::prelude::{bfs, dijkstra};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let machines: Vec<Machine> = advent.parse_sequences_from_strings::<String>(" ")
        .into_iter()
        .map(|it| parse(&it))
        .collect_vec();

    advent.part1("Result: {}", part1(&machines));
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|it| number_of_presses(it)).sum()
}

fn number_of_presses(machine: &Machine) -> usize {
    let n = machine.target.len();
    let initial_state = vec![false; n];
    let path = bfs(&initial_state, |it| succ(it, &machine.buttons), |it| it == &machine.target).unwrap();
    path.len() - 1
}

fn succ(state: &Vec<bool>, buttons: &[Vec<usize>]) -> Vec<Vec<bool>> {
    buttons.iter().map(|it| apply_button(state, it)).collect_vec()
}

fn apply_button(state: &Vec<bool>, button: &Vec<usize> ) -> Vec<bool> {
    let mut new_state = state.clone();
    for i in button {
        new_state[*i] = !new_state[*i];
    }
    new_state
}

fn parse(input: &[String]) -> Machine {
    let target = input[0]
        .chars().dropping(1).dropping_back(1)
        .map(|it| it == '#')
        .collect_vec();
    let buttons : Vec<Vec<usize>> = input[1..input.len() - 1].iter()
        .map(|it| parse_list(it))
        .collect_vec();
    let joltage_req : Vec<usize> = parse_list(input[input.len() - 1].as_str());
    Machine {target, buttons, joltage_req}
}

fn parse_list(input: &str) -> Vec<usize> {
    let trimmed = &input[1..input.len() - 1];
    trimmed.split(',').map(|it| it.parse().unwrap()).collect_vec()
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_req: Vec<usize>
}
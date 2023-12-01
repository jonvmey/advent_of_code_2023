use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(vals: &[String]) -> usize {
    0
}

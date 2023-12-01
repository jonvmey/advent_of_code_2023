use aoc_runner_derive::{aoc, aoc_generator};

fn process_line_part_1(line : &str) -> i32 {
    let first_digit = line.chars().find(|&c| c.is_digit(10)).unwrap().to_string();
    let last_digit = line.chars().rev().find(|&c| c.is_digit(10)).unwrap().to_string();

    first_digit.parse::<i32>().unwrap() * 10 + last_digit.parse::<i32>().unwrap()
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(lines: &[String]) -> i32 {
    lines.iter().map(|line| process_line_part_1(line)).sum()
}

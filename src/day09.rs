use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{newline, space1};
use nom::multi::separated_list1;
use nom::IResult;

fn calculate_next(samples: &[i32]) -> i32 {
    let differences: Vec<i32> = samples
        .windows(2)
        .map(|elements| elements[1] - elements[0])
        .collect();

    if differences.iter().all(|element| *element == 0) {
        return *samples.last().unwrap();
    }

    samples.last().unwrap() + calculate_next(&differences)
}

fn calculate_previous(samples: &[i32]) -> i32 {
    let differences: Vec<i32> = samples
        .windows(2)
        .map(|elements| elements[1] - elements[0])
        .collect();

    if differences.iter().all(|element| *element == 0) {
        return *samples.first().unwrap();
    }

    samples.first().unwrap() - calculate_previous(&differences)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, nom::character::complete::i32)(input)
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let (_, lines) = separated_list1(newline, parse_line)(input).unwrap();

    lines
}

#[aoc(day9, part1)]
fn part1(sample_histories: &[Vec<i32>]) -> i32 {
    sample_histories
        .iter()
        .map(|samples| calculate_next(samples))
        .sum()
}

#[aoc(day9, part2)]
fn part2(sample_histories: &[Vec<i32>]) -> i32 {
    sample_histories
        .iter()
        .map(|samples| calculate_previous(samples))
        .sum()
}

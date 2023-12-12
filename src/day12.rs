use aoc_runner_derive::{aoc, aoc_generator};
use integer_partitions::Partitions;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

type ConditionRecord = (String, Vec<u32>);

fn is_possible(test: &str, pattern: &str) -> bool {
    test.chars()
        .zip(pattern.chars())
        .all(|(test, pattern)| pattern == '?' || test == pattern)
}

fn generate_test_string(good_counts: &[&usize], broken_counts: &[u32], len: usize) -> String {
    let mut test = String::with_capacity(len);

    for (index, good_count) in good_counts.iter().enumerate() {
        test.push_str(&".".repeat(**good_count));

        if index != 0 && index != good_counts.len() - 1 {
            test.push('.');
        }

        if let Some(bad_count) = broken_counts.get(index) {
            test.push_str(&"#".repeat(*bad_count as usize));
        }
    }

    test
}

fn count_possible_condition_records((conditions, broken_counts): &ConditionRecord) -> u32 {
    let num_gears = conditions.len() as u32;
    let num_broken_gears: u32 = broken_counts.iter().sum();
    let num_good_gears = num_gears - num_broken_gears;

    let num_moveable_gears = num_good_gears - (broken_counts.len() as u32 - 1);
    let num_moveable_gear_locations = broken_counts.len() as u32 + 1;

    let mut count: u32 = 0;
    let mut partitions = Partitions::new(num_moveable_gears as usize);

    while let Some(partition) = partitions.next() {
        if partition.len() as u32 > num_moveable_gear_locations {
            continue;
        }

        let mut partition = partition.to_vec();
        partition.resize(num_moveable_gear_locations as usize, 0);

        count += partition
            .iter()
            .permutations(partition.len())
            .unique()
            .map(|permutation| generate_test_string(&permutation, broken_counts, conditions.len()))
            .filter(|test_string| is_possible(test_string, conditions))
            .count() as u32;
    }

    count
}

fn parse_conditions(input: &str) -> IResult<&str, String> {
    let (input, conditions) = many1(alt((tag("."), tag("#"), tag("?"))))(input)?;

    Ok((input, conditions.into_iter().collect()))
}

fn parse_broken_counts(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), nom::character::complete::u32)(input)
}

fn parse_row(input: &str) -> IResult<&str, ConditionRecord> {
    separated_pair(parse_conditions, tag(" "), parse_broken_counts)(input)
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<ConditionRecord> {
    separated_list1(newline, parse_row)(input).unwrap().1
}

#[aoc(day12, part1)]
fn part1(records: &[ConditionRecord]) -> u32 {
    records.iter().map(count_possible_condition_records).sum()
}

#[aoc(day12, part2)]
fn part2(_records: &[ConditionRecord]) -> u32 {
    0
}

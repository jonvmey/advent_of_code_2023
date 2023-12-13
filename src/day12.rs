use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

type ConditionRecord = (String, Vec<u64>);

#[cached]
fn count_possible_condition_records(
    conditions: String,
    mut broken_counts: Vec<u64>,
    mut broken_seen: u64,
) -> u64 {
    if conditions.is_empty() {
        if broken_counts.is_empty() || broken_counts == [broken_seen] {
            return 1;
        } else {
            return 0;
        }
    }

    if broken_counts.is_empty() {
        if conditions.find('#').is_some() {
            return 0;
        } else {
            return 1;
        }
    }

    match conditions.chars().next().unwrap() {
        '.' => {
            if broken_seen > 0 {
                if broken_seen == *broken_counts.first().expect("checked not empty above") {
                    broken_counts.remove(0);
                    broken_seen = 0;
                } else {
                    return 0;
                }
            }

            count_possible_condition_records(
                conditions[1..].to_string(),
                broken_counts,
                broken_seen,
            )
        }
        '#' => {
            broken_seen += 1;

            if broken_seen > *broken_counts.first().expect("checked not empty above") {
                return 0;
            }

            count_possible_condition_records(
                conditions[1..].to_string(),
                broken_counts,
                broken_seen,
            )
        }
        '?' => {
            let mut good_conditions = conditions.clone();
            good_conditions.replace_range(0..1, ".");
            let mut broken_conditions = conditions;
            broken_conditions.replace_range(0..1, "#");

            count_possible_condition_records(good_conditions, broken_counts.clone(), broken_seen)
                + count_possible_condition_records(broken_conditions, broken_counts, broken_seen)
        }
        _ => panic!(),
    }
}

fn parse_conditions(input: &str) -> IResult<&str, String> {
    let (input, conditions) = many1(alt((tag("."), tag("#"), tag("?"))))(input)?;

    Ok((input, conditions.into_iter().collect()))
}

fn parse_broken_counts(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), nom::character::complete::u64)(input)
}

fn parse_row(input: &str) -> IResult<&str, ConditionRecord> {
    separated_pair(parse_conditions, tag(" "), parse_broken_counts)(input)
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<ConditionRecord> {
    // let input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
    separated_list1(newline, parse_row)(input).unwrap().1
}

#[aoc(day12, part1)]
fn part1(records: &[ConditionRecord]) -> u64 {
    records
        .iter()
        .map(|(conditions, broken_counts)| {
            count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0)
        })
        .sum()
}

#[aoc(day12, part2)]
fn part2(records: &[ConditionRecord]) -> u64 {
    const REPETITIONS: usize = 5;

    let records: Vec<ConditionRecord> = records
        .iter()
        .map(|(conditions, broken_counts)| {
            (
                conditions
                    .chars()
                    .chain("?".chars())
                    .cycle()
                    .take((conditions.len() + 1) * REPETITIONS - 1)
                    .collect(),
                broken_counts
                    .iter()
                    .cycle()
                    .take(broken_counts.len() * REPETITIONS)
                    .cloned()
                    .collect(),
            )
        })
        .collect();

    records
        .iter()
        .map(|(conditions, broken_counts)| {
            count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "???.### 1,1,3";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 1);
    }

    #[test]
    fn test2() {
        let input = ".??..??...?##. 1,1,3";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 4);
    }

    #[test]
    fn test3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 1);
    }

    #[test]
    fn test4() {
        let input = "????.#...#... 4,1,1";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 1);
    }

    #[test]
    fn test5() {
        let input = "????.######..#####. 1,6,5";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 4);
    }

    #[test]
    fn test6() {
        let input = "?###???????? 3,2,1";
        let (conditions, broken_counts) = &parse_input(input)[0];
        let count = count_possible_condition_records(conditions.clone(), broken_counts.clone(), 0);
        assert_eq!(count, 10);
    }
}

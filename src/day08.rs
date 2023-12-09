use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use num::integer::lcm;
use std::collections::HashMap;

type Maps = HashMap<String, (String, String)>;

fn parse_map(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)
}

fn parse_turns(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> (String, Maps) {
    let (input, turns) = parse_turns(input).unwrap();
    let turns = turns.to_string();
    let (_, maps) = preceded(many1(newline), separated_list1(newline, parse_map))(input).unwrap();
    let maps = maps
        .into_iter()
        .map(|(key, (left, right))| (key.to_string(), (left.to_string(), right.to_string())))
        .collect();

    (turns, maps)
}

#[aoc(day8, part1)]
fn part1(input: &(String, Maps)) -> u32 {
    let (turns, maps) = input;
    let mut steps = 0;
    let mut position = "AAA".to_string();
    let mut turn_iter = turns.chars().cycle();

    while position != "ZZZ" {
        let turn = turn_iter.next().expect("cycled iterator should never end");

        position = match turn {
            'L' => maps[&position].0.clone(),
            'R' => maps[&position].1.clone(),
            _ => panic!(),
        };

        steps += 1;
    }

    steps
}

#[aoc(day8, part2)]
fn part2(input: &(String, Maps)) -> u64 {
    let (turns, maps) = input;
    let mut positions: Vec<String> = maps
        .keys()
        .filter(|key| key.chars().nth(2).unwrap() == 'A')
        .cloned()
        .collect();
    let mut cycle_steps: Vec<u64> = vec![0; positions.len()];

    for (position, steps) in positions.iter_mut().zip(cycle_steps.iter_mut()) {
        let mut turn_iter = turns.chars().cycle();

        while position.chars().nth(2).unwrap() != 'Z' {
            let turn = turn_iter.next().expect("cycled iterator should never end");

            *position = match turn {
                'L' => maps[position].0.clone(),
                'R' => maps[position].1.clone(),
                _ => panic!(),
            };

            *steps += 1;
        }
    }

    cycle_steps.into_iter().fold(1, lcm)
}

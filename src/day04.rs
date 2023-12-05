use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1};
use nom::multi::{many1, many_till, separated_list1};
use nom::sequence::preceded;
use nom::IResult;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    selected_numbers: HashSet<u32>,
}

impl Card {
    fn matched_numbers(&self) -> HashSet<u32> {
        self.winning_numbers
            .intersection(&self.selected_numbers)
            .copied()
            .collect()
    }
}

fn whitespace_preceded_u32(input: &str) -> IResult<&str, u32> {
    preceded(space1, nom::character::complete::u32)(input)
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = whitespace_preceded_u32(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, (winning_numbers, _)) = many_till(whitespace_preceded_u32, tag(" |"))(input)?;
    let (input, selected_numbers) = many1(whitespace_preceded_u32)(input)?;

    let winning_numbers: HashSet<u32> = winning_numbers.into_iter().collect();
    let selected_numbers: HashSet<u32> = selected_numbers.into_iter().collect();

    Ok((
        input,
        Card {
            winning_numbers,
            selected_numbers,
        },
    ))
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Card> {
    let (_, cards) = separated_list1(newline, parse_line)(input).unwrap();

    cards
}

#[aoc(day4, part1)]
fn part1(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(|card| 2u32.pow(card.matched_numbers().len().try_into().unwrap()) / 2)
        .sum()
}

#[aoc(day4, part2)]
fn part2(cards: &[Card]) -> usize {
    let match_counts: Vec<usize> = cards
        .iter()
        .map(|card| card.matched_numbers().len())
        .collect();
    let mut card_counts = vec![1; match_counts.len()];

    for index in 0..match_counts.len() {
        let count = match_counts[index];

        for i in 0..count {
            card_counts[index + i + 1] += card_counts[index];
        }
    }

    card_counts.iter().sum()
}

use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{alphanumeric1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [u32; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<u32, u32> = HashMap::new();

        for card in self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<u32> = counts.values().cloned().collect();
        counts.sort();

        if counts == [5] {
            return HandType::FiveOfAKind;
        }
        if counts == [1, 4] {
            return HandType::FourOfAKind;
        }
        if counts == [2, 3] {
            return HandType::FullHouse;
        }
        if counts == [1, 1, 3] {
            return HandType::ThreeOfAKind;
        }
        if counts == [1, 2, 2] {
            return HandType::TwoPair;
        }
        if counts == [1, 1, 1, 2] {
            return HandType::OnePair;
        }
        if counts == [1, 1, 1, 1, 1] {
            return HandType::HighCard;
        }
        panic!();
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_type = self.hand_type();
        let right_type = other.hand_type();

        let type_cmp = left_type.cmp(&right_type);

        if type_cmp == Ordering::Equal {
            return Some(other.cards.cmp(&self.cards));
        }

        Some(type_cmp)
    }
}

#[derive(Debug, Copy, Clone)]
struct Game {
    hand: Hand,
    bid: u32,
}

fn card_number_from_char(c: char) -> u32 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

fn str_to_hand(input: &str) -> Hand {
    let mut cards = [0; 5];

    for (index, c) in input.chars().enumerate() {
        cards[index] = card_number_from_char(c);
    }

    Hand { cards }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, hand_str) = alphanumeric1(input)?;

    Ok((input, str_to_hand(hand_str)))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (hand, bid)) =
        separated_pair(parse_hand, space1, nom::character::complete::u32)(input)?;

    Ok((input, Game { hand, bid }))
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Game> {
    let (_, hands) = separated_list1(newline, parse_game)(input).unwrap();

    hands
}

#[aoc(day7, part1)]
fn part1(games: &[Game]) -> u32 {
    let mut games: Vec<Game> = games.iter().copied().collect();
    games.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    games
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, game)| acc + (index as u32 + 1) * game.bid)
}

#[aoc(day7, part2)]
fn part2(_input: &[Game]) -> u32 {
    0
}

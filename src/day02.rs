use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pull {
    fn new(colours: &[(Colour, u32)]) -> Pull {
        let mut pull = Pull {
            red: 0,
            green: 0,
            blue: 0,
        };

        for (colour, count) in colours {
            match colour {
                Colour::Red => pull.red += count,
                Colour::Green => pull.green += count,
                Colour::Blue => pull.blue += count,
            }
        }

        pull
    }
}

#[derive(Debug)]
struct Game {
    index: u32,
    pulls: Vec<Pull>,
}

fn parse_red_count(input: &str) -> IResult<&str, (Colour, u32)> {
    let (input, _) = tag(" ")(input)?;
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, colour) = tag("red")(input)?;

    if colour == "red" {
        return Ok((input, (Colour::Red, count)));
    }
    Err(nom::Err::Failure(nom::error::Error {
        input,
        code: nom::error::ErrorKind::Fail,
    }))
}

fn parse_green_count(input: &str) -> IResult<&str, (Colour, u32)> {
    let (input, _) = tag(" ")(input)?;
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, colour) = tag("green")(input)?;

    if colour == "green" {
        return Ok((input, (Colour::Green, count)));
    }
    Err(nom::Err::Failure(nom::error::Error {
        input,
        code: nom::error::ErrorKind::Fail,
    }))
}

fn parse_blue_count(input: &str) -> IResult<&str, (Colour, u32)> {
    let (input, _) = tag(" ")(input)?;
    let (input, count) = nom::character::complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, colour) = tag("blue")(input)?;

    if colour == "blue" {
        return Ok((input, (Colour::Blue, count)));
    }
    Err(nom::Err::Failure(nom::error::Error {
        input,
        code: nom::error::ErrorKind::Fail,
    }))
}

fn parse_pull_colours(input: &str) -> IResult<&str, Vec<(Colour, u32)>> {
    nom::multi::separated_list1(
        tag(","),
        nom::branch::alt((parse_red_count, parse_green_count, parse_blue_count)),
    )(input)
}

fn parse_pull(input: &str) -> IResult<&str, Pull> {
    let (input, colours) = parse_pull_colours(input)?;
    Ok((input, Pull::new(&colours)))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, index) =
        nom::sequence::delimited(tag("Game "), nom::character::complete::u32, tag(":"))(input)?;
    let (input, pulls) = nom::multi::separated_list1(tag(";"), parse_pull)(input)?;

    Ok((input, Game { index, pulls }))
}

fn input_parser(input: &str) -> IResult<&str, Vec<Game>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_line)(input)
}

fn possible_pull(pull: &Pull) -> bool {
    const NUM_RED: u32 = 12;
    const NUM_GREEN: u32 = 13;
    const NUM_BLUE: u32 = 14;

    pull.red <= NUM_RED && pull.green <= NUM_GREEN && pull.blue <= NUM_BLUE
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Game> {
    let games = if let Ok((_, games)) = input_parser(input) {
        games
    } else {
        todo!()
    };

    games
}

#[aoc(day2, part1)]
fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|game| game.pulls.iter().all(possible_pull))
        .map(|game| game.index)
        .sum()
}

#[aoc(day2, part2)]
fn part2(games: &[Game]) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut red_min = 0;
        let mut green_min = 0;
        let mut blue_min = 0;

        for pull in &game.pulls {
            red_min = std::cmp::max(red_min, pull.red);
            green_min = std::cmp::max(green_min, pull.green);
            blue_min = std::cmp::max(blue_min, pull.blue);
        }

        sum += red_min * green_min * blue_min;
    }

    sum
}

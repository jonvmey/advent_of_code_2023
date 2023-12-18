use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, hex_digit1, newline, space1};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use num::abs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

fn shoelace_formula(vertices: &[(i64, i64)]) -> u64 {
    let first_part: i64 = vertices
        .iter()
        .map(|(x, _)| x)
        .zip(vertices.iter().skip(1).map(|(_, y)| y))
        .map(|(x, y)| x * y)
        .sum();
    let second_part: i64 = vertices
        .iter()
        .skip(1)
        .map(|(x, _)| x)
        .zip(vertices.iter().map(|(_, y)| y))
        .map(|(x, y)| x * y)
        .sum();

    abs(first_part - second_part) as u64 / 2
}

fn loop_area(dig_plans: &[(Direction, u64)]) -> u64 {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut vertices = vec![(x, y)];

    let mut loop_length = 0;

    for (direction, distance) in dig_plans {
        let distance = *distance as i64;
        match direction {
            Direction::Up => y += distance,
            Direction::Down => y -= distance,
            Direction::Left => x -= distance,
            Direction::Right => x += distance,
        };

        vertices.push((x, y));
        loop_length += distance;
    }

    // Add loop_length / 2 + 1 to account for the shoelace vertices tracing the inside of the blocks
    // rather than the other edge
    shoelace_formula(&vertices) + (loop_length / 2) as u64 + 1
}

fn parse_line(input: &str) -> IResult<&str, (Direction, u64)> {
    // R 4 (#9505a2)
    let (input, (direction, distance)) =
        separated_pair(anychar, space1, nom::character::complete::u64)(input)?;
    let (input, _colour) = preceded(space1, delimited(tag("(#"), hex_digit1, tag(")")))(input)?;

    Ok((input, (Direction::from(direction), distance)))
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<(Direction, u64)> {
    let (_, lines) = separated_list1(newline, parse_line)(input).unwrap();

    lines
}

#[aoc(day18, part1)]
fn part1(dig_plans: &[(Direction, u64)]) -> u64 {
    loop_area(dig_plans)
}

#[aoc(day18, part2)]
fn part2(_input: &[(Direction, u64)]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );

    #[test]
    fn test1() {
        let dig_plans = parse_input(INPUT);

        assert_eq!(loop_area(&dig_plans), 62);
    }
}

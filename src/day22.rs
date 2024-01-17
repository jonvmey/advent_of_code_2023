use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn new_tuple((x, y, z): (u64, u64, u64)) -> Self {
        Self { x, y, z }
    }
}

// impl From<(u64, u64, u64)> for Point {
//     fn from((x, y, z): (u64, u64, u64)) -> Self {
//         Self::new(x, y, z)
//     }
// }

#[derive(Debug, Eq, PartialEq)]
struct Block {
    x_range: Range<u64>,
    y_range: Range<u64>,
    z_range: Range<u64>,
}

impl Block {
    fn new(first: Point, second: Point) -> Self {
        Self {
            x_range: first.x..second.x + 1,
            y_range: first.y..second.y + 1,
            z_range: first.z..second.z + 1,
        }
    }

    fn cubes(&self) -> Vec<Point> {
        let mut cubes = vec![];

        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                for z in self.z_range.clone() {
                    cubes.push(Point::new(x, y, z))
                }
            }
        }

        cubes
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison to turn BinaryHeap into a min-heap
        let z = other.z.cmp(&self.z);

        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, coords) = tuple((
        nom::character::complete::u64,
        preceded(tag(","), nom::character::complete::u64),
        preceded(tag(","), nom::character::complete::u64),
    ))(input)?;

    Ok((input, Point::new_tuple(coords)))
}

fn parse_line(input: &str) -> IResult<&str, Block> {
    let (input, (point1, point2)) = separated_pair(parse_point, tag("~"), parse_point)(input)?;

    Ok((input, Block::new(point1, point2)))
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<Block> {
    let (_, blocks) = separated_list1(newline, parse_line)(input).unwrap();

    blocks
}

#[aoc(day22, part1)]
fn part1(blocks: &[Block]) -> u64 {
    let block_heap = blocks

    0
}

#[aoc(day22, part2)]
fn part2(_input: &[Block]) -> u64 {
    0
}

#[cfg(test)]
mod tests {}

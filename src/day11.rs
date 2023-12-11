use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Range;

#[derive(Copy, Clone, Debug)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn distance(self, other: Point) -> usize {
        (max(self.row, other.row) - min(self.row, other.row))
            + (max(self.column, other.column) - min(self.column, other.column))
    }
}

fn missing_values(range: Range<usize>, present_values: HashSet<usize>) -> Vec<usize> {
    let mut missing = vec![];

    for i in range {
        if !present_values.contains(&i) {
            missing.push(i);
        }
    }

    missing
}

fn expand_universe(galaxies: &mut [Point], expansion_size: usize) {
    const NUM_ROWS: usize = 140;
    const NUM_COLUMNS: usize = 140;

    let rows_with_galaxies: HashSet<usize> = galaxies.iter().map(|point| point.row).collect();
    let rows_without_galaxies = missing_values(0..NUM_ROWS, rows_with_galaxies);

    let columns_with_galaxies: HashSet<usize> = galaxies.iter().map(|point| point.column).collect();
    let columns_without_galaxies = missing_values(0..NUM_COLUMNS, columns_with_galaxies);

    for galaxy in galaxies {
        galaxy.row += expansion_size * rows_without_galaxies
            .iter()
            .filter(|g| *g < &galaxy.row)
            .count();
        galaxy.column += expansion_size * columns_without_galaxies
            .iter()
            .filter(|g| *g < &galaxy.column)
            .count();
    }
}

fn generate_pairs(galaxies: &[Point]) -> Vec<(Point, Point)> {
    let mut pairs = vec![];

    for (i, galaxy_i) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let galaxy_j = galaxies[j];

            pairs.push((*galaxy_i, galaxy_j));
        }
    }

    pairs
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(column, c)| {
                if c == '#' {
                    Some(Point { row, column })
                } else {
                    None
                }
            })
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(galaxies: &[Point]) -> usize {
    let mut galaxies = galaxies.to_vec();

    expand_universe(&mut galaxies, 1);

    generate_pairs(&galaxies)
        .iter()
        .map(|(a, b)| a.distance(*b))
        .sum()
}

#[aoc(day11, part2)]
fn part2(galaxies: &[Point]) -> usize {
    let mut galaxies = galaxies.to_vec();

    expand_universe(&mut galaxies, 1000000 - 1);

    generate_pairs(&galaxies)
        .iter()
        .map(|(a, b)| a.distance(*b))
        .sum()
}

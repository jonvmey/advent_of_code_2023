use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};
use std::collections::HashMap;

fn is_symbol(c: char) -> bool {
    matches!(c, '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' | '/')
}

fn adjacent_symbol(grid: &Grid<char>, coord: &Coord) -> Option<Coord> {
    let test = Coord::new(coord.x - 1, coord.y - 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x, coord.y - 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x + 1, coord.y - 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x - 1, coord.y);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x + 1, coord.y);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x - 1, coord.y + 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x, coord.y + 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    let test = Coord::new(coord.x + 1, coord.y + 1);
    if is_symbol(*grid.get(test).unwrap_or(&'.')) {
        return Some(test);
    }

    None
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> grid_2d::Grid<char> {
    const WIDTH: u32 = 140;
    const HEIGHT: u32 = 140;

    Grid::new_iterator(
        Size::new(WIDTH, HEIGHT),
        input.chars().filter(|c| *c != '\n'),
    )
}

#[aoc(day3, part1)]
fn part1(grid: &Grid<char>) -> u32 {
    let mut seen_symbol = false;
    let mut number = String::new();
    let mut sum = 0;

    for (coord, cell) in grid.enumerate() {
        if cell.is_ascii_digit() {
            number.push(*cell);

            if adjacent_symbol(grid, &coord).is_some() {
                seen_symbol = true;
            }
        } else {
            if seen_symbol {
                sum += number.parse::<u32>().unwrap();
                seen_symbol = false;
            }

            number.clear();
        }
    }

    sum
}

#[aoc(day3, part2)]
fn part2(grid: &Grid<char>) -> u32 {
    let mut seen_symbol = false;
    let mut symbol_coord = Coord::new(0, 0);
    let mut number = String::new();
    let mut numbers_seen_with_symbols: Vec<(u32, Coord)> = vec![];

    for (coord, cell) in grid.enumerate() {
        if cell.is_ascii_digit() {
            number.push(*cell);

            if let Some(coord) = adjacent_symbol(grid, &coord) {
                seen_symbol = true;
                symbol_coord = coord;
            }
        } else {
            if seen_symbol && *grid.get(symbol_coord).unwrap() == '*' {
                numbers_seen_with_symbols.push((number.parse::<u32>().unwrap(), symbol_coord));
                seen_symbol = false;
            }

            number.clear();
        }
    }

    let mut numbers_around_coord_counts: HashMap<Coord, Vec<u32>> = HashMap::new();

    for (number, coord) in numbers_seen_with_symbols {
        numbers_around_coord_counts
            .entry(coord)
            .or_default()
            .push(number);
    }

    numbers_around_coord_counts
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .map(|(_, numbers)| numbers[0] * numbers[1])
        .sum()
}

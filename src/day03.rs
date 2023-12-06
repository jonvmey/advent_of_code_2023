use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};

fn is_symbol(c: char) -> bool {
    match c {
        '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' | '/' => true,
        _ => false,
    }
}

fn adjacent_symbol(grid: &Grid<char>, coord: &Coord) -> bool {
    is_symbol(
        *grid
            .get(Coord::new(coord.x - 1, coord.y - 1))
            .unwrap_or(&'.'),
    ) || is_symbol(*grid.get(Coord::new(coord.x, coord.y - 1)).unwrap_or(&'.'))
        || is_symbol(
            *grid
                .get(Coord::new(coord.x + 1, coord.y - 1))
                .unwrap_or(&'.'),
        )
        || is_symbol(*grid.get(Coord::new(coord.x - 1, coord.y)).unwrap_or(&'.'))
        || is_symbol(*grid.get(Coord::new(coord.x + 1, coord.y)).unwrap_or(&'.'))
        || is_symbol(
            *grid
                .get(Coord::new(coord.x - 1, coord.y + 1))
                .unwrap_or(&'.'),
        )
        || is_symbol(*grid.get(Coord::new(coord.x, coord.y + 1)).unwrap_or(&'.'))
        || is_symbol(
            *grid
                .get(Coord::new(coord.x + 1, coord.y + 1))
                .unwrap_or(&'.'),
        )
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
        if cell.is_digit(10) {
            number.push(*cell);

            if adjacent_symbol(&grid, &coord) {
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
fn part2(_grid: &Grid<char>) -> u32 {
    0
}

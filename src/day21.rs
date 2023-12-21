use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Plot,
    Rock,
    Start,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Plot,
            '#' => Tile::Rock,
            'S' => Tile::Start,
            _ => panic!("unexpected tile"),
        }
    }
}

fn get_neighbours(location: Coord) -> [Coord; 4] {
    [
        location + Coord::new(0, -1),
        location + Coord::new(0, 1),
        location + Coord::new(-1, 0),
        location + Coord::new(1, 0),
    ]
}

fn calculate_distances_from_start(grid: &Grid<Tile>) -> HashMap<Coord, u64> {
    let (start, _) = grid
        .enumerate()
        .find(|(_, tile)| **tile == Tile::Start)
        .expect("grid must contain start");
    let size = grid.size();

    let mut distances_from_start: HashMap<Coord, u64> = HashMap::from([(start, 0)]);
    let mut unvisited = HashSet::from([start]);

    while let Some(location) = unvisited.iter().copied().next() {
        unvisited.remove(&location);

        let current_distance = *distances_from_start.get(&location).unwrap();

        for neighbour in get_neighbours(location) {
            if neighbour.is_valid(size) && *grid.get(neighbour).unwrap() != Tile::Rock {
                let tentative_distance = current_distance + 1;
                if tentative_distance < *distances_from_start.entry(neighbour).or_insert(u64::MAX) {
                    distances_from_start.insert(neighbour, tentative_distance);
                    unvisited.insert(neighbour);
                }
            }
        }
    }

    distances_from_start
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Grid<Tile> {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().step_by(width + 1).count();

    Grid::new_iterator(
        Size::new(width as u32, height as u32),
        input
            .chars()
            .filter_map(|c| if c == '\n' { None } else { Some(Tile::from(c)) }),
    )
}

#[aoc(day21, part1)]
fn part1(grid: &Grid<Tile>) -> u64 {
    const STEPS: u64 = 64;

    let distances_from_start = calculate_distances_from_start(grid);

    distances_from_start
        .values()
        .filter(|distance| *distance <= &STEPS && (*distance % 2) == 0)
        .count() as u64
}

#[aoc(day21, part2)]
fn part2(_grid: &Grid<Tile>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "...........\n",
        ".....###.#.\n",
        ".###.##..#.\n",
        "..#.#...#..\n",
        "....#.#....\n",
        ".##..S####.\n",
        ".##..#...#.\n",
        ".......##..\n",
        ".##.#.####.\n",
        ".##..##.##.\n",
        "...........\n",
    );

    #[test]
    fn test1() {
        const STEPS: u64 = 6;
        let grid = parse_input(INPUT);
        let distances_from_start = calculate_distances_from_start(&grid);

        let count = distances_from_start
            .values()
            .filter(|distance| *distance <= &STEPS && (*distance % 2) == 0)
            .count();

        assert_eq!(count, 16);
    }
}

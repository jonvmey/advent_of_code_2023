use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};
use num::abs;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Straight,
    Turned,
}

fn get_potential_neighbours(location: Coord, came_from: Coord, can_continue_straight: bool) -> Vec<(Coord, Direction)> {
    let mut neighbours = vec![];

    let delta_x = location.x - came_from.x;
    let delta_y = location.y - came_from.y;

    if delta_y == 0 {
        neighbours.push((Coord::new(location.x, location.y+1), Direction::Turned));
        neighbours.push((Coord::new(location.x, location.y-1), Direction::Turned));

        if can_continue_straight {
            if delta_x == 1 { // came from west, continue east
                neighbours.push((Coord::new(location.x+1, location.y), Direction::Straight));
            }
            if delta_x == -1 { // came from east, continue west
                neighbours.push((Coord::new(location.x-1, location.y), Direction::Straight));
            }
        }
    }
    if delta_x == 0 {
        neighbours.push((Coord::new(location.x+1, location.y), Direction::Turned));
        neighbours.push((Coord::new(location.x-1, location.y), Direction::Turned));

        if can_continue_straight {
            if delta_y == 1 { // came from north, continue south
                neighbours.push((Coord::new(location.x, location.y+1), Direction::Straight));
            }
            if delta_y == -1 { // came from south, continue north
                neighbours.push((Coord::new(location.x, location.y-1), Direction::Straight));
            }
        }
    }

    neighbours
}

fn heuristic(location: Coord, destination: Coord) -> u32 {
    (abs(destination.x - location.x) as u32 + abs(destination.y - location.y) as u32) * 1
}

fn a_star(cost_map: &Grid<u32>, start: Coord, destination: Coord) -> u32 {

    let mut unvisited: HashSet<(Coord, Coord, u32)> = HashSet::new();
    unvisited.insert((start, start, 0));

    let mut f_scores : HashMap<(Coord, Coord, u32), u32> = HashMap::new();
    f_scores.insert((start, start, 0), 0);

    let mut g_scores : HashMap<(Coord, Coord, u32), u32> = HashMap::new();
    g_scores.insert((start, start, 0), 0);

    let mut came_from : HashMap<Coord, Coord> = HashMap::new();

    loop {
        let node = unvisited.iter().min_by_key(|key| {
            *f_scores.entry(**key).or_insert(u32::MAX)
        });

        if node.is_none() {
            panic!();
        }
        let node = *node.unwrap();

        unvisited.remove(&node);

        let (location, came_from_location, straight_line) = node;

        let current_g_score = *g_scores.get(&node).unwrap();

        if location == destination {
            break current_g_score;
        }

        for (neighbour_location, direction) in get_potential_neighbours(location, came_from_location, straight_line < 3) {
            if !neighbour_location.is_valid(cost_map.size()) {
                continue;
            }

            let straight_line_count = if direction == Direction::Turned { 1 } else { straight_line + 1 };
            let neighbour_node = (neighbour_location, location, straight_line_count);

            let tentative_g_score = current_g_score + cost_map.get(neighbour_location).unwrap();
            if tentative_g_score < *g_scores.entry(neighbour_node).or_insert(u32::MAX) {
                g_scores.insert(neighbour_node, tentative_g_score);
                f_scores.insert(neighbour_node, tentative_g_score + heuristic(location, destination));
                came_from.insert(neighbour_location, location);

                unvisited.insert(neighbour_node);
            }
        }
    }
}

fn clone<T: Copy>(grid: &Grid<T>) -> Grid<T> {
    Grid::new_iterator(grid.size(), grid.iter().copied())
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Grid<u32> {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.chars().step_by(width + 1).count();

    Grid::new_iterator(
        Size::new(width as u32, height as u32),
        input
            .chars()
            .filter_map(|c| if c == '\n' { None } else { c.to_digit(10) }),
    )
}

#[aoc(day17, part1)]
fn part1(grid: &Grid<u32>) -> u32 {
    let start = Coord::new(0, 0);
    let destination = Coord::new(grid.width() as i32 - 1, grid.height() as i32 -1);
    a_star(&grid, start, destination)
}

#[aoc(day17, part2)]
fn part2(_grid: &Grid<u32>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "2413432311323\n",
        "3215453535623\n",
        "3255245654254\n",
        "3446585845452\n",
        "4546657867536\n",
        "1438598798454\n",
        "4457876987766\n",
        "3637877979653\n",
        "4654967986887\n",
        "4564679986453\n",
        "1224686865563\n",
        "2546548887735\n",
        "4322674655533\n",
    );

    #[test]
    fn test1() {
        let mut grid = parse_input(INPUT);
        let start = Coord::new(0, 0);
        let destination = Coord::new(grid.width() as i32 - 1, grid.height() as i32 -1);

        let heat_loss = a_star(&grid, start, destination);

        assert_eq!(heat_loss, 102);
    }
}

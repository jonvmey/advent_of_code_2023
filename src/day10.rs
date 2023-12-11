use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    NorthSouth, // |
    EastWest,   // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '|' => Tile::NorthSouth,
        '-' => Tile::EastWest,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => panic!(),
    }
}

fn adjacent_tile(location: Coord, direction: Direction) -> Coord {
    match direction {
        Direction::North => Coord::new(location.x, location.y - 1),
        Direction::East => Coord::new(location.x + 1, location.y),
        Direction::South => Coord::new(location.x, location.y + 1),
        Direction::West => Coord::new(location.x - 1, location.y),
    }
}

fn connected_directions(tile: Tile) -> [Direction; 2] {
    match tile {
        Tile::NorthSouth => [Direction::North, Direction::South],
        Tile::EastWest => [Direction::East, Direction::West],
        Tile::NorthEast => [Direction::North, Direction::East],
        Tile::NorthWest => [Direction::North, Direction::West],
        Tile::SouthWest => [Direction::South, Direction::West],
        Tile::SouthEast => [Direction::South, Direction::East],
        Tile::Start | Tile::Ground => panic!(),
    }
}

fn connected_tiles(grid: &Grid<(Tile, bool)>, tile: Coord) -> Vec<Coord> {
    connected_directions(grid.get(tile).unwrap().0)
        .iter()
        .map(|direction| adjacent_tile(tile, *direction))
        .collect()
}

fn find_loop(grid: &mut Grid<(Tile, bool)>, start: Coord) -> u32 {
    let mut loop_length = 0;
    let mut current_position = start;
    let mut previous_position = adjacent_tile(start, Direction::South);

    loop {
        grid.get_mut(current_position).unwrap().1 = true;

        let connected = connected_tiles(grid, current_position);

        let next_position = if connected[0] == previous_position {
            connected[1]
        } else {
            connected[0]
        };
        previous_position = current_position;
        current_position = next_position;

        loop_length += 1;

        if current_position == start {
            break;
        }
    }

    loop_length
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> (Grid<(Tile, bool)>, Coord) {
    const WIDTH: u32 = 140;
    const HEIGHT: u32 = 140;

    let mut grid = Grid::new_iterator(
        Size::new(WIDTH, HEIGHT),
        input
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| (char_to_tile(c), false)),
    );

    let (start, _) = grid
        .enumerate()
        .find(|(_, (tile, _))| *tile == Tile::Start)
        .unwrap();
    grid.get_mut(start).unwrap().0 = Tile::SouthWest;

    (grid, start)
}

#[aoc(day10, part1)]
fn part1((grid, start): &(Grid<(Tile, bool)>, Coord)) -> u32 {
    let mut grid = Grid::new_iterator(grid.size(), grid.iter().copied());

    find_loop(&mut grid, *start) / 2
}

#[aoc(day10, part2)]
fn part2((grid, start): &(Grid<(Tile, bool)>, Coord)) -> u32 {
    let mut grid = Grid::new_iterator(grid.size(), grid.iter().copied());

    find_loop(&mut grid, *start);

    let mut inside_loop = false;
    let mut inside_count = 0;

    for (tile, part_of_loop) in grid.iter() {
        if *part_of_loop {
            match tile {
                Tile::NorthSouth | Tile::NorthWest | Tile::NorthEast => inside_loop = !inside_loop,
                _ => (),
            }
        } else if inside_loop {
            inside_count += 1;
        }

    }

    inside_count
}

use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Coord, Grid, Size};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    West,
    North,
    East,
    South,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TileType {
    Empty,
    AscendingMirror,
    DescendingMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for TileType {
    fn from(c: char) -> Self {
        match c {
            '.' => TileType::Empty,
            '/' => TileType::AscendingMirror,
            '\\' => TileType::DescendingMirror,
            '-' => TileType::HorizontalSplitter,
            '|' => TileType::VerticalSplitter,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    tile_type: TileType,
    visited_west: bool,
    visited_north: bool,
    visited_east: bool,
    visited_south: bool,
}

impl Tile {
    fn visited(&self) -> bool {
        self.visited_west || self.visited_north || self.visited_east || self.visited_south
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        Tile {
            tile_type: TileType::from(c),
            visited_west: false,
            visited_north: false,
            visited_east: false,
            visited_south: false,
        }
    }
}

fn track_beam(grid: &mut Grid<Tile>, location: Coord, came_from: Direction) {
    if let Some(tile) = grid.get_mut(location) {
        match came_from {
            Direction::West => {
                if tile.visited_west {
                    return;
                }
                tile.visited_west = true;

                match tile.tile_type {
                    TileType::AscendingMirror => {
                        tile.visited_north = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y - 1),
                            Direction::South,
                        );
                    }
                    TileType::DescendingMirror => {
                        tile.visited_south = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y + 1),
                            Direction::North,
                        );
                    }
                    TileType::HorizontalSplitter | TileType::Empty => {
                        tile.visited_east = true;
                        track_beam(
                            grid,
                            Coord::new(location.x + 1, location.y),
                            Direction::West,
                        );
                    }
                    TileType::VerticalSplitter => {
                        tile.visited_north = true;
                        tile.visited_south = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y - 1),
                            Direction::South,
                        );
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y + 1),
                            Direction::North,
                        );
                    }
                }
            }
            Direction::North => {
                if tile.visited_north {
                    return;
                }
                tile.visited_north = true;

                match tile.tile_type {
                    TileType::AscendingMirror => {
                        tile.visited_west = true;
                        track_beam(
                            grid,
                            Coord::new(location.x - 1, location.y),
                            Direction::East,
                        );
                    }
                    TileType::DescendingMirror => {
                        tile.visited_east = true;
                        track_beam(
                            grid,
                            Coord::new(location.x + 1, location.y),
                            Direction::West,
                        );
                    }
                    TileType::VerticalSplitter | TileType::Empty => {
                        tile.visited_south = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y + 1),
                            Direction::North,
                        );
                    }
                    TileType::HorizontalSplitter => {
                        tile.visited_west = true;
                        tile.visited_east = true;
                        track_beam(
                            grid,
                            Coord::new(location.x - 1, location.y),
                            Direction::East,
                        );
                        track_beam(
                            grid,
                            Coord::new(location.x + 1, location.y),
                            Direction::West,
                        );
                    }
                }
            }
            Direction::East => {
                if tile.visited_east {
                    return;
                }
                tile.visited_east = true;

                match tile.tile_type {
                    TileType::AscendingMirror => {
                        tile.visited_south = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y + 1),
                            Direction::North,
                        );
                    }
                    TileType::DescendingMirror => {
                        tile.visited_north = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y - 1),
                            Direction::South,
                        );
                    }
                    TileType::HorizontalSplitter | TileType::Empty => {
                        tile.visited_west = true;
                        track_beam(
                            grid,
                            Coord::new(location.x - 1, location.y),
                            Direction::East,
                        );
                    }
                    TileType::VerticalSplitter => {
                        tile.visited_north = true;
                        tile.visited_south = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y - 1),
                            Direction::South,
                        );
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y + 1),
                            Direction::North,
                        );
                    }
                }
            }
            Direction::South => {
                if tile.visited_south {
                    return;
                }
                tile.visited_south = true;

                match tile.tile_type {
                    TileType::AscendingMirror => {
                        tile.visited_east = true;
                        track_beam(
                            grid,
                            Coord::new(location.x + 1, location.y),
                            Direction::West,
                        );
                    }
                    TileType::DescendingMirror => {
                        tile.visited_west = true;
                        track_beam(
                            grid,
                            Coord::new(location.x - 1, location.y),
                            Direction::East,
                        );
                    }
                    TileType::VerticalSplitter | TileType::Empty => {
                        tile.visited_north = true;
                        track_beam(
                            grid,
                            Coord::new(location.x, location.y - 1),
                            Direction::South,
                        );
                    }
                    TileType::HorizontalSplitter => {
                        tile.visited_west = true;
                        tile.visited_east = true;
                        track_beam(
                            grid,
                            Coord::new(location.x - 1, location.y),
                            Direction::East,
                        );
                        track_beam(
                            grid,
                            Coord::new(location.x + 1, location.y),
                            Direction::West,
                        );
                    }
                }
            }
        }
    }
}

fn count_energized_tiles(grid: &Grid<Tile>) -> usize {
    grid.iter().filter(|tile| tile.visited()).count()
}

fn test_beam_origin(grid: &Grid<Tile>, location: Coord, source_direction: Direction) -> usize {
    let mut grid = clone(grid);
    track_beam(&mut grid, location, source_direction);

    count_energized_tiles(&grid)
}

#[aoc_generator(day16)]
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

fn clone(grid: &Grid<Tile>) -> Grid<Tile> {
    Grid::new_iterator(grid.size(), grid.iter().copied())
}

#[aoc(day16, part1)]
fn part1(grid: &Grid<Tile>) -> usize {
    test_beam_origin(grid, Coord::new(0, 0), Direction::West)
}

#[aoc(day16, part2)]
fn part2(grid: &Grid<Tile>) -> usize {
    let width = grid.width() as i32;
    let height = grid.height() as i32;

    let max_from_north = (0..width).map(|index| test_beam_origin(grid, Coord::new(index, 0), Direction::North)).max();
    let max_from_south = (0..width).map(|index| test_beam_origin(grid, Coord::new(index, height - 1), Direction::South)).max();
    let max_from_west = (0..width).map(|index| test_beam_origin(grid, Coord::new(0, index), Direction::West)).max();
    let max_from_east = (0..width).map(|index| test_beam_origin(grid, Coord::new(width - 1, index), Direction::East)).max();

    [max_from_north, max_from_south, max_from_west, max_from_east].iter().max().unwrap().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );

    #[test]
    fn test1() {
        let mut grid = parse_input(INPUT);
        track_beam(&mut grid, Coord::new(0, 0), Direction::West);

        assert_eq!(count_energized_tiles(&grid), 46);
    }
}

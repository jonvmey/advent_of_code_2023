use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Grid, Size};
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
enum Tile {
    Round,
    Cube,
    Empty,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Cube,
            '.' => Tile::Empty,
            _ => panic!(),
        }
    }
}

fn clone(grid: &Grid<Tile>) -> Grid<Tile> {
    Grid::new_iterator(grid.size(), grid.iter().copied())
}

fn vec_from_summary(summary: &[(usize, usize, usize)]) -> Vec<Tile> {
    let mut v = vec![];

    for (cube_count, round_count, empty_count) in summary {
        v.resize(v.len() + cube_count, Tile::Cube);
        v.resize(v.len() + round_count, Tile::Round);
        v.resize(v.len() + empty_count, Tile::Empty);
    }

    v
}

fn tilt<'a>(
    src: impl std::iter::Iterator<Item = &'a Tile>,
    dest: impl std::iter::Iterator<Item = &'a mut Tile>,
) {
    let summary = src.fold(vec![(0, 0, 0)], |mut acc, tile| {
        match tile {
            Tile::Round => {
                let (_, round_count, _) = acc.last_mut().unwrap();

                *round_count += 1;
            }
            Tile::Cube => {
                let (cube_count, round_count, empty_count) = acc.last_mut().unwrap();

                if *round_count != 0 || *empty_count != 0 {
                    acc.push((1, 0, 0));
                } else {
                    *cube_count += 1;
                }
            }
            Tile::Empty => {
                let (_, _, empty_count) = acc.last_mut().unwrap();

                *empty_count += 1;
            }
        }

        acc
    });

    let new_row = vec_from_summary(&summary);

    dest.zip(new_row.iter()).for_each(|(old, new)| *old = *new);
}

fn tilt_north(grid: &Grid<Tile>) -> Grid<Tile> {
    let width: usize = grid.width().try_into().unwrap();
    let mut tilted = Grid::new_copy(grid.size(), Tile::Empty);

    for column_index in 0..width {
        tilt(
            grid.iter().skip(column_index).step_by(width),
            tilted.iter_mut().skip(column_index).step_by(width),
        );
    }

    tilted
}

fn tilt_west(grid: &Grid<Tile>) -> Grid<Tile> {
    let mut tilted = Grid::new_copy(grid.size(), Tile::Empty);

    for (grid_row, tilted_row) in grid.rows().zip(tilted.rows_mut()) {
        tilt(grid_row.iter(), tilted_row.iter_mut());
    }

    tilted
}

fn tilt_south(grid: &Grid<Tile>) -> Grid<Tile> {
    let width: usize = grid.width().try_into().unwrap();
    let mut tilted = Grid::new_copy(grid.size(), Tile::Empty);

    for column_index in 0..width {
        tilt(
            grid.iter().rev().skip(column_index).step_by(width),
            tilted.iter_mut().rev().skip(column_index).step_by(width),
        );
    }

    tilted
}

fn tilt_east(grid: &Grid<Tile>) -> Grid<Tile> {
    let mut tilted = Grid::new_copy(grid.size(), Tile::Empty);

    for (grid_row, tilted_row) in grid.rows().zip(tilted.rows_mut()) {
        tilt(grid_row.iter().rev(), tilted_row.iter_mut().rev());
    }

    tilted
}

fn spin_cycle(grid: &Grid<Tile>) -> Grid<Tile> {
    tilt_east(&tilt_south(&tilt_west(&tilt_north(grid))))
}

fn run_cycles(mut grid: Grid<Tile>, cycle_count: usize) -> Grid<Tile> {
    let mut grids_seen: HashMap<Grid<Tile>, usize> = HashMap::new();

    for index in 0..cycle_count {
        let new_grid = spin_cycle(&grid);

        if let Some(seen_index) = grids_seen.get(&new_grid) {
            let repeat_length = index - seen_index;
            let final_repeat_index = (cycle_count - index - 1) % repeat_length;
            let final_cycle_index = final_repeat_index + seen_index;

            return grids_seen
                .into_iter()
                .find(|(_, count)| *count == final_cycle_index)
                .unwrap()
                .0;
        }

        grid = clone(&new_grid);
        grids_seen.insert(new_grid, index);
    }

    grid
}

fn calculate_load(grid: &Grid<Tile>) -> usize {
    grid.rows()
        .rev()
        .enumerate()
        .map(|(index, row)| row.iter().filter(|tile| *tile == &Tile::Round).count() * (index + 1))
        .sum()
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Grid<Tile> {
    let width = input.chars().position(|c| c == '\n').unwrap() as u32;
    let height = input.chars().step_by(width as usize + 1).count() as u32;

    Grid::new_iterator(
        Size::new(width, height),
        input
            .chars()
            .filter_map(|c| if c == '\n' { None } else { Some(Tile::from(c)) }),
    )
}

#[aoc(day14, part1)]
fn part1(grid: &Grid<Tile>) -> usize {
    let tilted = tilt_north(grid);
    calculate_load(&tilted)
}

#[aoc(day14, part2)]
fn part2(grid: &Grid<Tile>) -> usize {
    let grid = clone(grid);

    calculate_load(&run_cycles(grid, 1000000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );

    static TILTED: &str = concat!(
        "OOOO.#.O..\n",
        "OO..#....#\n",
        "OO..O##..O\n",
        "O..#.OO...\n",
        "........#.\n",
        "..#....#.#\n",
        "..O..#.O.O\n",
        "..O.......\n",
        "#....###..\n",
        "#....#....\n",
    );

    static CYCLED: &str = concat!(
        ".....#....\n",
        "....#...O#\n",
        "...OO##...\n",
        ".OO#......\n",
        ".....OOO#.\n",
        ".O#...O#.#\n",
        "....O#....\n",
        "......OOOO\n",
        "#...O###..\n",
        "#..OO#....\n",
    );

    #[test]
    fn test1() {
        let grid = parse_input(INPUT);

        assert_eq!(tilt_north(&grid), parse_input(TILTED));
    }

    #[test]
    fn test2() {
        let tilted = parse_input(TILTED);

        assert_eq!(calculate_load(&tilted), 136);
    }

    #[test]
    fn test3() {
        let grid = parse_input(INPUT);

        assert_eq!(spin_cycle(&grid), parse_input(CYCLED));
    }

    #[test]
    fn test4() {
        let grid = parse_input(INPUT);

        assert_eq!(calculate_load(&run_cycles(grid, 1000000000)), 64);
    }
}

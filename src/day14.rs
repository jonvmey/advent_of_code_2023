use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Grid, Size};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

fn vec_from_summary(summary: &[(usize, usize, usize)]) -> Vec<Tile> {
    let mut v = vec![];

    for (cube_count, round_count, empty_count) in summary {
        v.resize(v.len() + cube_count, Tile::Cube);
        v.resize(v.len() + round_count, Tile::Round);
        v.resize(v.len() + empty_count, Tile::Empty);
    }

    v
}

fn tilt_north(grid: &Grid<Tile>) -> Grid<Tile> {
    let width: usize = grid.width().try_into().unwrap();
    let mut tilted = Grid::new_iterator(grid.size(), grid.iter().copied());

    for column_index in 0..width {
        let column_summary: Vec<(usize, usize, usize)> = grid
            .iter()
            .skip(column_index)
            .step_by(width)
            .fold(vec![(0, 0, 0)], |mut acc, tile| {
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

        let new_column = vec_from_summary(&column_summary);

        tilted
            .iter_mut()
            .skip(column_index)
            .step_by(width)
            .zip(new_column.iter())
            .for_each(|(old, new)| *old = *new);
    }

    tilted
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
    let tilted = tilt_north(&grid);
    calculate_load(&tilted)
}

#[aoc(day14, part2)]
fn part2(_grid: &Grid<Tile>) -> usize {
    0
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
        //0123456789
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
}

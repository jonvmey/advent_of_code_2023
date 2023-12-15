use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Grid, Size};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::IResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Terrain {
    Ash,
    Rock,
}

impl From<char> for Terrain {
    fn from(c: char) -> Self {
        match c {
            '.' => Terrain::Ash,
            '#' => Terrain::Rock,
            _ => panic!(),
        }
    }
}

fn calculate_row_inaccuracies(row: &[Terrain]) -> Vec<usize> {
    (1..row.len())
        .map(|index| {
            row[..index]
                .iter()
                .rev()
                .zip(row[index..].iter())
                .filter(|(a, b)| a != b)
                .count()
        })
        .collect()
}

fn calculate_horizontal_inaccuracies(grid: &Grid<Terrain>) -> Vec<usize> {
    let width = grid.width() as usize;

    grid.rows()
        .map(calculate_row_inaccuracies)
        .fold(vec![0; width - 1], |mut acc, v| {
            acc.iter_mut()
                .zip(v.iter())
                .for_each(|(acc, elem)| *acc += elem);
            acc
        })
}

fn calculate_vertical_inaccuracies(grid: &Grid<Terrain>) -> Vec<usize> {
    let width = grid.width() as usize;
    let height = grid.height() as usize;

    (0..width)
        .map(|column_index| {
            let column: Vec<Terrain> = grid
                .iter()
                .skip(column_index)
                .step_by(width)
                .copied()
                .collect();

            calculate_row_inaccuracies(&column)
        })
        .fold(vec![0; height - 1], |mut acc, v| {
            acc.iter_mut()
                .zip(v.iter())
                .for_each(|(acc, elem)| *acc += elem);
            acc
        })
}

fn calculate_inaccuracies(grid: &Grid<Terrain>) -> Vec<(usize, Axis, usize)> {
    let mut inaccuracies = vec![];

    for (index, count) in calculate_horizontal_inaccuracies(grid)
        .into_iter()
        .enumerate()
    {
        inaccuracies.push((index + 1, Axis::Vertical, count));
    }
    for (index, count) in calculate_vertical_inaccuracies(grid)
        .into_iter()
        .enumerate()
    {
        inaccuracies.push((index + 1, Axis::Horizontal, count));
    }

    inaccuracies
}

fn find_valid_mirror(grid: &Grid<Terrain>) -> (usize, Axis) {
    let mirror: Vec<(usize, Axis)> = calculate_inaccuracies(grid)
        .into_iter()
        .filter_map(|(index, axis, count)| {
            if count == 0 {
                Some((index, axis))
            } else {
                None
            }
        })
        .collect();

    if mirror.len() != 1 {
        panic!();
    }

    mirror[0]
}

fn parse_grid_line(input: &str) -> IResult<&str, Vec<Terrain>> {
    let (input, cells) = many1(alt((tag("."), tag("#"))))(input)?;

    Ok((
        input,
        cells
            .into_iter()
            .map(|c| Terrain::from(c.chars().next().unwrap()))
            .collect(),
    ))
}

fn parse_grid(input: &str) -> IResult<&str, Grid<Terrain>> {
    let (input, lines) = separated_list1(newline, parse_grid_line)(input)?;

    Ok((
        input,
        Grid::new_iterator(
            Size::new(lines.first().unwrap().len() as u32, lines.len() as u32),
            lines.into_iter().flatten(),
        ),
    ))
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Grid<Terrain>> {
    let (_, grids) = separated_list1(tag("\n\n"), parse_grid)(input).unwrap();

    grids
}

#[aoc(day13, part1)]
fn part1(grids: &[Grid<Terrain>]) -> usize {
    grids
        .iter()
        .map(|grid| match find_valid_mirror(grid) {
            (line, Axis::Vertical) => line,
            (line, Axis::Horizontal) => 100 * line,
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(_grids: &[Grid<Terrain>]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
        );
        let grid = parse_input(input);
        let (mirror_line, axis) = find_valid_mirror(&grid[0]);
        assert_eq!(mirror_line, 5);
        assert_eq!(axis, Axis::Vertical);
    }

    #[test]
    fn test2() {
        let input = concat!(
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );
        let grid = parse_input(input);
        let (mirror_line, axis) = find_valid_mirror(&grid[0]);
        assert_eq!(mirror_line, 4);
        assert_eq!(axis, Axis::Horizontal);
    }
}

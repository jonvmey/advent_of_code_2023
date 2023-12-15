use aoc_runner_derive::{aoc, aoc_generator};
use grid_2d::{Grid, Size};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::IResult;
use std::collections::HashSet;

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

fn find_mirror_points(input: &[Terrain]) -> Option<HashSet<usize>> {
    let points: HashSet<usize> = (1..input.len())
        .filter(|index| {
            input[..*index]
                .iter()
                .rev()
                .zip(input[*index..].iter())
                .all(|(a, b)| a == b)
        })
        .collect();

    if points.is_empty() {
        return None;
    }

    Some(points)
}

fn calculate_intersections(mut possible_mirror_points: Vec<HashSet<usize>>) -> Option<usize> {
    let (intersection, others) = possible_mirror_points.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }

    if intersection.len() == 1 {
        return Some(*intersection.iter().next().unwrap());
    }

    None
}

fn check_vertical_mirror(grid: &Grid<Terrain>) -> Option<usize> {
    let mut possible_mirror_point_list: Vec<HashSet<usize>> = vec![];

    for row in grid.rows() {
        possible_mirror_point_list.push(find_mirror_points(row)?);
    }

    calculate_intersections(possible_mirror_point_list)
}

fn check_horizontal_mirror(grid: &Grid<Terrain>) -> Option<usize> {
    let width = grid.width() as usize;
    let mut possible_mirror_point_list: Vec<HashSet<usize>> = vec![];

    for column_index in 0..width {
        let column: Vec<Terrain> = grid
            .iter()
            .skip(column_index)
            .step_by(width)
            .copied()
            .collect();

        possible_mirror_point_list.push(find_mirror_points(&column)?);
    }

    calculate_intersections(possible_mirror_point_list)
}

fn find_mirror_location(grid: &Grid<Terrain>) -> (usize, Axis) {
    if let Some(mirror_line) = check_vertical_mirror(grid) {
        return (mirror_line, Axis::Vertical);
    }
    if let Some(mirror_line) = check_horizontal_mirror(grid) {
        return (mirror_line, Axis::Horizontal);
    }

    panic!("could not find a mirroring");
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
        .map(|grid| match find_mirror_location(grid) {
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
        let (mirror_line, axis) = find_mirror_location(&grid[0]);
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
        let (mirror_line, axis) = find_mirror_location(&grid[0]);
        assert_eq!(mirror_line, 4);
        assert_eq!(axis, Axis::Horizontal);
    }
}

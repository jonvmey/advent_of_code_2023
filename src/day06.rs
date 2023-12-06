use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_times(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;

    separated_list1(space1, nom::character::complete::u32)(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("\nDistance:")(input)?;
    let (input, _) = space1(input)?;

    separated_list1(space1, nom::character::complete::u32)(input)
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Race> {
    let (input, times) = parse_times(input).unwrap();
    let (_, distances) = parse_distances(input).unwrap();

    times.into_iter().zip(distances.into_iter()).map(|(time, distance)| Race{time, distance}).collect()
}

#[aoc(day6, part1)]
fn part1(races: &[Race]) -> u32 {
    let mut total = 1;

    for race in races {
        let mut win_count = 0;

        for push_time in 1..race.time {
            let speed = push_time;
            let move_time = race.time - push_time;
            let distance = speed * move_time;

            if distance > race.distance {
                win_count += 1;
            }
        }

        total *= win_count;
    }

    total
}

#[aoc(day6, part2)]
fn part2(_input: &[Race]) -> u32 {
    0
}

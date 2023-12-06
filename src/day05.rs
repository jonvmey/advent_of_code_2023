use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use range_ext::intersect::{Intersect, IntersectionExt};
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: Range<u64>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

fn perform_mapping(inputs: &[Range<u64>], mappings: &[Mapping]) -> Vec<Range<u64>> {
    let mut outputs: Vec<Range<u64>> = vec![];

    for input in inputs {
        let mut remaining_input = input.start..input.end;
        let mut intersecting_mappings: Vec<Mapping> = mappings
            .iter()
            .filter(|mapping| input.does_intersect(&mapping.source))
            .map(|m| Mapping {
                destination: m.destination,
                source: m.source.start..m.source.end,
            })
            .collect();
        intersecting_mappings.sort_by(|a, b| a.source.start.cmp(&b.source.start));

        for mapping in intersecting_mappings {
            let source = mapping.source;
            let destination = mapping.destination;

            match remaining_input.intersect_ext(&source) {
                IntersectionExt::Empty | IntersectionExt::Less | IntersectionExt::Greater => {
                    panic!()
                }
                IntersectionExt::Same => {
                    let mapped_length = remaining_input.end - remaining_input.start;
                    outputs.push(destination..destination + mapped_length);
                    remaining_input = 0..0;
                }
                IntersectionExt::LessOverlap => {
                    let mapped_length = remaining_input.end - source.start;
                    outputs.push(remaining_input.start..source.start);
                    outputs.push(destination..destination + mapped_length);
                    remaining_input = 0..0;
                }
                IntersectionExt::Within => {
                    let offset = remaining_input.start - source.start;
                    let mapped_length = remaining_input.end - remaining_input.start;
                    outputs.push(destination + offset..destination + offset + mapped_length);
                    remaining_input = 0..0;
                }
                IntersectionExt::Over => {
                    let mapped_length = source.end - source.start;
                    outputs.push(remaining_input.start..source.start);
                    outputs.push(destination..destination + mapped_length);
                    remaining_input = source.end..remaining_input.end;
                }
                IntersectionExt::GreaterOverlap => {
                    let offset = remaining_input.start - source.start;
                    let mapped_length = source.end - remaining_input.start;
                    outputs.push(destination + offset..destination + offset + mapped_length);
                    remaining_input = source.end..remaining_input.end;
                }
            }
        }

        if !remaining_input.is_empty() {
            outputs.push(remaining_input.start..remaining_input.end);
        }
    }

    outputs
}

fn seeds_to_locations(almanac: &Almanac) -> Vec<Range<u64>> {
    perform_mapping(
        &perform_mapping(
            &perform_mapping(
                &perform_mapping(
                    &perform_mapping(
                        &perform_mapping(
                            &perform_mapping(&almanac.seeds, &almanac.seed_to_soil),
                            &almanac.soil_to_fertilizer,
                        ),
                        &almanac.fertilizer_to_water,
                    ),
                    &almanac.water_to_light,
                ),
                &almanac.light_to_temperature,
            ),
            &almanac.temperature_to_humidity,
        ),
        &almanac.humidity_to_location,
    )
}

fn parse_seeds1(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, seeds) = preceded(
        tag("seeds: "),
        separated_list1(tag(" "), nom::character::complete::u64),
    )(input)?;

    Ok((
        input,
        seeds.into_iter().map(|seed| seed..seed + 1).collect(),
    ))
}

fn parse_seeds2(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    preceded(
        tag("seeds: "),
        separated_list1(
            tag(" "),
            separated_pair(
                nom::character::complete::u64,
                tag(" "),
                nom::character::complete::u64,
            ),
        ),
    )(input)
}

fn parse_map_line(input: &str) -> IResult<&str, Mapping> {
    let (input, (destination, source, length)) = tuple((
        nom::character::complete::u64,
        preceded(tag(" "), nom::character::complete::u64),
        preceded(tag(" "), nom::character::complete::u64),
    ))(input)?;

    Ok((
        input,
        Mapping {
            destination,
            source: source..source + length,
        },
    ))
}

fn parse_seed_to_soil_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("seed-to-soil map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_soil_to_fertilizer_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("soil-to-fertilizer map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_fertilizer_to_water_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("fertilizer-to-water map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_water_to_light_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("water-to-light map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_light_to_temperature_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("light-to-temperature map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_temperature_to_humidity_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("temperature-to-humidity map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_humidity_to_location_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (input, _) = preceded(many1(newline), tag("humidity-to-location map:\n"))(input)?;
    let (input, mapping) = separated_list1(newline, parse_map_line)(input)?;

    Ok((input, mapping))
}

fn parse_input1(input: &str) -> Almanac {
    let (input, seeds) = parse_seeds1(input).unwrap();
    let (input, seed_to_soil) = parse_seed_to_soil_map(input).unwrap();
    let (input, soil_to_fertilizer) = parse_soil_to_fertilizer_map(input).unwrap();
    let (input, fertilizer_to_water) = parse_fertilizer_to_water_map(input).unwrap();
    let (input, water_to_light) = parse_water_to_light_map(input).unwrap();
    let (input, light_to_temperature) = parse_light_to_temperature_map(input).unwrap();
    let (input, temperature_to_humidity) = parse_temperature_to_humidity_map(input).unwrap();
    let (_, humidity_to_location) = parse_humidity_to_location_map(input).unwrap();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn parse_input2(input: &str) -> Almanac {
    let (input, seeds_ranges) = parse_seeds2(input).unwrap();
    let (input, seed_to_soil) = parse_seed_to_soil_map(input).unwrap();
    let (input, soil_to_fertilizer) = parse_soil_to_fertilizer_map(input).unwrap();
    let (input, fertilizer_to_water) = parse_fertilizer_to_water_map(input).unwrap();
    let (input, water_to_light) = parse_water_to_light_map(input).unwrap();
    let (input, light_to_temperature) = parse_light_to_temperature_map(input).unwrap();
    let (input, temperature_to_humidity) = parse_temperature_to_humidity_map(input).unwrap();
    let (_, humidity_to_location) = parse_humidity_to_location_map(input).unwrap();

    let seeds = seeds_ranges
        .into_iter()
        .map(|(start, length)| start..start + length)
        .collect();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    seeds_to_locations(&parse_input1(input))
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    seeds_to_locations(&parse_input2(input))
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

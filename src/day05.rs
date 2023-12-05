use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::multi::{many1, separated_list1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source: u64,
    length: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = perform_mapping(&self.seed_to_soil, seed);
        let fertilizer = perform_mapping(&self.soil_to_fertilizer, soil);
        let water = perform_mapping(&self.fertilizer_to_water, fertilizer);
        let light = perform_mapping(&self.water_to_light, water);
        let temperature = perform_mapping(&self.light_to_temperature, light);
        let humidity = perform_mapping(&self.temperature_to_humidity, temperature);
        let location = perform_mapping(&self.humidity_to_location, humidity);

        location
    }
}

fn perform_mapping(mappings: &[Mapping], input: u64) -> u64 {
    for mapping in mappings {
        if input >= mapping.source && input < mapping.source + mapping.length {
            let offset = input - mapping.source;
            return mapping.destination + offset;
        }
    }

    input
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("seeds: "),
        separated_list1(tag(" "), nom::character::complete::u64),
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
            source,
            length,
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

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Almanac {
    let (input, seeds) = parse_seeds(input).unwrap();
    let (input, seed_to_soil) = parse_seed_to_soil_map(input).unwrap();
    let (input, soil_to_fertilizer) = parse_soil_to_fertilizer_map(input).unwrap();
    let (input, fertilizer_to_water) = parse_fertilizer_to_water_map(input).unwrap();
    let (input, water_to_light) = parse_water_to_light_map(input).unwrap();
    let (input, light_to_temperature) = parse_light_to_temperature_map(input).unwrap();
    let (input, temperature_to_humidity) = parse_temperature_to_humidity_map(input).unwrap();
    let (input, humidity_to_location) = parse_humidity_to_location_map(input).unwrap();

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

#[aoc(day5, part1)]
fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_location(*seed))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(_input: &Almanac) -> u64 {
    0
}

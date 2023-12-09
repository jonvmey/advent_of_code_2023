use aoc_runner_derive::{aoc, aoc_generator};

fn process_line_part_1(line: &str) -> i32 {
    let first_digit = line.chars().find(|&c| c.is_ascii_digit()).unwrap().to_string();
    let last_digit = line
        .chars()
        .rev()
        .find(|&c| c.is_ascii_digit())
        .unwrap()
        .to_string();

    first_digit.parse::<i32>().unwrap() * 10 + last_digit.parse::<i32>().unwrap()
}

fn number_replace(line: &str) -> String {
    let mut line = line.to_string();

    line = line.replace("one", "o1e");
    line = line.replace("two", "t2o");
    line = line.replace("three", "th3ee");
    line = line.replace("four", "fo4r");
    line = line.replace("five", "fi5e");
    line = line.replace("six", "s6x");
    line = line.replace("seven", "se7en");
    line = line.replace("eight", "ei8ht");
    line = line.replace("nine", "ni9e");

    line
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(lines: &[String]) -> i32 {
    lines.iter().map(|line| process_line_part_1(line)).sum()
}

#[aoc(day1, part2)]
fn part2(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|line| process_line_part_1(&number_replace(line)))
        .sum()
}

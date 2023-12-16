use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::Wrapping;

type Lens = (String, u8);
type Step = (String, Operation, Option<u8>);

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Equals,
    Dash,
}

fn hash_u8(mut acc: Wrapping<u8>, c: u8) -> Wrapping<u8> {
    acc += c;
    acc *= 17;

    acc
}

fn hash_str(s: &str) -> u8 {
    s.bytes().fold(Wrapping(0u8), hash_u8).0
}

fn parse_steps(steps: &[String]) -> Vec<Step> {
    steps
        .iter()
        .map(|step| {
            if let Some(step) = step.strip_suffix('-') {
                (step.to_string(), Operation::Dash, None)
            } else {
                let mut split = step.split('=');
                let label = split.next().expect("label should always exist").to_string();
                let focal_length = split
                    .next()
                    .expect("equals op should always have focal length")
                    .parse::<u8>()
                    .unwrap();

                (label, Operation::Equals, Some(focal_length))
            }
        })
        .collect()
}

fn generate_boxes(steps: Vec<Step>) -> HashMap<u8, Vec<Lens>> {
    let mut box_map: HashMap<u8, Vec<Lens>> = HashMap::new();

    for (label, operation, focal_length) in steps {
        let box_index = hash_str(&label);
        let box_lenses = box_map.entry(box_index).or_default();

        if operation == Operation::Dash {
            if let Some(index) = box_lenses
                .iter()
                .position(|(lens_label, _)| *lens_label == label)
            {
                box_lenses.remove(index);
            }
        } else if operation == Operation::Equals {
            let focal_length = focal_length.expect("equals op should always have focal length");

            if let Some((_, lens_focal_length)) = box_lenses
                .iter_mut()
                .find(|(lens_label, _)| *lens_label == label)
            {
                *lens_focal_length = focal_length;
            } else {
                box_lenses.push((label, focal_length));
            }
        }
    }

    box_map
}

fn calculate_focusing_power(box_map: HashMap<u8, Vec<Lens>>) -> usize {
    box_map
        .into_iter()
        .map(|(box_index, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(lens_index, (_, focal_length))| {
                    (box_index as usize + 1) * (lens_index + 1) * focal_length as usize
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<String> {
    input.split(',').map(|s| s.to_string()).collect()
}

#[aoc(day15, part1)]
fn part1(steps: &[String]) -> usize {
    steps.iter().map(|step| hash_str(step) as usize).sum()
}

#[aoc(day15, part2)]
fn part2(steps: &[String]) -> usize {
    let steps = parse_steps(steps);

    let box_map = generate_boxes(steps);

    calculate_focusing_power(box_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test1() {
        assert_eq!(hash_str("HASH"), 52)
    }

    #[test]
    fn test2() {
        let steps = parse_input(INPUT);

        let sum: u32 = steps.into_iter().map(|step| hash_str(&step) as u32).sum();

        assert_eq!(sum, 1320)
    }

    #[test]
    fn test3() {
        let steps = parse_input(INPUT);
        let steps = parse_steps(&steps);
        let box_map = generate_boxes(steps);

        assert_eq!(calculate_focusing_power(box_map), 145);
    }
}

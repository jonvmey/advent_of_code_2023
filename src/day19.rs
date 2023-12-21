use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, anychar, newline};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::collections::HashMap;
use std::ops::Range;

type Workflow = (String, Vec<Rule>);

const ACCEPT: &str = "A";
const REJECT: &str = "R";

#[derive(Debug, Eq, PartialEq)]
enum Evaluation {
    Accept,
    Reject,
}

#[derive(Debug)]
enum Attribute {
    X,
    M,
    A,
    S,
}

impl From<char> for Attribute {
    fn from(c: char) -> Self {
        match c {
            'x' => Attribute::X,
            'm' => Attribute::M,
            'a' => Attribute::A,
            's' => Attribute::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Less,
    Greater,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '<' => Operator::Less,
            '>' => Operator::Greater,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    operation: Option<(Attribute, Operator, u64)>,
    destination: String,
}

impl Rule {
    fn new(destination: String) -> Self {
        Rule {
            operation: None,
            destination,
        }
    }

    fn new_with_operation(
        destination: String,
        attribute: Attribute,
        operator: Operator,
        value: u64,
    ) -> Self {
        Rule {
            operation: Some((attribute, operator, value)),
            destination,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(x: u64, m: u64, a: u64, s: u64) -> Self {
        Part { x, m, a, s }
    }

    fn matches_rule(&self, rule: &Rule) -> bool {
        if let Some((attribute, operator, value)) = &rule.operation {
            let attribute = match attribute {
                Attribute::X => self.x,
                Attribute::M => self.m,
                Attribute::A => self.a,
                Attribute::S => self.s,
            };

            match operator {
                Operator::Less => attribute < *value,
                Operator::Greater => attribute > *value,
            }
        } else {
            true
        }
    }

    fn rating(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn apply_workflows(part: &Part, workflows: &HashMap<String, Vec<Rule>>, start: &str) -> Evaluation {
    let mut workflow = start.to_string();

    loop {
        if workflow == ACCEPT {
            return Evaluation::Accept;
        } else if workflow == REJECT {
            return Evaluation::Reject;
        }

        let rules = workflows.get(&workflow).unwrap();

        for rule in rules {
            if part.matches_rule(rule) {
                workflow = rule.destination.clone();
                break;
            }
        }
    }
}

fn parse_rule_with_operation(input: &str) -> IResult<&str, Rule> {
    let (input, attribute) = anychar(input)?;
    let (input, operator) = anychar(input)?;
    let (input, value) = nom::character::complete::u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = alpha1(input)?;

    Ok((
        input,
        Rule::new_with_operation(
            destination.to_string(),
            Attribute::from(attribute),
            Operator::from(operator),
            value,
        ),
    ))
}

fn parse_rule_without_operation(input: &str) -> IResult<&str, Rule> {
    let (input, destination) = alpha1(input)?;

    Ok((input, Rule::new(destination.to_string())))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_rule_with_operation, parse_rule_without_operation))(input)
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = take_till(|c: char| c == '{')(input)?;
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;

    Ok((input, (name.to_string(), rules)))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = nom::character::complete::u64(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = nom::character::complete::u64(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = nom::character::complete::u64(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = nom::character::complete::u64(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, Part::new(x, m, a, s)))
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    let (input, workflows) = separated_list1(newline, parse_workflow)(input).unwrap();
    let (_, parts) = preceded(tag("\n\n"), separated_list1(newline, parse_part))(input).unwrap();

    let workflows: HashMap<String, Vec<Rule>> = workflows.into_iter().collect();
    (workflows, parts)
}

#[aoc(day19, part1)]
fn part1((workflows, parts): &(HashMap<String, Vec<Rule>>, Vec<Part>)) -> u64 {
    parts
        .iter()
        .filter(|part| apply_workflows(part, workflows, "in") == Evaluation::Accept)
        .map(|part| part.rating())
        .sum()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct PossibleParts {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PossibleParts {
    fn new(min: u64, max: u64) -> Self {
        PossibleParts {
            x: min..max + 1,
            m: min..max + 1,
            a: min..max + 1,
            s: min..max + 1,
        }
    }

    fn possibilities(&self) -> u64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    fn split_by_rule(self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        if let Some((attribute, operator, value)) = &rule.operation {
            let mut matched = self.clone();
            let mut unmatched = self.clone();
            let (split_attribute, matched_attribute, unmatched_attribute) = match attribute {
                Attribute::X => (self.x, &mut matched.x, &mut unmatched.x),
                Attribute::M => (self.m, &mut matched.m, &mut unmatched.m),
                Attribute::A => (self.a, &mut matched.a, &mut unmatched.a),
                Attribute::S => (self.s, &mut matched.s, &mut unmatched.s),
            };

            if split_attribute.contains(value) {
                match operator {
                    Operator::Less => {
                        *matched_attribute = split_attribute.start..*value;
                        *unmatched_attribute = *value..split_attribute.end;
                    }
                    Operator::Greater => {
                        *unmatched_attribute = split_attribute.start..*value + 1;
                        *matched_attribute = *value + 1..split_attribute.end;
                    }
                }

                (Some(matched), Some(unmatched))
            } else if (*operator == Operator::Less && *value > split_attribute.end)
                || (*operator == Operator::Greater && *value < split_attribute.start)
            {
                (Some(matched), None)
            } else {
                (None, Some(unmatched))
            }
        } else {
            (Some(self), None)
        }
    }
}

fn evaluate_parts(
    workflows: &HashMap<String, Vec<Rule>>,
    workflow: String,
    parts: PossibleParts,
) -> HashMap<PossibleParts, String> {
    let mut map: HashMap<PossibleParts, String> = HashMap::new();
    map.insert(parts, workflow);

    loop {
        let (mut parts, workflow) = {
            if let Some((parts, workflow)) = map
                .iter()
                .find(|(_, workflow)| *workflow != ACCEPT && **workflow != REJECT)
            {
                (parts.clone(), workflow.clone())
            } else {
                break;
            }
        };

        for rule in workflows.get(&workflow).unwrap() {
            match parts.clone().split_by_rule(rule) {
                (Some(matched), Some(unmatched)) => {
                    map.remove(&parts);
                    map.insert(matched, rule.destination.clone());
                    parts = unmatched;
                }
                (Some(matched), None) => {
                    map.insert(matched, rule.destination.clone());
                }
                (None, Some(unmatched)) => {
                    parts = unmatched;
                }
                _ => panic!(),
            }
        }
    }

    map
}

fn calculate_accepted_possibilities(map: &HashMap<PossibleParts, String>) -> u64 {
    map.iter()
        .filter(|(_, eval)| *eval == ACCEPT)
        .map(|(parts, _)| parts.possibilities())
        .sum()
}

#[aoc(day19, part2)]
fn part2((workflows, _): &(HashMap<String, Vec<Rule>>, Vec<Part>)) -> u64 {
    let parts = PossibleParts::new(1, 4000);
    let stuff = evaluate_parts(workflows, "in".to_string(), parts);
    calculate_accepted_possibilities(&stuff)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = concat!(
        "px{a<2006:qkq,m>2090:A,rfg}\n",
        "pv{a>1716:R,A}\n",
        "lnx{m>1548:A,A}\n",
        "rfg{s<537:gd,x>2440:R,A}\n",
        "qs{s>3448:A,lnx}\n",
        "qkq{x<1416:A,crn}\n",
        "crn{x>2662:A,R}\n",
        "in{s<1351:px,qqz}\n",
        "qqz{s>2770:qs,m<1801:hdj,R}\n",
        "gd{a>3333:R,R}\n",
        "hdj{m>838:A,pv}\n",
        "\n",
        "{x=787,m=2655,a=1222,s=2876}\n",
        "{x=1679,m=44,a=2067,s=496}\n",
        "{x=2036,m=264,a=79,s=2244}\n",
        "{x=2461,m=1339,a=466,s=291}\n",
        "{x=2127,m=1623,a=2188,s=1013}\n",
    );

    #[test]
    fn test1() {
        let (workflows, parts) = parse_input(INPUT);

        let sum: u64 = parts
            .iter()
            .filter(|part| apply_workflows(part, &workflows, &"in") == Evaluation::Accept)
            .map(|part| part.rating())
            .sum();

        assert_eq!(sum, 19114);
    }

    #[test]
    fn test2() {
        let (workflows, _) = parse_input(INPUT);

        let parts = PossibleParts::new(1, 4000);
        let stuff = evaluate_parts(&workflows, "in".to_string(), parts);
        let sum = calculate_accepted_possibilities(&stuff);

        assert_eq!(sum, 167409079868000);
    }
}

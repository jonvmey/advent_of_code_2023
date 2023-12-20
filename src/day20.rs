use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashMap;
use std::collections::VecDeque;

static BROADCASTER: &str = "broadcaster";
static BUTTON: &str = "button";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn toggle(&mut self) {
        *self = match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop { state: Pulse },
    Conjunction { inputs: HashMap<String, Pulse> },
    Broadcaster,
    Button,
}

impl ModuleType {
    fn process_pulse(&mut self, pulse: Pulse, source: String) -> Option<Pulse> {
        match self {
            ModuleType::FlipFlop { state } => {
                if pulse == Pulse::Low {
                    state.toggle();
                    Some(*state)
                } else {
                    None
                }
            }
            ModuleType::Conjunction { inputs } => {
                *inputs.get_mut(&source).unwrap() = pulse;
                if inputs.values().all(|state| *state == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::Button => Some(Pulse::Low),
        }
    }
}

#[derive(Debug)]
struct Module {
    module: ModuleType,
    outputs: Vec<String>,
    low_pulses_sent: u64,
    high_pulses_sent: u64,
}

impl Module {
    fn new_flip_flop(outputs: Vec<String>) -> Self {
        Module {
            module: ModuleType::FlipFlop { state: Pulse::Low },
            outputs,
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn new_conjunction(inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Module {
            module: ModuleType::Conjunction {
                inputs: inputs
                    .into_iter()
                    .map(|input| (input, Pulse::Low))
                    .collect(),
            },
            outputs,
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn new_broadcaster(outputs: Vec<String>) -> Self {
        Module {
            module: ModuleType::Broadcaster,
            outputs,
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn new_button() -> Self {
        Module {
            module: ModuleType::Button,
            outputs: vec![BROADCASTER.to_string()],
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn process_pulse(
        &mut self,
        pulse: Pulse,
        source: String,
        module_name: String,
        queue: &mut VecDeque<(Pulse, String, String)>,
    ) {
        if let Some(pulse) = self.module.process_pulse(pulse, source.clone()) {
            self.outputs.iter().for_each(|output| {
                match pulse {
                    Pulse::Low => self.low_pulses_sent += 1,
                    Pulse::High => self.high_pulses_sent += 1,
                }
                queue.push_back((pulse, module_name.clone(), output.clone()));
            });
        }
    }
}

#[derive(Debug)]
struct ModuleNetwork {
    modules: HashMap<String, Module>,
    pulse_queue: VecDeque<(Pulse, String, String)>,
}

impl ModuleNetwork {
    fn new(module_list: &[(String, String, Vec<String>)]) -> Self {
        let mut modules: HashMap<String, Module> = module_list
            .iter()
            .map(|(module_type, name, outputs)| {
                let module = match module_type.as_str() {
                    "%" => Module::new_flip_flop(outputs.clone()),
                    "&" => Module::new_conjunction(
                        inputs_from_list(module_list, name),
                        outputs.clone(),
                    ),
                    "broadcaster" => Module::new_broadcaster(outputs.clone()),
                    _ => panic!(),
                };

                (name.clone(), module)
            })
            .collect();
        modules.insert(BUTTON.to_string(), Module::new_button());

        ModuleNetwork {
            modules,
            pulse_queue: VecDeque::new(),
        }
    }

    fn press_button(&mut self) {
        self.modules
            .get_mut(BUTTON)
            .expect("button should be added during initializations")
            .process_pulse(
                Pulse::Low,
                "".to_string(),
                BUTTON.to_string(),
                &mut self.pulse_queue,
            );

        while let Some((pulse, source, destination)) = self.pulse_queue.pop_front() {
            if let Some(module) = self.modules.get_mut(&destination) {
                module.process_pulse(pulse, source, destination, &mut self.pulse_queue);
            }
        }
    }

    fn press_button_n(&mut self, n: u64) {
        for _ in 0..n {
            self.press_button();
        }
    }

    fn low_pulses_sent(&self) -> u64 {
        self.modules
            .values()
            .map(|module| module.low_pulses_sent)
            .sum()
    }

    fn high_pulses_sent(&self) -> u64 {
        self.modules
            .values()
            .map(|module| module.high_pulses_sent)
            .sum()
    }
}

fn inputs_from_list(module_list: &[(String, String, Vec<String>)], name: &str) -> Vec<String> {
    module_list
        .iter()
        .filter_map(|(_, module, outputs)| {
            if outputs.iter().any(|output| *output == name) {
                Some(module)
            } else {
                None
            }
        })
        .cloned()
        .collect()
}

fn parse_destinations(input: &str) -> IResult<&str, Vec<String>> {
    let (input, destinations) = separated_list1(tag(", "), alpha1)(input)?;

    Ok((input, destinations.iter().map(|s| s.to_string()).collect()))
}

fn parse_broadcaster(input: &str) -> IResult<&str, (String, String, Vec<String>)> {
    let (input, name) = tag("broadcaster")(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, destinations) = parse_destinations(input)?;

    Ok((input, (name.to_string(), name.to_string(), destinations)))
}

fn parse_ff_conj(input: &str) -> IResult<&str, (String, String, Vec<String>)> {
    let (input, module_type) = alt((tag("%"), tag("&")))(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, destinations) = parse_destinations(input)?;

    Ok((
        input,
        (module_type.to_string(), name.to_string(), destinations),
    ))
}

fn parse_line(input: &str) -> IResult<&str, (String, String, Vec<String>)> {
    alt((parse_ff_conj, parse_broadcaster))(input)
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Vec<(String, String, Vec<String>)> {
    let (_, lines) = separated_list1(newline, parse_line)(input).unwrap();

    lines
}

#[aoc(day20, part1)]
fn part1(modules: &[(String, String, Vec<String>)]) -> u64 {
    let mut network = ModuleNetwork::new(modules);
    network.press_button_n(1000);

    network.low_pulses_sent() * network.high_pulses_sent()
}

#[aoc(day20, part2)]
fn part2(_modules: &[(String, String, Vec<String>)]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = concat!(
        "broadcaster -> a, b, c\n",
        "%a -> b\n",
        "%b -> c\n",
        "%c -> inv\n",
        "&inv -> a\n",
    );

    static INPUT2: &str = concat!(
        "broadcaster -> a\n",
        "%a -> inv, con\n",
        "&inv -> b\n",
        "%b -> con\n",
        "&con -> output\n",
    );

    #[test]
    fn test1() {
        let modules = parse_input(INPUT1);
        let mut network = ModuleNetwork::new(&modules);
        network.press_button_n(1000);

        assert_eq!(
            network.low_pulses_sent() * network.high_pulses_sent(),
            32000000
        );
    }

    #[test]
    fn test2() {
        let modules = parse_input(INPUT2);
        let mut network = ModuleNetwork::new(&modules);
        network.press_button_n(1000);

        assert_eq!(
            network.low_pulses_sent() * network.high_pulses_sent(),
            11687500
        );
    }
}

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

struct FlipFlopModule<'a> {
    flip: bool,
    targets: Vec<&'a str>,
}

impl FlipFlopModule<'_> {
    fn process(&mut self, pulse_type: &Pulse) -> Option<Pulse> {
        match pulse_type {
            Pulse::High => None,
            Pulse::Low => {
                let next_pulse = if self.flip { Pulse::Low } else { Pulse::High };

                self.flip = !self.flip;
                Some(next_pulse)
            }
        }
    }
}

struct ConjunctionModule<'a> {
    prev: HashMap<&'a str, Pulse>,
    targets: Vec<&'a str>,
}

impl<'a> ConjunctionModule<'a> {
    fn process(&mut self, pulse_type: &Pulse, from: &'a str) -> Pulse {
        let new_prev = match pulse_type {
            Pulse::Low => Pulse::Low,
            Pulse::High => Pulse::High,
        };
        *self.prev.get_mut(&from).unwrap() = new_prev;

        if self
            .prev
            .iter()
            .all(|(_, prev_pulse)| matches!(prev_pulse, Pulse::High))
        {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

struct BroadcastModule<'a> {
    targets: Vec<&'a str>,
}

impl BroadcastModule<'_> {
    fn process(&self, pulse_type: &Pulse, _: &str) -> Pulse {
        match pulse_type {
            Pulse::Low => Pulse::Low,
            Pulse::High => Pulse::High,
        }
    }
}

enum Module<'a> {
    FlipFlop(FlipFlopModule<'a>),
    Conjunction(ConjunctionModule<'a>),
    Broadcast(BroadcastModule<'a>),
}

struct Configuration<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Configuration<'a> {
    fn process(
        &mut self,
        pulse: &Pulse,
        (from, to): (&'a str, &'a str),
        queue: &mut VecDeque<(Pulse, (&'a str, &'a str))>,
    ) {
        if let Some(wrapped_module) = self.modules.get_mut(to) {
            match wrapped_module {
                Module::Broadcast(broadcaster) => {
                    for target in &broadcaster.targets {
                        let new_pulse = broadcaster.process(pulse, from);
                        queue.push_back((new_pulse, ("broadcaster", target)));
                    }
                }
                Module::FlipFlop(flip_flop) => {
                    if let Some(next_pulse) = flip_flop.process(pulse) {
                        for target in &flip_flop.targets {
                            queue.push_back((next_pulse.clone(), (to, target)));
                        }
                    }
                }
                Module::Conjunction(conjuntion) => {
                    let next_pulse = conjuntion.process(pulse, from);
                    for target in &conjuntion.targets {
                        queue.push_back((next_pulse.clone(), (to, target)))
                    }
                }
            }
        }
    }
}

const PART1_PRESSES: usize = 1000;

fn main() {
    let input = Path::new("2023/day20/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let mut configuration = parse_configuration(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Multiplied pulses after 1000 cycles: {:?}",
        pulse(&mut configuration, PART1_PRESSES)
    );
    println!("In {:?}", part1.elapsed());

    let mut configuration = parse_configuration(&contents);
    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fewest number of button presses to start machine: {:?}",
        button_presses_to_start(&mut configuration, "kz")
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_configuration(input: &str) -> Configuration<'_> {
    let mut modules = HashMap::new();
    let mut conjunction_modules = Vec::new();
    input.lines().for_each(|line| {
        let (module, targets) = line.split_once(" -> ").unwrap();

        let targets: Vec<_> = targets.split(", ").collect();

        if module == "broadcaster" {
            modules.insert(module, Module::Broadcast(BroadcastModule { targets }));
        } else if module.starts_with("%") {
            modules.insert(
                module.trim_start_matches("%"),
                Module::FlipFlop(FlipFlopModule {
                    flip: false,
                    targets,
                }),
            );
        } else if module.starts_with("&") {
            let name = module.trim_start_matches("&");
            conjunction_modules.push(name);
            modules.insert(
                name,
                Module::Conjunction(ConjunctionModule {
                    prev: HashMap::new(),
                    targets,
                }),
            );
        }
    });

    let mut module_to_inputs = HashMap::new();

    for name in conjunction_modules {
        let mut inputs = Vec::new();
        for (other_name, module) in &modules {
            let is_input = match module {
                Module::Broadcast(broadcast) => broadcast.targets.contains(&name),
                Module::Conjunction(conjunction) => conjunction.targets.contains(&name),
                Module::FlipFlop(flip_flop) => flip_flop.targets.contains(&name),
            };
            if is_input {
                inputs.push(*other_name);
            }
        }
        module_to_inputs.insert(name, inputs);
    }

    for (conjunction, inputs) in module_to_inputs {
        if let Some(Module::Conjunction(m)) = &mut modules.get_mut(conjunction) {
            for inp in inputs {
                m.prev.insert(inp, Pulse::Low);
            }
        }
    }

    Configuration { modules }
}

fn button_presses_to_start(configuration: &mut Configuration, watch_module: &str) -> usize {
    let mut watched_module_receiving_high_pulse_from: HashMap<&str, Vec<usize>> = HashMap::new();

    let mut queue = VecDeque::new();
    'outer: for cycle in 0.. {
        queue.clear();
        queue.push_front((Pulse::Low, ("", "broadcaster")));

        while let Some((pulse_type, (from, to))) = queue.pop_front() {
            if matches!(pulse_type, Pulse::High) && to == watch_module {
                watched_module_receiving_high_pulse_from
                    .entry(from)
                    .and_modify(|c| c.push(cycle))
                    .or_insert(vec![cycle]);
            }

            if watched_module_receiving_high_pulse_from.len() == 4
                && watched_module_receiving_high_pulse_from
                    .iter()
                    .all(|(_, c)| c.len() >= 2)
            {
                break 'outer;
            }

            configuration.process(&pulse_type, (from, to), &mut queue);
        }
    }

    let cycle_loops = watched_module_receiving_high_pulse_from
        .values()
        .map(|cycle_occurences| cycle_occurences[1] - cycle_occurences[0]);
    cycle_loops.reduce(lcm).unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn pulse<'a>(configuration: &'a mut Configuration<'a>, cycles: usize) -> usize {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut queue = VecDeque::new();
    for _ in 0..cycles {
        queue.clear();
        queue.push_front((Pulse::Low, ("", "broadcaster")));

        while let Some((pulse_type, (from, to))) = queue.pop_front() {
            match pulse_type {
                Pulse::High => {
                    high_pulses += 1;
                }
                Pulse::Low => {
                    low_pulses += 1;
                }
            }

            configuration.process(&pulse_type, (from, to), &mut queue);
        }
    }

    low_pulses * high_pulses
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const SAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1_sample() {
        let mut configuration1 = parse_configuration(SAMPLE1);

        assert_eq!(pulse(&mut configuration1, 1), 32);

        let mut configuration1 = parse_configuration(SAMPLE1);
        assert_eq!(pulse(&mut configuration1, 1000), 32_000_000);

        let mut configuration2 = parse_configuration(SAMPLE2);
        assert_eq!(pulse(&mut configuration2, 1000), 11_687_500);
    }
}

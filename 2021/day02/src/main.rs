use std::fs;
use std::path::Path;

enum Command {
    Forward,
    Down,
    Up,
}

impl std::str::FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(std::io::Error::other("Unknown commad")),
        }
    }
}

fn main() {
    let input = Path::new("2021/day02/src/input.txt");
    let commands = parse_commands(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Multiplied horizontal and depth position: {:?}",
        steer(&commands)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Multiplied horizontal and depth position, when using aim: {:?}",
        steer_with_aim(&commands)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_commands(input: &str) -> Vec<(Command, usize)> {
    input
        .lines()
        .map(|line| {
            let (command, value) = line.split_once(" ").unwrap();
            (command.parse().unwrap(), value.parse().unwrap())
        })
        .collect()
}

fn steer(commands: &[(Command, usize)]) -> usize {
    let mut position = (0, 0);
    for (command, value) in commands {
        match command {
            Command::Forward => position.1 += *value,
            Command::Up => position.0 -= *value,
            Command::Down => position.0 += *value,
        }
    }
    position.0 * position.1
}

fn steer_with_aim(commands: &[(Command, usize)]) -> usize {
    let mut aim = 0;
    let mut position = (0, 0);
    for (command, value) in commands {
        match command {
            Command::Forward => {
                position.1 += *value;
                position.0 += *value * aim;
            }
            Command::Up => aim -= *value,
            Command::Down => aim += *value,
        }
    }
    position.0 * position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part_1_sample() {
        let commands = parse_commands(SAMPLE);

        assert_eq!(steer(&commands), 150);
    }

    #[test]
    fn test_part_2_sample() {
        let commands = parse_commands(SAMPLE);

        assert_eq!(steer_with_aim(&commands), 900);
    }
}

use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2025/day01/src/input.txt");
    let rotations_input: String = fs::read_to_string(input).unwrap();

    println!("Step 1");
    println!("Zero positions: {}", count_zero_positions(&rotations_input));

    println!("Step 2");
    println!("Zero passed: {}", count_zero_passes(&rotations_input));
}

#[derive(Debug, PartialEq)]
enum Direction {
    R,
    L,
}

#[derive(Debug, PartialEq)]
struct Position(i32);

#[derive(Debug, PartialEq)]
struct Rotation {
    direction: Direction,
    rotate_by: i32,
}

impl Rotation {
    fn rotate(&self, position: &Position) -> (Position, usize) {
        match self.direction {
            Direction::L => {
                let total_rotation = position.0 - self.rotate_by;
                let rotation_after_wrapping = total_rotation.rem_euclid(100);
                let next_position = if rotation_after_wrapping < 0 {
                    100 + rotation_after_wrapping
                } else {
                    rotation_after_wrapping
                };
                let passing_zero_count = (total_rotation.div_euclid(100)).unsigned_abs() as usize
                    - { if position.0 == 0 { 1 } else { 0 } };

                (Position(next_position), passing_zero_count)
            }
            Direction::R => {
                let total_rotation = position.0 + self.rotate_by;
                let next_position = total_rotation.rem_euclid(100);
                let passing_zero_count = total_rotation.div_euclid(100) as usize - {
                    if next_position == 0 { 1 } else { 0 }
                };

                (Position(next_position), passing_zero_count)
            }
        }
    }
}

fn parse_line(line: &str) -> Rotation {
    let (direction, rotate_by) = line.split_at(1);
    Rotation {
        direction: if direction == "R" {
            Direction::R
        } else {
            Direction::L
        },
        rotate_by: rotate_by.parse().unwrap(),
    }
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input.lines().map(parse_line).collect()
}

fn count_zero_positions(input: &str) -> usize {
    let mut zero_positions = 0;

    let rotations = parse_rotations(input);
    let mut position = Position(50);

    for rotation in rotations.iter() {
        (position, _) = rotation.rotate(&position);
        if position == Position(0) {
            zero_positions += 1;
        }
    }

    zero_positions
}

fn count_zero_passes(input: &str) -> usize {
    let mut total_zero_passes = 0;

    let rotations = parse_rotations(input);
    let mut position = Position(50);

    for rotation in rotations.iter() {
        let (next_position, zero_passes) = rotation.rotate(&position);

        total_zero_passes += zero_passes;
        if next_position == Position(0) {
            total_zero_passes += 1;
        }

        position = next_position
    }

    total_zero_passes
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    const SAMPLE2: &'static str = "L168\nR130\nR480\nL1\nL99\nR14\nL82";

    #[test]
    fn test_parse_line_left() {
        let result = parse_line("L68");
        assert_eq!(
            result,
            Rotation {
                direction: Direction::L,
                rotate_by: 68_i32
            }
        );
    }

    #[test]
    fn test_parse_line_right() {
        let result = parse_line("R60");
        assert_eq!(
            result,
            Rotation {
                direction: Direction::R,
                rotate_by: 60_i32
            }
        );
    }

    #[test]
    fn test_parse_rotations() {
        let result = parse_rotations(SAMPLE2);
        assert_eq!(
            result,
            [
                Rotation {
                    direction: Direction::L,
                    rotate_by: 168_i32
                },
                Rotation {
                    direction: Direction::R,
                    rotate_by: 130_i32
                },
                Rotation {
                    direction: Direction::R,
                    rotate_by: 480_i32
                },
                Rotation {
                    direction: Direction::L,
                    rotate_by: 1_i32
                },
                Rotation {
                    direction: Direction::L,
                    rotate_by: 99_i32
                },
                Rotation {
                    direction: Direction::R,
                    rotate_by: 14_i32
                },
                Rotation {
                    direction: Direction::L,
                    rotate_by: 82_i32
                },
            ]
        )
    }

    #[test]
    fn test_rotation_rotate_left_simple() {
        let rotation18 = Rotation {
            direction: Direction::L,
            rotate_by: 18_i32,
        };
        let rotation0 = Rotation {
            direction: Direction::L,
            rotate_by: 0_i32,
        };

        assert_eq!(rotation18.rotate(&Position(50)), (Position(32), 0));
        assert_eq!(rotation18.rotate(&Position(18)), (Position(0), 0));
        assert_eq!(rotation18.rotate(&Position(0)), (Position(82), 0));
        assert_eq!(rotation0.rotate(&Position(50)), (Position(50), 0));
    }

    #[test]
    fn test_rotation_rotate_left_with_wrap() {
        let rotation168 = Rotation {
            direction: Direction::L,
            rotate_by: 168_i32,
        };
        let rotation300 = Rotation {
            direction: Direction::L,
            rotate_by: 300_i32,
        };
        let rotation78 = Rotation {
            direction: Direction::L,
            rotate_by: 78_i32,
        };
        let rotation1000 = Rotation {
            direction: Direction::L,
            rotate_by: 1000_i32,
        };

        assert_eq!(rotation168.rotate(&Position(50)), (Position(82), 2));
        assert_eq!(rotation300.rotate(&Position(50)), (Position(50), 3));
        assert_eq!(rotation78.rotate(&Position(50)), (Position(72), 1));
        assert_eq!(rotation78.rotate(&Position(0)), (Position(22), 0));
        assert_eq!(rotation78.rotate(&Position(78)), (Position(0), 0));
        assert_eq!(rotation1000.rotate(&Position(50)), (Position(50), 10));
        assert_eq!(rotation168.rotate(&Position(0)), (Position(32), 1));
    }

    #[test]
    fn test_rotation_rotate_right_simple() {
        let rotation18 = Rotation {
            direction: Direction::R,
            rotate_by: 18_i32,
        };
        let rotation0 = Rotation {
            direction: Direction::R,
            rotate_by: 0_i32,
        };

        assert_eq!(rotation18.rotate(&Position(50)), (Position(68), 0));
        assert_eq!(rotation18.rotate(&Position(81)), (Position(99), 0));
        assert_eq!(rotation18.rotate(&Position(18)), (Position(36), 0));
        assert_eq!(rotation18.rotate(&Position(0)), (Position(18), 0));
        assert_eq!(rotation0.rotate(&Position(50)), (Position(50), 0));
    }

    #[test]
    fn test_rotation_rotate_right_with_wrap() {
        let rotation168 = Rotation {
            direction: Direction::R,
            rotate_by: 168_i32,
        };
        let rotation300 = Rotation {
            direction: Direction::R,
            rotate_by: 300_i32,
        };
        let rotation78 = Rotation {
            direction: Direction::R,
            rotate_by: 78_i32,
        };
        let rotation1000 = Rotation {
            direction: Direction::R,
            rotate_by: 1000_i32,
        };

        assert_eq!(rotation168.rotate(&Position(50)), (Position(18), 2));
        assert_eq!(rotation300.rotate(&Position(50)), (Position(50), 3));
        assert_eq!(rotation78.rotate(&Position(50)), (Position(28), 1));
        assert_eq!(rotation78.rotate(&Position(99)), (Position(77), 1));
        assert_eq!(rotation78.rotate(&Position(22)), (Position(0), 0));
        assert_eq!(rotation1000.rotate(&Position(50)), (Position(50), 10));
    }

    #[test]
    fn test_step_1_sample_input() {
        let result = count_zero_positions(SAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_step_2_sample_input() {
        let result = count_zero_passes(SAMPLE);
        assert_eq!(result, 6);
    }
}

use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Robot {
    x: isize,
    y: isize,
    velocity: (isize, isize),
}

impl Robot {
    fn navigate(&mut self, for_seconds: usize, tiles: (isize, isize)) {
        let (x_change, y_change) = self.velocity;
        let total_x = x_change * for_seconds as isize;
        let total_y = y_change * for_seconds as isize;
        self.x = (self.x + total_x).rem_euclid(tiles.0);
        self.y = (self.y + total_y).rem_euclid(tiles.1);
    }
}

fn main() {
    let width = 101;
    let height = 103;
    let input = Path::new("2024/day14/src/input.txt");

    let mut robots_part1 = parse_robots(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    navigate_robots(&mut robots_part1, 100, (width, height));
    let quadrants = count_robots_in_quadrants(&robots_part1, width, height);
    println!("Part 1");
    println!(
        "Safety factor after 100 seconds: {:?}",
        calculate_safety_factor(quadrants)
    );
    println!("In {:?}", part1.elapsed());

    let mut robots_part2 = parse_robots(&fs::read_to_string(input).unwrap());
    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Fewest number of seconds for tree: {:?}",
        find_tree(&mut robots_part2, (width, height))
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let (px, py) = p[2..].split_once(',').unwrap();
            let (vx, vy) = v[2..].split_once(',').unwrap();

            Robot {
                x: px.parse().unwrap(),
                y: py.parse().unwrap(),
                velocity: (vx.parse().unwrap(), vy.parse().unwrap()),
            }
        })
        .collect()
}

fn calculate_safety_factor(quadrants: [usize; 4]) -> usize {
    quadrants.iter().product()
}

fn navigate_robots(robots: &mut [Robot], for_seconds: usize, tiles: (isize, isize)) {
    robots
        .iter_mut()
        .for_each(|robot| robot.navigate(for_seconds, tiles));
}

fn find_tree(robots: &mut [Robot], tiles: (isize, isize)) -> usize {
    for seconds_elapsed in 1.. {
        navigate_robots(robots, 1, tiles);
        if is_tree(robots, tiles) {
            return seconds_elapsed;
        }
    }
    0
}

fn is_tree(robots: &[Robot], (width, height): (isize, isize)) -> bool {
    let mut grid: Vec<Vec<usize>> = (0..height)
        .map(|_| (0..width).map(|_| 0).collect())
        .collect();

    for robot in robots.iter() {
        grid[robot.y as usize][robot.x as usize] += 1;
    }

    let mut iter = grid[52..].iter();

    if TREE.iter().zip(iter.next()).all(|(left, right)| {
        left.chars().zip(right).all(|(l, r)| match r {
            0 => l == '.',
            num => {
                if l == '.' {
                    return false;
                }
                *num == l.to_digit(10).unwrap() as usize
            }
        })
    }) {
        return true;
    }
    false
}

fn count_robots_in_quadrants(robots: &[Robot], width: isize, height: isize) -> [usize; 4] {
    let mut quadrants = [0, 0, 0, 0];

    let middle_row = height / 2;
    let middle_col = width / 2;

    for robot in robots {
        match (
            robot.x < middle_col,
            robot.x > middle_col,
            robot.y < middle_row,
            robot.y > middle_row,
        ) {
            (true, false, true, false) => {
                quadrants[0] += 1;
            }
            (false, true, true, false) => {
                quadrants[1] += 1;
            }
            (true, false, false, true) => {
                quadrants[2] += 1;
            }
            (false, true, false, true) => {
                quadrants[3] += 1;
            }
            _ => {}
        }
    }

    quadrants
}

const TREE: [&str; 33] = [
    "............................1111111111111111111111111111111",
    "............................1.............................1",
    "............1...............1.............................1",
    "............11..............1.............................1",
    "............................1.............................1",
    "............................1..............1..............1",
    "............................1.............111.............1",
    "............................1............11111............1",
    ".......1....................1...........1111111...........1",
    "............................1..........111111111..........1",
    "..............1.............1............11111............1",
    "...............1............1...........1111111...........1",
    "............................1..........111111111..........1",
    "............................1.........11111111111.........1",
    "......................1.....1........1111111111111........1",
    ".........................1..1..........111111111..........1",
    "............................1.........11111111111.........1",
    "............................1........1111111111111........1",
    "............................1.......111111111111111.......1",
    ".........1..................1......11111111111111111......1",
    ".......1..........1.........1........1111111111111........1",
    "........1...................1.......111111111111111.......1",
    "............................1......11111111111111111......1",
    "...............1............1.....1111111111111111111.....1",
    "............1...............1....111111111111111111111....1",
    "............................1.............111.............1",
    ".................1.......1..1.............111.............1",
    "............................1.............111.............1",
    ".....................1......1.............................1",
    "...................1........1.............................1",
    "..............1.............1.............................1",
    "............................1.............................1",
    "............................1111111111111111111111111111111",
];

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse_robots() {
        let robots = parse_robots(SAMPLE);

        assert_eq!(
            robots,
            [
                Robot {
                    x: 0,
                    y: 4,
                    velocity: (3, -3)
                },
                Robot {
                    x: 6,
                    y: 3,
                    velocity: (-1, -3)
                },
                Robot {
                    x: 10,
                    y: 3,
                    velocity: (-1, 2)
                },
                Robot {
                    x: 2,
                    y: 0,
                    velocity: (2, -1)
                },
                Robot {
                    x: 0,
                    y: 0,
                    velocity: (1, 3)
                },
                Robot {
                    x: 3,
                    y: 0,
                    velocity: (-2, -2)
                },
                Robot {
                    x: 7,
                    y: 6,
                    velocity: (-1, -3)
                },
                Robot {
                    x: 3,
                    y: 0,
                    velocity: (-1, -2)
                },
                Robot {
                    x: 9,
                    y: 3,
                    velocity: (2, 3)
                },
                Robot {
                    x: 7,
                    y: 3,
                    velocity: (-1, 2)
                },
                Robot {
                    x: 2,
                    y: 4,
                    velocity: (2, -3)
                },
                Robot {
                    x: 9,
                    y: 5,
                    velocity: (-3, -3)
                },
            ]
        )
    }

    #[test]
    fn test_navigate_robot() {
        let mut robot = Robot {
            x: 2,
            y: 4,
            velocity: (2, -3),
        };
        robot.navigate(1, (11, 7));

        assert_eq!(
            robot,
            Robot {
                x: 4,
                y: 1,
                velocity: (2, -3)
            }
        );

        let mut robot = Robot {
            x: 2,
            y: 4,
            velocity: (2, -3),
        };
        robot.navigate(2, (11, 7));

        assert_eq!(
            robot,
            Robot {
                x: 6,
                y: 5,
                velocity: (2, -3)
            }
        );

        let mut robot = Robot {
            x: 2,
            y: 4,
            velocity: (2, -3),
        };
        robot.navigate(3, (11, 7));

        assert_eq!(
            robot,
            Robot {
                x: 8,
                y: 2,
                velocity: (2, -3)
            }
        );

        let mut robot = Robot {
            x: 2,
            y: 4,
            velocity: (2, -3),
        };
        robot.navigate(4, (11, 7));

        assert_eq!(
            robot,
            Robot {
                x: 10,
                y: 6,
                velocity: (2, -3)
            }
        );

        let mut robot = Robot {
            x: 2,
            y: 4,
            velocity: (2, -3),
        };
        robot.navigate(5, (11, 7));

        assert_eq!(
            robot,
            Robot {
                x: 1,
                y: 3,
                velocity: (2, -3)
            }
        );
    }

    #[test]
    fn test_part_1_sample() {
        let mut robots = parse_robots(SAMPLE);
        let width = 11;
        let height = 7;

        navigate_robots(&mut robots, 100, (width, height));

        let quadrants = count_robots_in_quadrants(&robots, width, height);
        let safety_factor = calculate_safety_factor(quadrants);

        assert_eq!(safety_factor, 12);
    }
}

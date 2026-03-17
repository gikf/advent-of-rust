use std::fs;
use std::path::Path;

const MIN: isize = 200_000_000_000_000;
const MAX: isize = 400_000_000_000_000;

const EPSILON: f64 = 0.0001;

struct Xyz {
    x: f64,
    y: f64,
    z: f64,
}

struct Hailstone {
    pos: Xyz,
    v: Xyz,
}

impl std::str::FromStr for Hailstone {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (positions, velocities) = s.split_once(" @ ").unwrap();
        let positions: Vec<_> = positions
            .split(", ")
            .map(|p| p.trim().parse().unwrap())
            .collect();
        let velocities: Vec<_> = velocities
            .split(", ")
            .map(|v| v.trim().parse().unwrap())
            .collect();
        Ok(Hailstone {
            pos: Xyz {
                x: positions[0],
                y: positions[1],
                z: positions[2],
            },
            v: Xyz {
                x: velocities[0],
                y: velocities[1],
                z: velocities[2],
            },
        })
    }
}

fn main() {
    let input = Path::new("2023/day24/src/input.txt");
    let hailstones = parse_hailstones(&fs::read_to_string(input).unwrap());

    let part1t = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Intersections within test area: {:?}",
        intersecting_hailstones(&hailstones, MIN, MAX)
    );
    println!("In {:?}", part1t.elapsed());

    let part2t = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of x, y, z coordinates of initial position of the throw: {:?}",
        rock_at_t_zero(&hailstones)
    );
    println!("In {:?}", part2t.elapsed());
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn intersecting_hailstones(hailstones: &[Hailstone], area_min: isize, area_max: isize) -> usize {
    let mut count = 0;
    for (index, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(index + 1) {
            if are_intersecting(h1, h2, area_min, area_max) {
                count += 1;
            }
        }
    }
    count
}

fn are_intersecting(h1: &Hailstone, h2: &Hailstone, area_min: isize, area_max: isize) -> bool {
    let p1 = &h1.pos;
    let p2 = Xyz {
        x: p1.x + h1.v.x,
        y: p1.y + h1.v.y,
        z: p1.z + h1.v.z,
    };

    let p3 = &h2.pos;
    let p4 = Xyz {
        x: p3.x + h2.v.x,
        y: p3.y + h2.v.y,
        z: p3.z + h2.v.z,
    };

    let denominator = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    if denominator == 0. {
        return false;
    }
    let t = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let u = (p1.x - p3.x) * (p1.y - p2.y) - (p1.y - p3.y) * (p1.x - p2.x);
    if (t < 0.) != (denominator < 0.) || (u < 0.) != (denominator < 0.) {
        return false;
    }
    let t = t / denominator;

    let x = p1.x + t * (p2.x - p1.x);
    let y = p1.y + t * (p2.y - p1.y);

    ((area_min as f64)..=(area_max as f64)).contains(&x)
        && ((area_min as f64)..=(area_max as f64)).contains(&y)
}

fn find_rock(a: &Hailstone, b: &Hailstone, vx: f64, vy: f64, vz: f64) -> Option<(f64, f64, Xyz)> {
    let t2_numerator = b.pos.y - a.pos.y - (((a.v.y - vy) * (b.pos.x - a.pos.x)) / (a.v.x - vx));
    let t2_denominator = vy - b.v.y - (((a.v.y - vy) * (vx - b.v.x)) / (a.v.x - vx));

    let t2 = t2_numerator / t2_denominator;

    let t1 = (b.pos.x - a.pos.x - t2 * (vx - b.v.x)) / (a.v.x - vx);

    let px = a.pos.x - t1 * (vx - a.v.x);
    let py = a.pos.y - t1 * (vy - a.v.y);
    let pz = a.pos.z - t1 * (vz - a.v.z);

    if (pz + t2 * vz - t2 * b.v.z - b.pos.z).abs() > EPSILON {
        None
    } else {
        Some((
            t1,
            t2,
            Xyz {
                x: px,
                y: py,
                z: pz,
            },
        ))
    }
}

fn rock_at_t_zero(hailstones: &[Hailstone]) -> usize {
    // based on https://old.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kerasvv/
    let a = &hailstones[0];
    let b = &hailstones[1];

    let is_int = |f: f64| f == (f as usize) as f64;

    for vx in -500..500 {
        for vy in -500..500 {
            'outer: for vz in -500..500 {
                let vx = vx as f64;
                let vy = vy as f64;
                let vz = vz as f64;

                if let Some((
                    t1,
                    t2,
                    Xyz {
                        x: px,
                        y: py,
                        z: pz,
                    },
                )) = find_rock(a, b, vx, vy, vz)
                {
                    if [t1, t2, px, py, pz].iter().all(|v| v.is_infinite())
                        || t1.is_sign_negative()
                        || t2.is_sign_negative()
                        || [t1, t2, px, py, pz].iter().all(|v| !is_int(*v))
                    {
                        continue;
                    }

                    for c in hailstones.iter().skip(2) {
                        let t3 = (c.pos.x - px) / (vx - c.v.x);

                        if (py + t3 * vy - c.pos.y - t3 * c.v.y).abs() > EPSILON
                            || (pz + t3 * vz - c.pos.z - t3 * c.v.z).abs() > EPSILON
                        {
                            continue 'outer;
                        }
                    }

                    return px as usize + py as usize + pz as usize;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part_1_sample() {
        let hailstones = parse_hailstones(SAMPLE);

        assert_eq!(intersecting_hailstones(&hailstones, 7, 27), 2);
    }

    #[test]
    fn test_part_2_sample() {
        let hailstones = parse_hailstones(SAMPLE);

        assert_eq!(rock_at_t_zero(&hailstones), 47);
    }
}

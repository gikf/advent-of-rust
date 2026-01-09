use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day01/src/input.txt");
    let elfs = parse_elfs(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Most Calories carried: {:?}", most_calories_carried(&elfs));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let number_of_elfs = 3;
    println!("Part 2");
    println!(
        "Calories carried by top three elfs: {:?}",
        calories_carried_by_top(&elfs, number_of_elfs)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_elfs(input: &str) -> Vec<Vec<usize>> {
    let mut elfs = Vec::new();
    let mut current_elf = Vec::new();
    for line in input.lines().chain([""]) {
        if line.is_empty() {
            elfs.push(current_elf.clone());
            current_elf.clear();
            continue;
        }
        current_elf.push(line.parse().unwrap());
    }
    elfs
}

fn most_calories_carried(elfs: &[Vec<usize>]) -> usize {
    elfs.iter()
        .map(|calories| calories.iter().sum::<usize>())
        .max()
        .unwrap()
}

fn calories_carried_by_top(elfs: &[Vec<usize>], number_of_elfs: usize) -> usize {
    let mut carried: Vec<usize> = elfs.iter().map(|calories| calories.iter().sum()).collect();
    carried.sort_by(|a, b| b.cmp(a));

    carried.iter().take(number_of_elfs).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000\n2000\n3000

4000

5000\n6000

7000\n8000\n9000

10000";

    #[test]
    fn test_most_calories_carried() {
        let elfs = parse_elfs(SAMPLE);

        assert_eq!(most_calories_carried(&elfs), 24000);
    }

    #[test]
    fn test_parse_elfs() {
        let elfs = parse_elfs(SAMPLE);

        assert_eq!(
            elfs,
            [
                Vec::from([1000, 2000, 3000]),
                Vec::from([4000]),
                Vec::from([5000, 6000]),
                Vec::from([7000, 8000, 9000]),
                Vec::from([10000])
            ]
        );
    }

    #[test]
    fn test_calories_carried_by_top() {
        let elfs = parse_elfs(SAMPLE);
        let number_of_elfs = 3;

        assert_eq!(calories_carried_by_top(&elfs, number_of_elfs), 45000);
    }
}

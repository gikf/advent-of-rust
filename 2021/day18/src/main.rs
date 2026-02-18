use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day18/src/input.txt");
    let mut snail_numbers = parse_snailfish(&fs::read_to_string(input).unwrap());

    let mut part1_snail_numbers = snail_numbers.clone();
    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Magnitude of the sum: {:?}",
        magnitude_snails(&mut part1_snail_numbers)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Largest magnitude of sum of two different snails: {:?}",
        largest_magnitude(&mut snail_numbers)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_snailfish(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .fold((0_usize, Vec::new()), |(mut depth, mut nums), ch| {
                    match ch {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        '0'..='9' => nums.push((depth, ch.to_digit(10).unwrap() as usize)),
                        _ => {}
                    }
                    (depth, nums)
                })
                .1
        })
        .collect()
}

fn largest_magnitude(snail_numbers: &mut [Vec<(usize, usize)>]) -> usize {
    let mut largest = 0;
    for (index, snail1) in snail_numbers
        .iter()
        .take(snail_numbers.len() - 1)
        .enumerate()
    {
        for snail2 in snail_numbers.iter().skip(index + 1) {
            let mut pair_a = vec![snail1.clone(), snail2.clone()];

            largest = largest.max(magnitude_snails(&mut pair_a));

            let mut pair_b = vec![snail2.clone(), snail1.clone()];
            largest = largest.max(magnitude_snails(&mut pair_b));
        }
    }

    largest
}

fn magnitude_snails(snail_numbers: &mut Vec<Vec<(usize, usize)>>) -> usize {
    while snail_numbers.len() > 1 {
        let mut other = snail_numbers.remove(1);
        let snail_number = &mut snail_numbers[0];
        add_snails(snail_number, &mut other);
        reduce(snail_number, 0);
    }

    magnitude(&mut 0, 1, &snail_numbers[0])
}

fn add_snails(snail_number: &mut Vec<(usize, usize)>, other: &mut Vec<(usize, usize)>) {
    snail_number.append(other);
    snail_number.iter_mut().for_each(|(depth, _)| *depth += 1);
}

fn magnitude(index: &mut usize, depth: usize, snail_number: &Vec<(usize, usize)>) -> usize {
    3 * if snail_number[*index].0 == depth {
        *index += 1;
        snail_number[*index - 1].1
    } else {
        magnitude(index, depth + 1, snail_number)
    } + 2 * if snail_number[*index].0 == depth {
        *index += 1;
        snail_number[*index - 1].1
    } else {
        magnitude(index, depth + 1, snail_number)
    }
}

fn reduce(snail_number: &mut Vec<(usize, usize)>, start: usize) {
    for index in start..snail_number.len() - 1 {
        if snail_number[index].0 == 5 {
            let (left, right) = (snail_number[index].1, snail_number[index + 1].1);

            snail_number[index] = (4, 0);
            snail_number.remove(index + 1);
            if index >= 1 {
                let _ = snail_number.get_mut(index - 1).map(|num| num.1 += left);
            }
            let _ = snail_number.get_mut(index + 1).map(|num| num.1 += right);
            return reduce(snail_number, index);
        }
    }
    for index in 0..snail_number.len() {
        let (depth, num) = snail_number[index];
        if num >= 10 {
            snail_number[index] = (depth + 1, num / 2);
            snail_number.insert(index + 1, (depth + 1, num.div_ceil(2)));
            return reduce(snail_number, index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_part_1_sample() {
        let mut parsed = parse_snailfish(SAMPLE);

        assert_eq!(magnitude_snails(&mut parsed), 4140);
    }

    #[test]
    fn test_part_2_sample() {
        let mut parsed = parse_snailfish(SAMPLE);

        assert_eq!(largest_magnitude(&mut parsed), 3993);
    }
}

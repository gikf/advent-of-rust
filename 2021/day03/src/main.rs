use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day03/src/input.txt");
    let bits = parse_bits(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Power consumption: {:?}", power_consumption(&bits));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!("Life support rating {:?}", life_support_rating(&bits));
    println!("In {:?}", part2.elapsed());
}

fn parse_bits(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn bit_ocurrences(bits: &[Vec<u32>]) -> Vec<[i32; 2]> {
    let mut ocurrences = vec![[0, 0]; bits[0].len()];
    for bit in bits {
        for (position, value) in bit.iter().enumerate() {
            ocurrences[position][usize::try_from(*value).unwrap()] += 1;
        }
    }
    ocurrences
}

fn power_consumption(bits: &[Vec<u32>]) -> usize {
    let ocurrences = bit_ocurrences(bits);
    let gamma_rate_binary: Vec<_> = ocurrences
        .iter()
        .map(|[zeros, ones]| if zeros > ones { 0 } else { 1 })
        .collect();
    let epsilon_binary: Vec<_> = gamma_rate_binary
        .iter()
        .map(|v| if *v == 1 { 0 } else { 1 })
        .collect();

    let binary_to_usize = |binary: &[usize]| -> usize {
        binary
            .iter()
            .rev()
            .enumerate()
            .map(|(index, v)| v * 2_usize.pow(u32::try_from(index).unwrap()))
            .sum::<usize>()
    };

    let gamma_rate = binary_to_usize(&gamma_rate_binary);
    let epsilon = binary_to_usize(&epsilon_binary);

    gamma_rate * epsilon
}

fn life_support_rating(bits: &[Vec<u32>]) -> usize {
    let mut oxygen_generator: Vec<_> = bits.to_vec();
    for position in 0..bits[0].len() {
        let ocurrences = bit_ocurrences(&oxygen_generator);
        let [zeros, ones] = ocurrences[position];

        oxygen_generator.retain(|bit| {
            if ones >= zeros {
                bit[position] == 1
            } else {
                bit[position] == 0
            }
        });
        if oxygen_generator.len() <= 1 {
            break;
        }
    }

    let mut co2_scrubber: Vec<_> = bits.to_vec();
    for position in 0..bits[0].len() {
        let ocurrences = bit_ocurrences(&co2_scrubber);
        let [zeros, ones] = ocurrences[position];

        co2_scrubber.retain(|bit| {
            if zeros <= ones {
                bit[position] == 0
            } else {
                bit[position] == 1
            }
        });
        if co2_scrubber.len() <= 1 {
            break;
        }
    }

    let unwrap_rating = |items: &[Vec<u32>]| {
        usize::try_from(
            items[0]
                .iter()
                .rev()
                .enumerate()
                .map(|(index, v)| *v * 2_u32.pow(u32::try_from(index).unwrap()))
                .sum::<u32>(),
        )
        .unwrap()
    };

    let oxygen_generator_rating = unwrap_rating(&oxygen_generator);
    let co2_scrubber_rating = unwrap_rating(&co2_scrubber);

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "00100\n11110\n10110\n10111
10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn test_part_1_sample() {
        let bits = parse_bits(SAMPLE);
        assert_eq!(power_consumption(&bits), 198);
    }

    #[test]
    fn test_part_2_sample() {
        let bits = parse_bits(SAMPLE);
        assert_eq!(life_support_rating(&bits), 230);
    }
}

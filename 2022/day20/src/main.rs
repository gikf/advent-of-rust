use std::fs;
use std::path::Path;

const MIX_TIMES_PART1: usize = 1;
const MIX_TIMES_PART2: usize = 10;
const DECRYPTION_KEY_PART1: isize = 1;
const DECRYPTION_KEY_PART2: isize = 811589153;

fn main() {
    let input = Path::new("2022/day20/src/input.txt");
    let file = parse_file(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of three numbers forming groove coordinates: {:?}",
        groove_coordinates(&file, DECRYPTION_KEY_PART1, MIX_TIMES_PART1)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of numbers forming groove coordinates, after decrypting and mixing 10 times: {:?}",
        groove_coordinates(&file, DECRYPTION_KEY_PART2, MIX_TIMES_PART2)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_file(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix_file(file: &[(usize, isize)], times_to_mix: usize) -> Vec<isize> {
    let mut mixed: Vec<_> = file.to_vec();
    let spots = file.len() as isize - 1;

    for _ in 0..times_to_mix {
        for id_num in file.iter() {
            let num_id = mixed.iter().position(|i_n| i_n == id_num).unwrap() as isize;
            mixed.remove(num_id as usize);
            let new_id = (num_id + id_num.1 + spots).rem_euclid(spots) as usize;
            mixed.insert(new_id, *id_num);
        }
    }

    mixed.iter().map(|(_, v)| *v).collect()
}

fn groove_coordinates(file: &[isize], decryption_key: isize, mix_times: usize) -> isize {
    let file: Vec<_> = file
        .iter()
        .enumerate()
        .map(|(id, v)| (id, *v * decryption_key))
        .collect();
    let mixed = mix_file(&file, mix_times);
    let zero_index = mixed.iter().position(|v| *v == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|offset| mixed[(zero_index + offset).rem_euclid(mixed.len())])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn test_parse_file() {
        let parsed = parse_file(SAMPLE);

        assert_eq!(parsed, [1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn test_part_1_sample() {
        let file = parse_file(SAMPLE);

        assert_eq!(groove_coordinates(&file, 1, 1), 3);
    }

    #[test]
    fn test_part_2_sample() {
        let file = parse_file(SAMPLE);

        assert_eq!(
            groove_coordinates(&file, DECRYPTION_KEY_PART2, 10),
            1623178306
        );
    }
}

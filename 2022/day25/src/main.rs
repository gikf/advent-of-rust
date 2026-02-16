use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day25/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let snafus = parse_snafus(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("SNAFU to enter to console: {:?}", snafu_to_enter(&snafus));
    println!("In {:?}", part1.elapsed());
}

fn sum_snafu_lines_to_decimal(snafus: &[&str]) -> isize {
    snafus.iter().map(|snafu| snafu_to_decimal(snafu)).sum()
}

fn parse_snafus(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn snafu_to_decimal(snafu: &str) -> isize {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(offset, c)| {
            5_isize.pow(offset as u32)
                * match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => unimplemented!(),
                }
        })
        .sum()
}

fn decimal_to_snafu(value: isize) -> String {
    let mut result = Vec::new();
    let mut digits = Vec::new();
    let mut value = value;

    while value > 0 {
        digits.push(value.rem_euclid(5));
        value /= 5;
    }

    let mut offset = 0;
    let mut digits_it = digits.iter();
    loop {
        let next_digit = match (digits_it.next(), offset) {
            (Some(num), offset) => num + offset,
            (None, offset) if offset > 0 => offset,
            _ => break,
        };
        offset = 0;
        match next_digit {
            0 => result.push("0"),
            1 => result.push("1"),
            2 => result.push("2"),
            3 => {
                result.push("=");
                offset = 1;
            }
            4 => {
                result.push("-");
                offset = 1;
            }
            5 => {
                result.push("0");
                offset = 1;
            }
            _ => unimplemented!(),
        }
    }

    result.into_iter().rev().collect()
}

fn snafu_to_enter(snafus: &[&str]) -> String {
    let decimal = sum_snafu_lines_to_decimal(snafus);
    decimal_to_snafu(decimal)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    const SNAFUS: [&str; 15] = [
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
        "1=0",
        "1-0",
        "1=11-2",
        "1-0---0",
        "1121-1110-1=0",
    ];
    const DECIMALS: [isize; 15] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
    ];

    const SNAFUS2: [&str; 13] = [
        "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
        "1=", "122",
    ];

    const DECIMALS2: [isize; 13] = [1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37];

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal(SNAFUS[0]), DECIMALS[0]);
        assert_eq!(snafu_to_decimal(SNAFUS[1]), DECIMALS[1]);
        assert_eq!(snafu_to_decimal(SNAFUS[2]), DECIMALS[2]);
        assert_eq!(snafu_to_decimal(SNAFUS[3]), DECIMALS[3]);
        assert_eq!(snafu_to_decimal(SNAFUS[4]), DECIMALS[4]);
        assert_eq!(snafu_to_decimal(SNAFUS[5]), DECIMALS[5]);
        assert_eq!(snafu_to_decimal(SNAFUS[6]), DECIMALS[6]);
        assert_eq!(snafu_to_decimal(SNAFUS[7]), DECIMALS[7]);
        assert_eq!(snafu_to_decimal(SNAFUS[8]), DECIMALS[8]);
        assert_eq!(snafu_to_decimal(SNAFUS[9]), DECIMALS[9]);
        assert_eq!(snafu_to_decimal(SNAFUS[10]), DECIMALS[10]);
        assert_eq!(snafu_to_decimal(SNAFUS[11]), DECIMALS[11]);
        assert_eq!(snafu_to_decimal(SNAFUS[12]), DECIMALS[12]);
        assert_eq!(snafu_to_decimal(SNAFUS[13]), DECIMALS[13]);
        assert_eq!(snafu_to_decimal(SNAFUS[14]), DECIMALS[14]);
    }

    #[test]
    fn test_snafu_to_decimal2() {
        assert_eq!(snafu_to_decimal(SNAFUS2[0]), DECIMALS2[0]);
        assert_eq!(snafu_to_decimal(SNAFUS2[1]), DECIMALS2[1]);
        assert_eq!(snafu_to_decimal(SNAFUS2[2]), DECIMALS2[2]);
        assert_eq!(snafu_to_decimal(SNAFUS2[3]), DECIMALS2[3]);
        assert_eq!(snafu_to_decimal(SNAFUS2[4]), DECIMALS2[4]);
        assert_eq!(snafu_to_decimal(SNAFUS2[5]), DECIMALS2[5]);
        assert_eq!(snafu_to_decimal(SNAFUS2[6]), DECIMALS2[6]);
        assert_eq!(snafu_to_decimal(SNAFUS2[7]), DECIMALS2[7]);
        assert_eq!(snafu_to_decimal(SNAFUS2[8]), DECIMALS2[8]);
        assert_eq!(snafu_to_decimal(SNAFUS2[9]), DECIMALS2[9]);
        assert_eq!(snafu_to_decimal(SNAFUS2[10]), DECIMALS2[10]);
        assert_eq!(snafu_to_decimal(SNAFUS2[11]), DECIMALS2[11]);
        assert_eq!(snafu_to_decimal(SNAFUS2[12]), DECIMALS2[12]);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(DECIMALS[0]), SNAFUS[0]);
        assert_eq!(decimal_to_snafu(DECIMALS[1]), SNAFUS[1]);
        assert_eq!(decimal_to_snafu(DECIMALS[2]), SNAFUS[2]);
        assert_eq!(decimal_to_snafu(DECIMALS[3]), SNAFUS[3]);
        assert_eq!(decimal_to_snafu(DECIMALS[4]), SNAFUS[4]);
        assert_eq!(decimal_to_snafu(DECIMALS[5]), SNAFUS[5]);
        assert_eq!(decimal_to_snafu(DECIMALS[6]), SNAFUS[6]);
        assert_eq!(decimal_to_snafu(DECIMALS[7]), SNAFUS[7]);
        assert_eq!(decimal_to_snafu(DECIMALS[8]), SNAFUS[8]);
        assert_eq!(decimal_to_snafu(DECIMALS[9]), SNAFUS[9]);
        assert_eq!(decimal_to_snafu(DECIMALS[10]), SNAFUS[10]);
        assert_eq!(decimal_to_snafu(DECIMALS[11]), SNAFUS[11]);
        assert_eq!(decimal_to_snafu(DECIMALS[12]), SNAFUS[12]);
        assert_eq!(decimal_to_snafu(DECIMALS[13]), SNAFUS[13]);
        assert_eq!(decimal_to_snafu(DECIMALS[14]), SNAFUS[14]);
    }

    #[test]
    fn test_decimal_to_snafu2() {
        assert_eq!(decimal_to_snafu(DECIMALS2[0]), SNAFUS2[0]);
        assert_eq!(decimal_to_snafu(DECIMALS2[1]), SNAFUS2[1]);
        assert_eq!(decimal_to_snafu(DECIMALS2[2]), SNAFUS2[2]);
        assert_eq!(decimal_to_snafu(DECIMALS2[3]), SNAFUS2[3]);
        assert_eq!(decimal_to_snafu(DECIMALS2[4]), SNAFUS2[4]);
        assert_eq!(decimal_to_snafu(DECIMALS2[5]), SNAFUS2[5]);
        assert_eq!(decimal_to_snafu(DECIMALS2[6]), SNAFUS2[6]);
        assert_eq!(decimal_to_snafu(DECIMALS2[7]), SNAFUS2[7]);
        assert_eq!(decimal_to_snafu(DECIMALS2[8]), SNAFUS2[8]);
        assert_eq!(decimal_to_snafu(DECIMALS2[9]), SNAFUS2[9]);
        assert_eq!(decimal_to_snafu(DECIMALS2[10]), SNAFUS2[10]);
        assert_eq!(decimal_to_snafu(DECIMALS2[11]), SNAFUS2[11]);
        assert_eq!(decimal_to_snafu(DECIMALS2[12]), SNAFUS2[12]);
    }

    #[test]
    fn test_part_1_sample() {
        let snafus = parse_snafus(SAMPLE);
        assert_eq!(snafu_to_enter(&snafus), "2=-1=0");
    }
}

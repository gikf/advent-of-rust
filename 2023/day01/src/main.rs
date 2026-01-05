use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2023/day01/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of calibration values: {:?}",
        calibration_values(contents)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Sum of calibration digits, with spelt out digits: {:?}",
        total_calibration_digits(contents)
    );
    println!("In {:?}", part2.elapsed());
}

fn calibration_values(input: &str) -> u32 {
    input.lines().map(calibration_value).sum()
}

fn calibration_value(line: &str) -> u32 {
    let first = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last = line
        .chars()
        .rfind(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();

    first * 10 + last
}

const NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn calibration_digits(line: &str) -> u32 {
    let mut first_digit = None;
    if let Some(digit) = line.chars().find(|c| c.is_numeric()) {
        let index = line.chars().position(|c| c == digit).unwrap();
        first_digit = Some((index, digit.to_digit(10).unwrap()))
    }

    let mut last_digit = None;
    if let Some(digit) = line.chars().rfind(|c| c.is_numeric()) {
        let index = line.rfind(digit).unwrap();
        last_digit = Some((index, digit.to_digit(10).unwrap()))
    }

    let mut first_spelt = None;
    NUMS.iter().for_each(|spelt| {
        if let Some(index) = line.find(*spelt)
            && (first_spelt.is_none() || first_spelt.is_some_and(|(v, _)| v > index))
        {
            first_spelt = Some((index, spelt_to_digit(spelt)));
        }
    });

    let mut last_spelt = None;
    NUMS.iter().for_each(|spelt| {
        if let Some(index) = line.rfind(*spelt)
            && (last_spelt.is_none() || last_spelt.is_some_and(|(v, _)| v < index))
        {
            last_spelt = Some((index, spelt_to_digit(spelt)));
        }
    });

    let first: u32 = match (first_digit, first_spelt) {
        (Some((index1, digit)), Some((index2, _))) if index1 < index2 => digit,
        (Some((_, digit)), None) => digit,
        (Some((index1, digit)), Some((index2, digit_from_spelt))) if index1 > index2 => {
            digit_from_spelt
        }
        (None, Some((_, digit_from_spelt))) => digit_from_spelt,
        _ => unreachable!(),
    };
    let last: u32 = match (last_digit, last_spelt) {
        (Some((index1, digit)), Some((index2, digit_from_spelt))) if index1 > index2 => digit,
        (Some((_, digit)), None) => digit,
        (Some((index1, digit)), Some((index2, digit_from_spelt))) if index1 < index2 => {
            digit_from_spelt
        }
        (None, Some((_, digit_from_spelt))) => digit_from_spelt,
        _ => unreachable!(),
    };

    first * 10 + last
}

fn total_calibration_digits(input: &str) -> u32 {
    input.lines().map(calibration_digits).sum()
}

fn spelt_to_digit(spelt: &str) -> u32 {
    match spelt {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const SAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_calibration_value() {
        let numbers: Vec<_> = SAMPLE.lines().collect();
        assert_eq!(calibration_value(numbers[0]), 12);
        assert_eq!(calibration_value(numbers[1]), 38);
        assert_eq!(calibration_value(numbers[2]), 15);
        assert_eq!(calibration_value(numbers[3]), 77);
    }

    #[test]
    fn test_calibration_digits() {
        let numbers: Vec<_> = SAMPLE2.lines().collect();
        assert_eq!(calibration_digits(numbers[0]), 29);
        assert_eq!(calibration_digits(numbers[1]), 83);
        assert_eq!(calibration_digits(numbers[2]), 13);
        assert_eq!(calibration_digits(numbers[3]), 24);
        assert_eq!(calibration_digits(numbers[4]), 42);
        assert_eq!(calibration_digits(numbers[5]), 14);
        assert_eq!(calibration_digits(numbers[6]), 76);
    }

    #[test]
    fn test_calibration_digits2() {
        assert_eq!(calibration_digits("5sevenlnrnqjq77eight5"), 55);
    }

    #[test]
    fn test_calibration_values() {
        assert_eq!(calibration_values(SAMPLE), 142);
    }

    #[test]
    fn test_total_calibration_digits() {
        assert_eq!(total_calibration_digits(SAMPLE2), 281);
    }
}

use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2022/day06/src/input.txt");
    let contents = &fs::read_to_string(input).unwrap();

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Characters to process before start-of-packet marker: {:?}",
        find_marker(contents, 4)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Characters to process before 14 character start-of-packer marker: {:?}",
        find_marker(contents, 14)
    );
    println!("In {:?}", part2.elapsed());
}

fn find_marker(buffer: &str, marker_length: usize) -> usize {
    let mut unique_chars_in_slice = Vec::with_capacity(marker_length);
    let marker_boundary = marker_length - 1;
    buffer
        .char_indices()
        .position(|(slice_index_end, _)| {
            if slice_index_end > marker_boundary {
                unique_chars_in_slice.clear();

                let slice_index_start = slice_index_end - marker_boundary;

                for ch in buffer[slice_index_start..=slice_index_end].chars() {
                    if unique_chars_in_slice.contains(&ch) {
                        return false;
                    }
                    unique_chars_in_slice.push(ch);
                }
                unique_chars_in_slice.len() == marker_length
            } else {
                false
            }
        })
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_find_marker() {
        let marker_length = 4;
        assert_eq!(find_marker(SAMPLE1, marker_length), 7);
        assert_eq!(find_marker(SAMPLE2, marker_length), 5);
        assert_eq!(find_marker(SAMPLE3, marker_length), 6);
        assert_eq!(find_marker(SAMPLE4, marker_length), 10);
        assert_eq!(find_marker(SAMPLE5, marker_length), 11);
    }
    #[test]
    fn test_fin_marker_length_14() {
        let marker_length = 14;
        assert_eq!(find_marker(SAMPLE1, marker_length), 19);
        assert_eq!(find_marker(SAMPLE2, marker_length), 23);
        assert_eq!(find_marker(SAMPLE3, marker_length), 23);
        assert_eq!(find_marker(SAMPLE4, marker_length), 29);
        assert_eq!(find_marker(SAMPLE5, marker_length), 26);
    }
}

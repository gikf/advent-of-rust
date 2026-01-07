use std::fs;
use std::path::Path;

type MapRange = Vec<(usize, usize, usize)>;

fn main() {
    let input = Path::new("2023/day05/src/input.txt");
    let (seeds, maps) = parse_input(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Lowest location for seeds: {:?}",
        lowest_location(&seeds, &maps)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Lowest location for seed ranges: {:?}",
        lowest_location_from_seed_ranges(&seeds, &maps)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<MapRange>) {
    let mut lines = input.lines().chain([""]);

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    lines.next();

    let mut maps = Vec::new();
    let mut current_map = Vec::new();

    for line in lines {
        if line.contains("map") {
            continue;
        } else if line.is_empty() {
            maps.push(current_map.clone());
            current_map.clear();
            continue;
        }

        let range_data: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let start = range_data[1];
        let end = start + range_data[2];
        let starting_num = range_data[0];
        current_map.push((start, end, starting_num));
    }
    (seeds, maps)
}

fn seeds_to_seed_ranges(seeds: &[usize]) -> Vec<(usize, usize)> {
    let mut seed_ranges = Vec::new();
    for chunk in seeds.chunks(2) {
        if let &[start, length] = chunk {
            seed_ranges.push((start, start + length));
        }
    }
    seed_ranges
}

fn seeds_to_location(seeds: &[usize], maps: &[MapRange]) -> Vec<usize> {
    seeds
        .iter()
        .map(|seed| {
            let mut location = *seed;
            for ranges in maps {
                for (start, end, starting_num) in ranges.iter() {
                    if location >= *start && location < *end {
                        location = location - *start + *starting_num;
                        break;
                    }
                }
            }
            location
        })
        .collect()
}

fn seed_ranges_to_location_ranges(
    seed_ranges: &[(usize, usize)],
    maps: &[MapRange],
) -> Vec<(usize, usize)> {
    seed_ranges
        .iter()
        .flat_map(|(seed_range_start, seed_range_end)| {
            let mut seeds = vec![(*seed_range_start, *seed_range_end)];
            for ranges in maps {
                let mut next_seeds = vec![];
                let mut loop_again = true;
                while loop_again {
                    loop_again = false;

                    for (range_start, range_end, starting_num) in ranges.iter() {
                        let seeds_contained_by_range =
                            seeds.extract_if(.., |(seed_start, seed_end)| {
                                *seed_start >= *range_start && *seed_end <= *range_end
                            });
                        for (seed_start, seed_end) in seeds_contained_by_range {
                            let overlap_length = seed_end - seed_start;
                            let start_offset = seed_start - range_start;
                            let mapping_start = *starting_num + start_offset;

                            next_seeds.push((mapping_start, mapping_start + overlap_length));
                        }

                        let seeds_containing_range: Vec<_> = seeds
                            .extract_if(.., |(seed_start, seed_end)| {
                                *seed_start <= *range_start && *seed_end >= *range_end
                            })
                            .collect();
                        for (seed_start, seed_end) in seeds_containing_range {
                            let overlap_length = range_end - range_start;
                            let mapping_start = *starting_num;

                            if seed_start != *range_start {
                                seeds.push((seed_start, *range_start));
                                loop_again = true;
                            }
                            next_seeds.push((mapping_start, mapping_start + overlap_length));
                            if seed_end != *range_end {
                                seeds.push((*range_end, seed_end));
                                loop_again = true;
                            }
                        }

                        let seeds_overlapping_from_bottom: Vec<_> = seeds
                            .extract_if(.., |(seed_start, seed_end)| {
                                *seed_start < *range_start
                                    && *seed_end > *range_start
                                    && *range_end > *seed_end
                            })
                            .collect();
                        for (seed_start, seed_end) in seeds_overlapping_from_bottom {
                            let overlap_length = seed_end - *range_start;
                            let mapping_start = *starting_num;

                            if seed_start != *range_start {
                                seeds.push((seed_start, *range_start));
                                loop_again = true;
                            }
                            next_seeds.push((mapping_start, mapping_start + overlap_length));
                        }

                        let seeds_overlapping_from_top: Vec<_> = seeds
                            .extract_if(.., |(seed_start, seed_end)| {
                                *seed_start > *range_start
                                    && *seed_start < *range_end
                                    && *seed_end > *range_end
                            })
                            .collect();
                        for (seed_start, seed_end) in seeds_overlapping_from_top {
                            let overlap_length = range_end - seed_start;
                            let start_offset = seed_start - range_start;
                            let mapping_start = *starting_num + start_offset;

                            next_seeds.push((mapping_start, mapping_start + overlap_length));
                            if *range_end != seed_end {
                                seeds.push((*range_end, seed_end));
                                loop_again = true;
                            }
                        }
                    }
                }
                next_seeds.extend(seeds);
                seeds = next_seeds;
            }
            seeds
        })
        .collect()
}

fn lowest_location(seeds: &[usize], maps: &[MapRange]) -> usize {
    seeds_to_location(seeds, maps).into_iter().min().unwrap()
}

fn lowest_location_from_seed_ranges(seeds: &[usize], maps: &[MapRange]) -> usize {
    let seed_ranges = seeds_to_seed_ranges(seeds);

    let location_ranges = seed_ranges_to_location_ranges(&seed_ranges, maps);

    location_ranges
        .iter()
        .map(|(start, _)| *start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:\n50 98 2\n52 50 48

soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15

fertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4

water-to-light map:\n88 18 7\n18 25 70

light-to-temperature map:\n45 77 23\n81 45 19\n68 64 13

temperature-to-humidity map:\n0 69 1\n1 0 69

humidity-to-location map:\n60 56 37\n56 93 4";
    const SAMPLE2: &str = "seeds: 500 1

seed-to-soil map:\n1000 500 1

soil-to-fertilizer map:\n0 0 1

fertilizer-to-water map:\n0 0 1

water-to-light map:\n0 0 1

light-to-temperature map:\n0 0 1

temperature-to-humidity map:\n0 0 1

humidity-to-location map:\n0 0 1";

    #[test]
    fn test_parse_input() {
        let (seeds, maps) = parse_input(SAMPLE);

        assert_eq!(seeds, [79, 14, 55, 13]);
        assert_eq!(maps[0], [(98, 100, 50), (50, 98, 52)]);
        assert_eq!(maps[1], [(15, 52, 0), (52, 54, 37), (0, 15, 39)]);
        assert_eq!(
            maps[2],
            [(53, 61, 49), (11, 53, 0), (0, 7, 42), (7, 11, 57)]
        );
        assert_eq!(maps[3], [(18, 25, 88), (25, 95, 18)]);
        assert_eq!(maps[4], [(77, 100, 45), (45, 64, 81), (64, 77, 68)]);
        assert_eq!(maps[5], [(69, 70, 0), (0, 69, 1)]);
        assert_eq!(maps[6], [(56, 93, 60), (93, 97, 56)]);
    }

    #[test]
    fn test_seeds_to_location() {
        let (seeds, maps) = parse_input(SAMPLE);

        let locations = seeds_to_location(&seeds, &maps);
        assert_eq!(locations, [82, 43, 86, 35]);
    }

    #[test]
    fn test_seeds_to_seed_ranges() {
        let (seeds, _) = parse_input(SAMPLE);

        assert_eq!(seeds_to_seed_ranges(&seeds), [(79, 93), (55, 68)]);
    }

    #[test]
    fn test_part_1_sample() {
        let (seeds, maps) = parse_input(SAMPLE);

        let locations = seeds_to_location(&seeds, &maps);
        assert_eq!(locations, [82, 43, 86, 35]);
    }

    #[test]
    fn test_part_2_sample() {
        let (seeds, maps) = parse_input(SAMPLE);

        assert_eq!(lowest_location_from_seed_ranges(&seeds, &maps), 46);

        let (seeds2, maps2) = parse_input(SAMPLE2);

        assert_eq!(lowest_location_from_seed_ranges(&seeds2, &maps2), 1000);
    }
}

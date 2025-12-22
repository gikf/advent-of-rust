use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2024/day09/src/input.txt");
    let memory = parse_input(&fs::read_to_string(input).unwrap());

    let part1_memory = memory.clone();
    let part1 = std::time::Instant::now();

    let compressed = compress(part1_memory);
    println!("Part 1");
    println!("Filesystem checkum: {:?}", filesystem_checksum(&compressed));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let compressed = compress_whole_files(memory);
    println!("Part 2");
    println!(
        "Whole files compression filesystem checkum: {:?}",
        filesystem_checksum(&compressed)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> Vec<Vec<Option<usize>>> {
    let mut memory = Vec::with_capacity(input.len());

    for offset in 0_usize.. {
        let file_offset = offset * 2;
        let free_offset = file_offset + 1;

        match input.get(file_offset..file_offset + 1) {
            Some(file_length) => memory.push(vec![Some(offset); file_length.parse().unwrap()]),
            None => unreachable!(),
        }

        match input.get(free_offset..free_offset + 1) {
            Some(free_space) => {
                memory.push(vec![None; free_space.parse().unwrap()]);
            }
            None => {
                break;
            }
        }
    }

    memory
}

fn compress(mut memory: Vec<Vec<Option<usize>>>) -> Vec<Vec<Option<usize>>> {
    let mut free_space_pointer = 1_usize;
    let memory_length = memory.len();
    let last_file_pointer = memory_length
        - if memory_length.is_multiple_of(2) {
            2
        } else {
            1
        };
    let mut free_block_pointer = 0_usize;

    for file_pointer in (0..=last_file_pointer).rev().step_by(2) {
        if free_space_pointer >= file_pointer {
            break;
        }
        while let Some(file_block) = memory[file_pointer].pop() {
            if memory[free_space_pointer].len() > free_block_pointer {
                memory[free_space_pointer][free_block_pointer] = file_block;
            } else {
                free_space_pointer += 2;
                while memory[free_space_pointer].is_empty() {
                    free_space_pointer += 2;
                }
                memory[free_space_pointer][0] = file_block;
                free_block_pointer = 0;
            }
            free_block_pointer += 1;
        }
    }
    memory
}

fn compress_whole_files(mut memory: Vec<Vec<Option<usize>>>) -> Vec<Vec<Option<usize>>> {
    let memory_length = memory.len();
    let last_file_pointer = memory_length
        - if memory_length.is_multiple_of(2) {
            2
        } else {
            1
        };
    let mut space_pointers_with_size: Vec<(usize, usize)> = (1..memory_length)
        .step_by(2)
        .map(|space_pointer| (space_pointer, memory[space_pointer].len()))
        .collect();

    for file_pointer in (1..=last_file_pointer).rev().step_by(2) {
        let file_size = memory[file_pointer].len();
        for (space_pointer, space_left) in space_pointers_with_size.iter_mut() {
            if *space_pointer > file_pointer {
                break;
            }
            if *space_left >= file_size {
                let first_empty_block = memory[*space_pointer]
                    .iter()
                    .position(|space| space.is_none())
                    .unwrap();

                for (file_block_pointer, block_pointer) in (0..file_size).zip(first_empty_block..) {
                    memory[*space_pointer][block_pointer] =
                        memory[file_pointer][file_block_pointer];
                    memory[file_pointer][file_block_pointer] = None;
                }

                *space_left -= file_size;
                break;
            }
        }
    }
    memory
}

fn filesystem_checksum(memory: &[Vec<Option<usize>>]) -> usize {
    let mut position = 0;
    let mut checksum = 0;
    for chunk in memory {
        for block in chunk {
            if let Some(id) = block {
                checksum += position * id;
            }
            position += 1;
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";
    const SAMPLE2: &str = "12345";

    #[test]
    fn test_filesystem_checksum() {
        let memory = [
            Vec::from([Some(0), Some(0)]),
            Vec::from([Some(9), Some(9), Some(8)]),
            Vec::from([Some(1), Some(1), Some(1)]),
            Vec::from([Some(8), Some(8), Some(8)]),
            Vec::from([Some(2)]),
            Vec::from([Some(7), Some(7), Some(7)]),
            Vec::from([Some(3), Some(3), Some(3)]),
            Vec::from([Some(6)]),
            Vec::from([Some(4), Some(4)]),
            Vec::from([Some(6)]),
            Vec::from([Some(5), Some(5), Some(5), Some(5)]),
            Vec::from([Some(6)]),
            Vec::from([Some(6)]),
        ];

        assert_eq!(filesystem_checksum(&memory), 1928);
    }

    #[test]
    fn test_compress() {
        let memory = parse_input(SAMPLE2);
        let compressed = compress(memory);

        assert_eq!(
            compressed,
            Vec::from([
                Vec::from([Some(0)]),
                Vec::from([Some(2), Some(2)]),
                Vec::from([Some(1), Some(1), Some(1)]),
                Vec::from([Some(2), Some(2), Some(2), None]),
                Vec::from([]),
            ])
        )
    }

    #[test]
    fn test_part_1_sample() {
        let memory = parse_input(SAMPLE);
        let compressed = compress(memory);

        let checksum = filesystem_checksum(&compressed);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_part_2_sample() {
        let memory = parse_input(SAMPLE);
        let compressed = compress_whole_files(memory);

        let checksum = filesystem_checksum(&compressed);
        assert_eq!(checksum, 2858);
    }
}

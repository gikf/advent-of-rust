use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day16/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let packets = parse_packets(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of version numbers: {:?}",
        sum_packet_versions(&packets)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Value of evaluated transmission: {:?}",
        evaluate_bits(&contents)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_packets(input: &str) -> Vec<(usize, usize, usize)> {
    let bits = hex_to_binary(input);
    parse_packet(&bits)
}

fn hex_to_binary(hex: &str) -> String {
    let mut bits = Vec::new();
    hex.chars().for_each(|hex| {
        bits.push(match hex {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => unimplemented!(),
        })
    });

    bits.join("")
}

fn parse_packet(binary_packet: &str) -> Vec<(usize, usize, usize)> {
    let (version, rest) = binary_packet.split_at(3);
    let (packet_type, rest) = rest.split_at(3);
    let version = usize::from_str_radix(version, 2).unwrap();
    let packet_type = usize::from_str_radix(packet_type, 2).unwrap();

    if packet_type == 4 {
        let mut binary = Vec::new();
        let mut pointer = 0;
        loop {
            let start = pointer;
            let end = start + 5;
            let bits = &rest[start..end];
            binary.push(&bits[1..]);
            let is_last = bits.starts_with('0');

            if is_last {
                break;
            }
            pointer += 5;
        }
        let rest = &rest[pointer + 5..];

        let value = usize::from_str_radix(&binary.join(""), 2).unwrap();
        let mut result = vec![(version, packet_type, value)];
        if rest.chars().all(|c| c == '0') {
            result
        } else {
            let other_packets = parse_packet(rest);
            result.extend_from_slice(&other_packets);
            result
        }
    } else {
        let mut result = vec![(version, packet_type, 0)];
        let (length_type, rest) = rest.split_at(1);
        if length_type == "0" {
            let (_, rest) = rest.split_at(15);
            result.extend_from_slice(&parse_packet(rest));
        } else if length_type == "1" {
            let (_, rest) = rest.split_at(11);
            result.extend_from_slice(&parse_packet(rest));
        }
        result
    }
}

fn evaluate_bits(hex: &str) -> usize {
    let bits = hex_to_binary(hex);
    let (packet, _) = parse_next(&bits);

    packet.2
}

fn parse_next(binary_packet: &str) -> ((usize, usize, usize), &str) {
    let (version, rest) = binary_packet.split_at(3);
    let (packet_type, rest) = rest.split_at(3);
    let version = usize::from_str_radix(version, 2).unwrap();
    let packet_type = usize::from_str_radix(packet_type, 2).unwrap();

    if packet_type == 4 {
        let mut binary = Vec::new();
        let mut pointer = 0;
        loop {
            let start = pointer;
            let end = start + 5;
            let bits = &rest[start..end];
            binary.push(&bits[1..]);
            let is_last_group = bits.starts_with('0');

            if is_last_group {
                break;
            }
            pointer += 5;
        }
        let rest = &rest[pointer + 5..];

        let value = usize::from_str_radix(&binary.join(""), 2).unwrap();
        let result = (version, packet_type, value);
        (result, rest)
    } else {
        let mut packet = (version, packet_type, 0);
        let (length_type, mut rest) = rest.split_at(1);
        let mut sub_packets = Vec::new();
        if length_type == "0" {
            let (sub_packets_size, r) = rest.split_at(15);
            let sub_packets_size = usize::from_str_radix(sub_packets_size, 2).unwrap();
            let mut size_so_far = 0;
            rest = r;

            while size_so_far < sub_packets_size {
                let (sub_packet, next_rest) = parse_next(rest);
                sub_packets.push(sub_packet);
                size_so_far += rest.len() - next_rest.len();
                rest = next_rest;
            }
        } else if length_type == "1" {
            let (sub_packets_count, r) = rest.split_at(11);
            let sub_packets_count = usize::from_str_radix(sub_packets_count, 2).unwrap();
            rest = r;
            for _ in 0..sub_packets_count {
                let (sub_packet, next_rest) = parse_next(rest);
                sub_packets.push(sub_packet);
                rest = next_rest;
            }
        }
        let sub_values: Vec<_> = sub_packets.iter().map(|sub| sub.2).collect();
        packet.2 = match packet_type {
            0 => sub_values.iter().sum(),
            1 => sub_values.iter().product(),
            2 => *sub_values.iter().min().unwrap(),
            3 => *sub_values.iter().max().unwrap(),
            5 => {
                if sub_values[0] > sub_values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub_values[0] < sub_values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_values[0] == sub_values[1] {
                    1
                } else {
                    0
                }
            }
            _ => unimplemented!(),
        };
        (packet, rest)
    }
}

fn sum_packet_versions(packets: &[(usize, usize, usize)]) -> usize {
    packets.iter().map(|(version, _, _)| *version).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "D2FE28";
    const SAMPLE2: &str = "38006F45291200";
    const SAMPLE3: &str = "EE00D40C823060";
    const SAMPLE4: &str = "8A004A801A8002F478";
    const SAMPLE5: &str = "620080001611562C8802118E34";
    const SAMPLE6: &str = "C0015000016115A2E0802F182340";
    const SAMPLE7: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_parse_packets() {
        assert_eq!(parse_packets(SAMPLE1), [(6, 4, 2021)]);
        assert_eq!(parse_packets(SAMPLE2), [(1, 6, 0), (6, 4, 10), (2, 4, 20),]);
        assert_eq!(
            parse_packets(SAMPLE3),
            [(7, 3, 0), (2, 4, 1), (4, 4, 2), (1, 4, 3),]
        );
    }

    #[test]
    fn test_part_1_sample() {
        let packets4 = parse_packets(SAMPLE4);
        assert_eq!(sum_packet_versions(&packets4), 16);

        let packets5 = parse_packets(SAMPLE5);
        assert_eq!(sum_packet_versions(&packets5), 12);

        let packets6 = parse_packets(SAMPLE6);
        assert_eq!(sum_packet_versions(&packets6), 23);

        let packets7 = parse_packets(SAMPLE7);
        assert_eq!(sum_packet_versions(&packets7), 31);
    }

    const SAMPLE8: &str = "C200B40A82";
    const SAMPLE9: &str = "04005AC33890";
    const SAMPLE10: &str = "880086C3E88112";
    const SAMPLE11: &str = "CE00C43D881120";
    const SAMPLE12: &str = "D8005AC2A8F0";
    const SAMPLE13: &str = "F600BC2D8F";
    const SAMPLE14: &str = "9C005AC2F8F0";
    const SAMPLE15: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_part_2_sample() {
        assert_eq!(evaluate_bits(SAMPLE8), 3);
        assert_eq!(evaluate_bits(SAMPLE9), 54);
        assert_eq!(evaluate_bits(SAMPLE10), 7);
        assert_eq!(evaluate_bits(SAMPLE11), 9);
        assert_eq!(evaluate_bits(SAMPLE12), 1);
        assert_eq!(evaluate_bits(SAMPLE13), 0);
        assert_eq!(evaluate_bits(SAMPLE14), 0);
        assert_eq!(evaluate_bits(SAMPLE15), 1);
    }
}

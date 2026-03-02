use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::path::Path;

type Diagram<const N: usize> = ([u8; 11], [[u8; N]; 4]);

fn main() {
    let input = Path::new("2021/day23/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let d = parse_diagram(&contents);

    let part1 = std::time::Instant::now();
    let diagram1 = rooms_part1(&d);
    println!("Part 1");
    println!(
        "Least energy to organize amphipods: {:?}",
        lowest_energy(([b'.'; 11], diagram1))
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    let diagram2 = rooms_part2(&d);
    println!("Part 2");
    println!(
        "Least energy to organize more amphipods: {:?}",
        lowest_energy(([b'.'; 11], diagram2))
    );
    println!("In {:?}", part2.elapsed());
}

const COSTS: [usize; 4] = [1, 10, 100, 1000];
const ROOM_POSITIONS: [usize; 4] = [2, 4, 6, 8];
const EMPTY: u8 = b'.';
const A: u8 = b'A';

fn parse_diagram(input: &str) -> Vec<Vec<&u8>> {
    input
        .lines()
        .skip(2)
        .take(2)
        .map(|line| line.as_bytes().iter().collect())
        .collect()
}

fn rooms_part1(d: &[Vec<&u8>]) -> [[u8; 2]; 4] {
    let mut rooms = [[0; 2]; 4];
    for (room_id, room) in ROOM_POSITIONS.iter().enumerate() {
        rooms[room_id][0] = *d[0][*room + 1];
        rooms[room_id][1] = *d[1][*room + 1];
    }
    rooms
}

fn rooms_part2(d: &[Vec<&u8>]) -> [[u8; 4]; 4] {
    let part2_extra = [[b'D', b'D'], [b'C', b'B'], [b'B', b'A'], [b'A', b'C']];
    let mut rooms = [[0; 4]; 4];
    for (room_id, room) in ROOM_POSITIONS.iter().enumerate() {
        rooms[room_id][0] = *d[0][*room + 1];
        rooms[room_id][1] = part2_extra[room_id][0];
        rooms[room_id][2] = part2_extra[room_id][1];
        rooms[room_id][3] = *d[1][*room + 1];
    }
    rooms
}

fn are_rooms_organized<const N: usize>((_, rooms): &Diagram<N>) -> bool {
    rooms
        .iter()
        .zip("ABCD".bytes())
        .all(|(room, amphi)| room.iter().all(|a| *a == amphi))
}

fn make_move<const N: usize>(
    (mut hallway, mut rooms): Diagram<N>,
    position: usize,
    target_room: usize,
    position_in_room: usize,
) -> (usize, Diagram<N>) {
    let amphipod_id = (if hallway[position] == EMPTY {
        rooms[target_room][position_in_room]
    } else {
        hallway[position]
    } - A) as usize;
    let room_position = ROOM_POSITIONS[target_room];
    let steps_outside_of_room = room_position.abs_diff(position) + 1;
    let move_cost = (position_in_room + steps_outside_of_room) * COSTS[amphipod_id];

    (hallway[position], rooms[target_room][position_in_room]) =
        (rooms[target_room][position_in_room], hallway[position]);

    (move_cost, (hallway, rooms))
}

fn possible_moves<const N: usize>((hallway, rooms): Diagram<N>) -> Vec<(usize, Diagram<N>)> {
    let mut moves = Vec::new();

    for position in 0..hallway.len() {
        let in_spot = hallway[position];
        if in_spot == EMPTY {
            continue;
        }

        let room_id = (in_spot - A) as usize;
        let room_target = ROOM_POSITIONS[room_id];

        let (start, end) = if position > room_target {
            (room_target, position)
        } else {
            (position + 1, room_target + 1)
        };

        if (start..end).any(|p| hallway[p] != EMPTY) {
            continue;
        }

        if let Some(position_in_room) = (0..N).take_while(|&i| rooms[room_id][i] == EMPTY).last()
            && ((position_in_room + 1)..N).all(|in_room| rooms[room_id][in_room] == in_spot)
        {
            moves.push(make_move(
                (hallway, rooms),
                position,
                room_id,
                position_in_room,
            ));
        }
    }

    for room_id in 0..4 {
        if let Some(position_in_room) = (0..N).find(|&i| rooms[room_id][i] != EMPTY) {
            let room_position = ROOM_POSITIONS[room_id];
            let valid_moves = (room_position..hallway.len())
                .take_while(|&hall_id| hallway[hall_id] == EMPTY)
                .chain(
                    (0..room_position)
                        .rev()
                        .take_while(|&hall_id| hallway[hall_id] == EMPTY),
                )
                .filter(|hall_id| !ROOM_POSITIONS.contains(hall_id))
                .map(|hall_id| make_move((hallway, rooms), hall_id, room_id, position_in_room));

            moves.extend(valid_moves);
        }
    }

    moves
}

fn lowest_energy<const N: usize>(diagram: Diagram<N>) -> usize {
    let mut state_to_energy = HashMap::new();
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, diagram)));

    while let Some(Reverse((energy, state))) = queue.pop() {
        if are_rooms_organized(&state) {
            return energy;
        }
        if let Some(prev_cost) = state_to_energy.get(&state)
            && energy > *prev_cost
        {
            continue;
        }

        for (move_cost, next_state) in possible_moves(state) {
            let next_cost = move_cost + energy;
            if let Some(prev_cost) = state_to_energy.get(&next_state)
                && *prev_cost <= next_cost
            {
                continue;
            }
            state_to_energy.insert(next_state, next_cost);
            queue.push(Reverse((next_cost, next_state)));
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn test_part_1_sample() {
        let d = parse_diagram(SAMPLE);
        let diagram = rooms_part1(&d);
        assert_eq!(lowest_energy(([b'.'; 11], diagram)), 12521);
    }

    #[test]
    fn test_part_2_sample() {
        let d = parse_diagram(SAMPLE);
        let diagram = rooms_part2(&d);
        assert_eq!(lowest_energy(([b'.'; 11], diagram)), 44169);
    }
}

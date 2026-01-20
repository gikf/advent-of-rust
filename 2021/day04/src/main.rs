use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("2021/day04/src/input.txt");
    let (numbers, boards) = parse_bingo(&fs::read_to_string(input).unwrap());

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!("Final score of board: {:?}", win_bingo(&numbers, &boards));
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Score of last winning board: {:?}",
        lose_bingo(&numbers, &boards)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_bingo(input: &str) -> (Vec<usize>, Vec<[[usize; 5]; 5]>) {
    let mut lines = input.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    lines.next();

    let mut boards = Vec::new();
    let mut board = [[0; 5]; 5];
    for (index, line) in lines.chain([""]).enumerate() {
        if line.is_empty() {
            boards.push(board);
            board = [[0; 5]; 5];
            continue;
        }
        let split = line.split_ascii_whitespace();
        for (col_no, num) in split.enumerate() {
            board[index % 6][col_no] = num.parse::<usize>().unwrap();
        }
    }

    (numbers, boards)
}

fn win_bingo(numbers: &[usize], boards: &[[[usize; 5]; 5]]) -> usize {
    let mut marked = vec![[[false; 5]; 5]; boards.len()];

    for number in numbers {
        for (board_no, _) in boards.iter().enumerate() {
            mark_board(&boards[board_no], &mut marked[board_no], *number);
        }

        for (board_no, marked_board) in marked.iter().enumerate() {
            let has_winning_row = marked_board
                .iter()
                .any(|marked_row| marked_row.iter().all(|v| *v));
            let has_winning_col = (0..(marked_board.len() - 1))
                .any(|index| marked_board.iter().all(|row| row[index]));

            if has_winning_row || has_winning_col {
                return score_board(&boards[board_no], marked_board, *number);
            }
        }
    }
    0
}

fn mark_board(board: &[[usize; 5]; 5], marked_board: &mut [[bool; 5]; 5], number: usize) {
    for (board_row, row) in board.iter().enumerate() {
        for (board_col, num) in row.iter().enumerate() {
            if *num == number {
                marked_board[board_row][board_col] = true;
            }
        }
    }
}

fn lose_bingo(numbers: &[usize], boards: &[[[usize; 5]; 5]]) -> usize {
    let mut marked = vec![[[false; 5]; 5]; boards.len()];
    let mut boards_in_play: Vec<_> = (0..boards.len()).collect();
    for number in numbers {
        for board_no in boards_in_play.iter() {
            mark_board(&boards[*board_no], &mut marked[*board_no], *number);
        }

        let mut to_remove = Vec::new();
        for board_no in boards_in_play.iter() {
            let marked_board = marked[*board_no];
            let has_winning_row = marked_board
                .iter()
                .any(|marked_row| marked_row.iter().all(|v| *v));
            let has_winning_col = (0..(marked_board.len() - 1))
                .any(|index| marked_board.iter().all(|row| row[index]));

            if has_winning_row || has_winning_col {
                if boards_in_play.len() == 1 {
                    return score_board(&boards[*board_no], &marked_board, *number);
                } else {
                    to_remove.push(*board_no);
                    continue;
                }
            }
        }
        boards_in_play.retain(|num| !to_remove.contains(num));
    }
    0
}

fn score_board(board: &[[usize; 5]; 5], marked: &[[bool; 5]; 5], last_number: usize) -> usize {
    let sum_of_unmarked: usize = board
        .iter()
        .enumerate()
        .map(|(row_no, row)| {
            row.iter()
                .enumerate()
                .map(|(col_no, num)| if !marked[row_no][col_no] { *num } else { 0 })
                .sum::<usize>()
        })
        .sum();

    sum_of_unmarked * last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_bingo() {
        let (numbers, boards) = parse_bingo(SAMPLE);

        assert_eq!(
            numbers,
            [
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            boards[0],
            [
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ]
        );
        assert_eq!(
            boards[2],
            [
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ]
        );
    }
    #[test]
    fn test_part_1_sample() {
        let (numbers, boards) = parse_bingo(SAMPLE);

        assert_eq!(win_bingo(&numbers, &boards), 4512);
    }

    #[test]
    fn test_part_2_sample() {
        let (numbers, boards) = parse_bingo(SAMPLE);

        assert_eq!(lose_bingo(&numbers, &boards), 1924);
    }
}

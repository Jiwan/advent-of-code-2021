use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Board<T> = Vec<Vec<(i32, T)>>;

fn parse<MetaData>(path: &'static str) -> (Vec<i32>, Vec<Board<MetaData>>)
where
    MetaData: Default + Clone,
{
    let file = File::open(path).expect("Couldn't open data file");
    let mut lines = BufReader::new(file).lines();

    let drawn_numbers = lines.next().unwrap().unwrap().split(",").map(|number| number.parse::<i32>().unwrap()).collect();

    lines.next();

    let all_row: Vec<Vec<(i32, MetaData)>> = lines
        .map(|line| {
            line.unwrap()
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|number| (number.parse::<i32>().unwrap(), MetaData::default()))
                .collect()
        })
        .collect();

    let boards = all_row.split(|v| v.is_empty()).map(|v| v.to_vec()).collect();

    (drawn_numbers, boards)
}

fn update_board(number: i32, board: &mut Board<bool>) {
    for row in board {
        row.iter_mut()
            .filter(|(x, _)| *x == number)
            .for_each(|(_, seen)| *seen = true);
    }
}

fn update_boards(number: i32, boards: &mut Vec<Board<bool>>) {
    for board in boards {
        update_board(number, board);
    }
}

fn scan_for_completion(board: &Board<bool>) -> bool {
    for row in board {
        if row.iter().all(|(_, seen)| *seen) {
            return true;
        }
    }

    let row_length = board[0].len();

    for column in 0..row_length {
        let mut matching_rows = 0usize;

        for row in board {
            if row[column].1 {
                matching_rows += 1;
            }
        }

        if matching_rows == board.len() {
            return true;
        }
    }

    return false;
}

fn scan_first_completed_board(boards: &Vec<Board<bool>>) -> Option<usize> {
    for (index, board) in boards.iter().enumerate() {
        if scan_for_completion(&board) {
            return Some(index);
        }
    }

    None
}

fn compute_board_score(board: &Board<bool>) -> i32 {
    board.iter().fold(0, |sum, row| {
        let row_sum: i32 = row.iter().filter(|(_, seen)| !seen).map(|(x, _)| x).sum();
        sum + row_sum
    })
}

fn part1() {
    let (drawn_numbers, mut boards) = parse::<bool>("data/part1.txt");

    for number in drawn_numbers {
        update_boards(number, &mut boards);
        if let Some(board_index) = scan_first_completed_board(&boards) {
            let score = compute_board_score(&boards[board_index]);
            println!("{} * {} == {}", score, number, score * number);
            break;
        }
    }
}

fn part2() {
    let (drawn_numbers, mut boards) = parse::<bool>("data/part1.txt");

    let mut boards_left: Vec<usize> = (0..boards.len()).collect();
    let mut last_board = None;

    for number in drawn_numbers {
        boards_left.retain(|board_left| {
            let unfinished_board = &mut boards[*board_left];
            update_board(number, unfinished_board);

            !scan_for_completion(unfinished_board)
        });

        if boards_left.len() == 1 {
            last_board = Some(boards_left[0]);
        } else if boards_left.is_empty() {
            let score = compute_board_score(&boards[last_board.unwrap()]);
            println!("{} * {} == {}", score, number, score * number);
            break;
        }
    }
}

fn main() {
    part1();
    part2();
}

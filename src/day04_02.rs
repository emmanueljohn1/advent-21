use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use array2d::Array2D;
use itertools::Itertools;

const MARKED: i32 = -1;

fn mark_item(board: &Array2D<i32>, item: i32) -> Array2D<i32> {
    let mut new_board = Array2D::from_rows(&board.as_rows());
    for row in 0..new_board.num_rows() {
        for col in 0..new_board.num_columns() {
            if item == new_board[(row, col)] {
                new_board[(row, col)] = MARKED;
            }
        }
    }

    return new_board;
}

fn is_winning_board(board: &Array2D<i32>) -> bool {
    let rows = board.as_rows();
    let cols = board.as_columns();

    return rows.iter().any(|row| row.iter().all(|val| val == &MARKED))
        || cols.iter().any(|row| row.iter().all(|val| val == &MARKED));
}

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_04.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let (numbers, mut data) = parse_data(buffered);

    let mut winning_board: Array2D<i32> = Array2D::from_rows(&vec![]);
    let mut winning_num = 0; // Assume there's always going to be a winning value
    for num in numbers {
        data = data.iter().filter_map(|arr|
            match !is_winning_board(arr) {
                true => Some(mark_item(arr, num)),
                false => None
            }
        ).collect();
        let winning_array = data.iter().find(|array| is_winning_board(array));
        if winning_array.is_some() {
            winning_num = num;
            winning_board = Array2D::from_rows(&winning_array.unwrap().as_rows());
        }
    }

    let sum: i32 = winning_board.as_row_major().iter().filter(|&val| val != &MARKED).sum();
    let score: i64 = (sum * winning_num) as i64;
    println!("Winning board: ({}) {:?}, \nsum {}, \nscore {}", winning_num, winning_board.as_rows(), sum, score);

    Ok(())
}

fn parse_data(buffered: BufReader<File>) -> (Vec<i32>, Vec<Array2D<i32>>) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let mut data: Vec<Array2D<i32>> = Vec::new();

    for (idx, line) in buffered.lines().enumerate() {
        let line = line.unwrap();
        if line.trim() == "" { continue; }
        if idx == 0 {
            numbers = string_to_vec(&line.trim().to_string(),",");
        } else {
            lines.push(line);
        }
    }

    let vec_rows = lines.iter().fold(vec![], |mut acc, val| {
        acc.push(string_to_vec(&val.trim().to_string(), " "));
        acc
    });

    for chunk in vec_rows.chunks(5) {
        data.push(Array2D::from_rows(chunk))
    }

    return (numbers, data);
}

fn string_to_vec(line: &String, delim: &str) -> Vec<i32> {
    return line
        .split(delim)
        .filter(|val| val.trim() != "")
        .map(|val| val.trim().to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}

#[cfg(test)]
mod tests {
    use crate::utils::print_vec;
    use super::*;

    #[test]
    fn test_mark_item() {
        let mut board = Array2D::filled_with(0, 5, 5);
        board[(1, 4)] = 30;
        board[(2, 3)] = 35;
        board = mark_item(&mut board, 30);
        board = mark_item(&mut board, 35);
        assert_eq!(board[(1, 4)], -1);
        assert_eq!(board[(2, 3)], -1);
        assert_eq!(board[(1, 1)], 0);
    }

    #[test]
    fn test_is_winning_board() {
        let mut winning_col =
            Array2D::from_rows(&vec![vec![-1, 2, 3], vec![-1, 5, 6], vec![-1, 5, 6]]);
        let winning_row =
            Array2D::from_rows(&vec![vec![1, 2, 3], vec![-1, -1, -1], vec![7, 8, 9]]);

        let not_a_winner =
            Array2D::from_rows(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);

        // one column filled
        assert_eq!(is_winning_board(&winning_col), true);

        // one row filled
        assert_eq!(is_winning_board(&winning_row), true);

        // one row filled
        assert_eq!(is_winning_board(&not_a_winner), false);
    }
}





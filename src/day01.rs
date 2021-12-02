use std::io::Error;
use crate::utils::{print_vec, read_file};

fn sliding_window_sums(data: &Vec<i32>) -> Vec<i32> {
    let data = data.to_vec();
    let mut result = vec![(&data[0..3]).iter().sum()];

    for i in 1..data.len() - 2 {
        result.push((&data[i..i + 3]).iter().sum())
    }

    result
}

fn num_depths(data: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            count = count + 1;
        }
    }

    count
}

pub fn run() -> Result<(), Error> {
    let path = "test_inputs/input_01.txt";
    let data = read_file(path);
    let transformed = sliding_window_sums(&data);
    let count = num_depths(&transformed);

    println!("count {}", count);
    Ok(())
}
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> Vec<i32> {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    let mut data = Vec::new();

    for line in buffered.lines() {
        data.push(line.unwrap().parse::<i32>().unwrap());
    }

    return data;
}

pub fn print_vec (vec: &Vec<u32>) {
    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }
}
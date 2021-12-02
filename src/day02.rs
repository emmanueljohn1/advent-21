use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::utils::{print_vec};

pub fn run() -> Result<(), Error> {
    let path = "test_inputs/input_02.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let (x, y, aim) = buffered.lines().fold((0, 0, 0), |acc, val| {
        let line = val.unwrap();
        let mut split = line.split(" ");
        let vec: Vec<&str> = split.collect();

        let (x, y, aim) = acc;
        let result = if vec[0] == "forward" {
            let dist = vec[1].parse::<i64>().unwrap();
            (x + dist, y + (aim * dist), aim)
        } else if vec[0] == "down" {
            let dist = vec[1].parse::<i64>().unwrap();
            (x, y, aim + dist)
        } else if vec[0] == "up" {
            let dist = vec[1].parse::<i64>().unwrap();
            (x, y, aim - dist)
        } else { (x, y, aim) };

        return result;
    });

    // print_vec(&data);
    println!("x {}, y {}, aim {}, product {}", x, y, aim, x * y);
    Ok(())
}
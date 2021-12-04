use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::utils::print_vec;

pub fn run() -> Result<(), Error>{
    let path = "test_inputs/input_03.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);

    let data = buffered.lines().fold(vec![], |acc, val| {
        let mut result = acc;
        let line = val.unwrap();

        if result.len() == 0 {
            result = vec![(0, 0); line.len()]
        }

        for (i, ch) in line.chars().enumerate() {
            let (x, y) = result[i];
            if ch == '1' { result[i] = (x, y+1); } else { result[i] = (x+1, y); }
        }

        return result;
    });

    let mut gamma_rate_bin = String::from("");
    let mut epsilon_rate_bin = String::from("");

    for i in 0..data.len() {
        // println!("{} {}", data[i].0, data[i].1);
        if data[i].0 > data[i].1 {
            gamma_rate_bin.push('0');
            epsilon_rate_bin.push('1');

        } else {
            gamma_rate_bin.push('1');
            epsilon_rate_bin.push('0');
        }
    }

    let gamma = isize::from_str_radix(&gamma_rate_bin, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_rate_bin, 2).unwrap();

    println!("gamma {}, epsilon {}, product {}", gamma, epsilon, gamma * epsilon);

    Ok(())
}
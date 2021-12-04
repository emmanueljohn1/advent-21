use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::utils::print_vec;

fn bit_frequencies(raw_data: &mut Vec<String>) -> Vec<(i64, i64)> {
    return raw_data.iter().fold(vec![], |acc, line| {
        let mut result = acc;
        if result.len() == 0 {
            result = vec![(0, 0); line.len()];
        }

        for (i, ch) in line.chars().enumerate() {
            let (x, y) = result[i];
            if ch == '1' { result[i] = (x, y + 1); } else { result[i] = (x + 1, y); }
        }

        return result;
    });
}

pub fn run() -> Result<(), Error> {
    let path = "test_inputs/input_03.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let mut raw_data: Vec<String> = Vec::new();

    for element in buffered.lines() {
        raw_data.push(String::from(element.unwrap()));
    };

    let mut o2_ratings: Vec<String> = raw_data.to_vec();
    let mut co2_ratings: Vec<String> = raw_data.to_vec();

    for i in 0..raw_data[0].len() {
        let o2_bit_frequencies = bit_frequencies(&mut o2_ratings);
        let co2_bit_frequencies = bit_frequencies(&mut co2_ratings);
        if o2_ratings.len() > 1 {
            o2_ratings = o2_ratings.into_iter().filter(|val| {
                let (freq0, freq1) = o2_bit_frequencies[i];
                let most_common = if freq1 >= freq0 { '1' } else { '0' };
                if most_common == val.chars().nth(i).unwrap() { return true; }
                return false;
            }).collect();
        }

        if co2_ratings.len() > 1 {
            co2_ratings = co2_ratings.into_iter().filter(|val| {
                let (freq0, freq1) = co2_bit_frequencies[i];
                let least_common = if freq0 <= freq1 { '0' } else { '1' };
                if least_common == val.chars().nth(i).unwrap() { return true; }
                return false;
            }).collect();
        }
    }

    let o2 = isize::from_str_radix(&o2_ratings[0], 2).unwrap();
    let co2 = isize::from_str_radix(&co2_ratings[0], 2).unwrap();
    println!("o2 {}, c02 {}, product {}", o2, co2, o2 * co2);
    Ok(())
}
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use itertools::Itertools;
use rayon::prelude::*;

const DAYS: i32 = 256;

fn generate_children_count(days_left: i32, initial_age: i32, table: &mut HashMap<(i32, i32), usize>) -> usize {
    let time_until_rep:i32 = initial_age + 1;
    let children_count: usize = (((days_left - time_until_rep) as f32 / 7_f32).floor() + 1_f32) as usize;
    let mut children_count: usize = max(children_count, 0);

    let mut total_count = children_count;

    for i in 0..children_count {
        let key = (days_left - time_until_rep - (i * 7) as i32, 8);
        if table.get(&key) != None {
            total_count +=table.get(&key).unwrap();
        }else {
            let local_count = generate_children_count(days_left - time_until_rep - (i * 7) as i32, 8, table);
            table.insert(key, local_count);
            total_count +=local_count;
        }
    }

    return total_count;
}

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_06.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);

    let mut fishes: Vec<(i32, i32)> = buffered.lines()
        .map(|line| string_to_vec(&line.unwrap(), ","))
        .flatten()
        .map(|age| (DAYS, age))
        .collect();

    let mut table: HashMap<(i32, i32), usize> = HashMap::new();
    let mut count = fishes.len();

    for fish in fishes {
        let (days_left, initial_age) = fish;
        if table.get(&(days_left, initial_age)) != None {
            count += table.get(&(days_left, initial_age)).unwrap();
        } else {
            let local_count = generate_children_count(days_left, initial_age, &mut table);
            table.insert((days_left, initial_age), local_count);
            count += local_count;
        }
    }
    println!("count {}", count);

    Ok(())
}

fn string_to_vec(line: &String, delim: &str) -> Vec<i32> {
    return line
        .split(delim)
        .filter(|val| val.trim() != "")
        .map(|val| val.trim().to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}




// use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use itertools::{max, min};

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_07.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);

    let mut nums: Vec<i32> = buffered.lines()
        .map(|line| string_to_vec(&line.unwrap(), ","))
        .flatten()
        .collect();

    // Part 1
    // let mut min_fuel = 100000000;
    //
    // for i in &nums {
    //     let mut sum = 0;
    //     for j in &nums {
    //         sum += (i - j).abs();
    //     }
    //
    //     min_fuel = min(sum, min_fuel);
    // }

    // part 2
    let mut min_fuel: i64 = 100000000000;
    let mut positions = (*min(&nums).unwrap()..*max(&nums).unwrap() + 1).collect::<Vec<i32>>();
    // println!("{:?}", positions);


    for j in positions {
        let mut sum: i64 = 0;
        for i in &nums {
            let diff = (i - j).abs();
            let mut cost: i64 =( (diff * (diff+1)) / 2) as i64; // After looking at Cale's answer
            // How I was computing it before :
            // for x in 1..(diff + 1) {
            //     cost += x as i64;
            // }

            sum += cost;
        }

        min_fuel = min(vec![sum, min_fuel]).unwrap();
    }
    println!("min fuel {}", min_fuel);

    Ok(())
}

fn string_to_vec(line: &String, delim: &str) -> Vec<i32> {
    return line
        .split(delim)
        .filter(|val| val.trim() != "")
        .map(|val| val.trim().to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
}
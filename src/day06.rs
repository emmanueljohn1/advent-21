use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const DAYS: i32 = 80;
fn generate_children(days_left: i32, initial_age: i32) -> Vec<(i32, i32)> {
    let time_until_rep = initial_age + 1;
    let children_count: i32 = (((days_left - time_until_rep) as f32 / 7_f32).floor() + 1_f32) as i32;
    let children_count: i32 = max(children_count, 0);

    let mut children: Vec<(i32, i32)> = Vec::new();

    for i in 0..children_count {
        children.push((days_left - time_until_rep - (i * 7), 8));
    }
    return children;
}

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/example_06.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let mut fishes: VecDeque<(i32, i32)> = buffered.lines()
        .map(|line| string_to_vec(&line.unwrap(), ","))
        .flatten()
        .map(|age| (DAYS, age ))
        .collect();

    let mut count = fishes.len();

    loop {
        if fishes.len() == 0 { break; }

        let (days_left, initial_age) = fishes.pop_front().unwrap();
        // println!(" days {}, age {}", days_left, initial_age);
        let children = generate_children(days_left, initial_age);
        // println!(" children {:?}", children);
        count = count + children.len();
        for child in children {
            fishes.push_back(child);
        }
    }

    println!("# fishes {}", count);
    Ok(())
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
    use super::*;

    #[test]
    fn test_generate_children() {
        let children = generate_children(18, 3);
        assert_eq!(children.len(), 3);
        assert_eq!(children[0], (14, 8));
        assert_eq!(children[1], (7, 8));
        assert_eq!(children[2], (0, 8));

        let children = generate_children(14, 8);
        assert_eq!(children.len(), 1);

        let children = generate_children(7, 8);
        assert_eq!(children.len(), 0);

        let children = generate_children(0, 8);
        assert_eq!(children.len(), 0);
    }
}





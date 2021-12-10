use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::str::Chars;
use array2d::Array2D;
use itertools::{Itertools, max, min};


pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_10.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let lines =  buffered.lines();

    let opening = HashSet::from(['{', '[', '(', '<' ]);
    let closing = HashMap::from([('{','}'), ('[', ']'), ('(', ')'), ('<', '>') ]);
    let scores = HashMap::from([(')', 3), (']', 57 ), ('}',1197), ('>', 25137)]);
    let mut error_score = 0 as usize;

    for line in lines  {
        let mut stack = Vec::new();
        let line = line.unwrap();
        for c in line.chars() {
            if opening.contains(&c){
                stack.push(c);
            } else {
                let stack_char = stack.pop().unwrap();
                if *closing.get(&stack_char).unwrap() != c {
                    error_score += scores.get(&c).unwrap();
                    // println!("Expected {}, but found {} instead", c,  closing.get(&stack_char).unwrap());
                    break;
                }
            }
        }
    }

    println!("Error score {}", error_score);

    Ok(())
}





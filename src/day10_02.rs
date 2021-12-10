use std::borrow::BorrowMut;
use std::cmp::Ordering;
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
    let score_map = HashMap::from([(')', 1), (']', 2 ), ('}',3), ('>', 4)]);
    let mut scores = Vec::new();

    for line in lines  {
        let mut stack = Vec::new();
        let line = line.unwrap();
        let mut invalid = false;
        for c in line.chars() {
            if opening.contains(&c){
                stack.push(c);
            } else {
                let stack_char = stack.pop().unwrap();
                if *closing.get(&stack_char).unwrap() != c {
                    invalid = true;
                    break;
                }
            }
        }
        if invalid { continue; }
        let mut local_score = 0 as usize;
        while !stack.is_empty() {
            let stack_char = stack.pop().unwrap();
            let closing_char = closing.get(&stack_char).unwrap();
            local_score = (local_score * 5) + score_map.get(&closing_char).unwrap();
        }

        scores.push(local_score);
    }

    scores.sort();
    let mid = scores[scores.len()/2];
    println!("Mid score {}", mid);

    Ok(())
}




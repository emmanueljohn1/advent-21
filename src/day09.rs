use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use array2d::Array2D;
use itertools::{Itertools, max, min};


fn neighbors(array: &Vec<Vec<u32>>, i: usize, j: usize) -> Vec<Vec<usize>> {
    let positions: Vec<[i64; 2]> = vec![[-1, 0], [0, -1], [1, 0], [0, 1]];
    let values = positions.iter().fold(vec![], |mut acc, val| {
        let x = (i as i64) + val[0];
        let y = (j as i64) + val[1];
        if (x >= 0 && x < array.len() as i64) &&
            (y >= 0 && y < array[0].len() as i64) {
            acc.push(vec![x as usize, y as usize]);
        }

        return acc;
    });

    return values;
}


pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_09.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let grid = parse_data(buffered);

    // ====================================== Part 1- collect low points ============================
    let mut low_points = vec![];
    let mut sum: usize = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let value = grid[row][col];
            let nbs = neighbors(&grid, row, col);
            let lower_nbs_count = nbs.iter().filter(|pos| {
                let nb_val = grid[pos[0]][pos[1]];
                // print!("nbv {:?}, ", nb_val);
                if value < nb_val { return false; }
                return true;
            }).count();

            if lower_nbs_count == 0 {
                low_points.push((row, col));
                sum += (value + 1) as usize;
            }
        }
    }

    println!("low points {:?}", low_points);
    println!("low points height sum {}", sum);

    // ====================================== Part 2 - using DFS ============================
    // build undirected graph
    // let graph = vec![10];

    let mut basin_size_product = 1;
    let mut sizes = BinaryHeap::new();

    for i in 0..low_points.len() {
        let mut basin_size = 1;
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        stack.push(low_points[i]);
        visited.insert(low_points[i]);
        while !stack.is_empty() {
            let u = &low_points[i];
            let (row, col) = stack.pop().unwrap();
            let nbs = neighbors(&grid, row, col);

            for nb in nbs {
                let key = (nb[0], nb[1]);
                if !visited.contains(&key) && grid[nb[0]][nb[1]] != 9 && grid[nb[0]][nb[1]] > grid[row][col] {
                    visited.insert(key);
                    stack.push(key);
                    basin_size += 1;
                }
            }
        }

        sizes.push(basin_size);
        // println!("basin size {}", basin_size);
    }

    for s in 0..3 {
        basin_size_product *= sizes.pop().unwrap();
    }

    println!("basin size product {}", basin_size_product);

    Ok(())
}

fn parse_data(buffered: BufReader<File>) -> Vec<Vec<u32>> {
    let mut numbers: Vec<i32> = Vec::new();

    let data = buffered.lines().fold(vec![], |mut acc, line| {
        const RADIX: u32 = 10;
        let row = line.unwrap().chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<u32>>();
        acc.push(row);
        return acc;
    });

    return data;
}





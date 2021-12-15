use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use array2d::Array2D;
use itertools::{Itertools, max, min};


fn neighbors(array: &Vec<Vec<u32>>, i: i32, j: i32) -> Vec<Vec<i32>> {
    let positions: Vec<[i32; 2]> = vec![[-1, 0], [0, -1], [1, 0], [0, 1]];
    let values = positions.iter().fold(vec![], |mut acc, val| {
        let x = i + val[0];
        let y = j + val[1];
        if (x >= 0 && x < array.len() as i32) &&
            (y >= 0 && y < array[0].len() as i32) {
            acc.push(vec![x, y]);
        }

        return acc;
    });

    return values;
}


pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_15.txt";
    let buffered = BufReader::new(File::open(path).expect("no such file"));
    let mut grid = parse_data(buffered);
    let p: usize = grid.len();
    let q: usize = grid.len();

    let n: usize = p * 5;
    let m: usize = q * 5;
    let mut new_grid = vec![vec![0_usize; m]; n];
    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 {
                continue;
            }

            let min_cost = vec![[-1, 0], [0, -1]].iter().map(|val| {
                let x = i as i32 + val[0];
                let y = j as i32 + val[1];

                if (x >= 0 && x < n as i32) &&
                    (y >= 0 && y < m as i32) {
                    return new_grid[x as usize][y as usize];
                }
                return u32::MAX as usize;
            }).min().unwrap();

            let qx = (i / p) as f32;
            let qy = (j / q) as f32;
            let mut value_ij: usize = 0;

            if qx == 0_f32 && qy == 0_f32 {
                value_ij = grid[i][j] as usize;
            } else {
                let x_prime = (i as f32 - (p as f32 * qx.floor())) as usize;
                let y_prime = (j as f32 - (q as f32 * qy.floor())) as usize;
                value_ij = (grid[x_prime][y_prime] as f32 + ((i / p) as f32).ceil() + ((j / q) as f32).ceil()) as usize;
                if value_ij > 9 {
                    value_ij = value_ij % 9;
                }
            };


            new_grid[i][j] = value_ij + min_cost;
        }
    }
    println!("min cost {:?}", new_grid[n - 1][m - 1]);
    Ok(())
}


fn parse_data(buffered: BufReader<File>) -> Vec<Vec<u32>> {
    let data = buffered.lines().fold(vec![], |mut acc, line| {
        const RADIX: u32 = 10;
        let row = line.unwrap().chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<u32>>();
        acc.push(row);
        return acc;
    });
    return data;
}





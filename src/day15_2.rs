use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use array2d::Array2D;
use itertools::{Itertools, max, min};

pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/input_15.txt";
    let buffered = BufReader::new(File::open(path).expect("no such file"));
    let mut grid = parse_data(buffered);
    let p = grid.len() as u32;
    let q = grid[0].len() as u32;

    let n = p * 5;
    let m = q * 5;
    let mut new_grid: Vec<Vec<u32>> = vec![vec![0; m as usize]; n as usize];

    // This brute force solution computes the min cost of a node as the min of the min_cost of nodes on it's left and above it
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
                return u32::MAX;
            }).min().unwrap();

            let value_ij = infer_value(&mut grid, p, q, i, j);

            new_grid[i as usize][j as usize] = value_ij + min_cost;
        }
    }

    // Works for example but doesn't work for actual input
    println!("min cost with last block sum {}", new_grid[n as usize - 1][m as usize - 1]);

    // This works for the input but not for example
    // I'm basically subtracting the value of the last node from the total cost
    println!("min cost without last {}", new_grid[n as usize - 1][m as usize - 1] - infer_value(&mut grid, p, q, n-1, m-1));

    println!("min cost for first tile block (part 1) {}", new_grid[p as usize - 1][q as usize - 1]); // Works for example but doesn't work for actual input
    Ok(())
}

fn infer_value(grid: &mut Vec<Vec<u32>>, num_rows: u32, num_cols0: u32, row: u32, col: u32) -> u32 {
    let qx = ((row / num_rows) as f32).floor() as u32;
    let qy = ((col / num_rows) as f32).floor() as u32;
    let mut value_ij = 0;

    if qx == 0 && qy == 0 {
        value_ij = grid[row as usize][col as usize];
    } else {
        let x_prime = row - (num_rows * qx);
        let y_prime = col - (num_cols0 * qy);
        value_ij = grid[x_prime as usize][y_prime as usize] + qx + qy;
        if value_ij > 9 {
            value_ij = value_ij % 9; // FIXME - This formula is inaccurate
        }
    };
    value_ij
}


fn parse_data(buffered: BufReader<File>) -> Vec<Vec<u32>> {
    let data = buffered.lines().fold(vec![], |mut acc, line| {
        const RADIX: u32 = 10;
        let row = line.unwrap().trim().chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<u32>>();
        acc.push(row);
        return acc;
    });
    return data;
}





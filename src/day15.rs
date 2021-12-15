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
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    grid[0][0] = 0;

    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 { continue; }
            let min_cost = vec![[-1, 0], [0, -1]].iter().map(|val| {
                let x = i as i32 + val[0];
                let y = j as i32 + val[1];

                if (x >= 0 && x < n as i32) &&
                    (y >= 0 && y < m as i32) {
                    return grid[x as usize][y as usize];
                }
                return u32::MAX;
            }).min().unwrap();

            grid[i][j] = grid[i][j] + min_cost;
        }
    }

    println!("min cost {:?}", grid[n - 1][m - 1]);

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





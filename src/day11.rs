use std::borrow::BorrowMut;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use array2d::Array2D;
use itertools::{Itertools, max, min};


fn neighbors(array: &Vec<Vec<u32>>, i: i32, j: i32) -> Vec<Vec<i32>> {
    let positions: Vec<[i32; 2]> = vec![[-1, 0], [0, -1], [1, 0], [0, 1], [-1, -1], [-1, 1], [1, -1], [1, 1]];
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
    let path = "test_inputs/input_11.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let mut grid = parse_data(buffered);

    let mut flashes = 0;

    for step in 0..100 {
        let mut queue = VecDeque::new();
        for  i in 0..grid.len() {
            for j in 0..grid.len() {
                grid[i][j] = (grid[i][j] + 1) % 10;
                if grid[i][j] == 0 {
                    queue.push_back((i as i32, j as i32));
                }
            }
        }

        while !queue.is_empty() {
            let size = queue.len();
            flashes += size;
            for i in 0..size {
                let (x, y) = queue.pop_front().unwrap();
                let nbs = neighbors(&grid, x, y);

                for nb in nbs {
                    let nb_x = nb[0] as usize;
                    let nb_y = nb[1] as usize;
                    if grid[nb_x][nb_y] != 0 {
                        grid[nb_x][nb_y] = (grid[nb_x][nb_y] + 1) % 10;
                        if grid[nb_x][nb_y] == 0 {
                            queue.push_back((nb_x as i32, nb_y as i32))
                        }
                    }
                }
            }
        }
    }

    println!("flashes {}", flashes); // 1729

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





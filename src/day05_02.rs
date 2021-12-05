use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use array2d::Array2D;
use itertools::{Itertools, max, min};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

// References:
// http://www.sunshine2k.de/coding/java/PointOnLine/PointOnLine.html
// https://stackoverflow.com/questions/11907947/how-to-check-if-a-point-lies-on-a-line-between-2-other-points
fn is_point_between_line(line: &Line, curr_point: &Point) -> bool {
    let Line { p1, p2 } = line;
    let dxc = curr_point.x - p1.x;
    let dyc = curr_point.y - p1.y;

    let dxl = p2.x - p1.x;
    let dyl = p2.y - p1.y;

    // perp dot product
    let cross_prod = (dxc * dyl - dyc * dxl) as i64;

    if cross_prod != 0 { return false; }

    if dxl.abs() >= dyl.abs() {
        return if dxl > 0 {
            p1.x <= curr_point.x && curr_point.x <= p2.x
        } else {
            p2.x <= curr_point.x && curr_point.x <= p1.x
        };
    }

    return if dyl > 0 {
        p1.y <= curr_point.y && curr_point.y <= p2.y
    } else {
        p2.y <= curr_point.y && curr_point.y <= p1.y
    };
}


pub fn run() -> Result<(), Error> {
    println!("Running...");
    let path = "test_inputs/example_05.txt";
    let input = File::open(path).expect("no such file");
    let buffered = BufReader::new(input);
    let (max_p, lines) = parse_data(buffered);
    let mut coordinates = Array2D::filled_with(0, max_p.0 as usize + 1, max_p.1 as usize + 1);

    // This feels very brute-force but I was getting sleepy at this point
    for i in 0..coordinates.row_len() {
        for j in 0..coordinates.column_len() {
            let mut point = Point { x: i as i32, y: j as i32 };
            for line in &lines {
                if is_point_between_line(&line, &point) {
                    coordinates[(point.x as usize, point.y as usize)] = coordinates[(point.x as usize, point.y as usize)] + 1;
                }
            }
        }
    }

    let twos_count = coordinates.as_row_major().iter().filter(|&val| *val >= 2).count();
    println!("count {:?}", twos_count);
    Ok(())
}

fn parse_data(buffered: BufReader<File>) -> ((i32, i32), Vec<Line>) {
    let mut numbers: Vec<i32> = Vec::new();

    let (max_point, lines) = buffered.lines().fold(((0, 0), vec![]), |acc, val| {
        let val = val.unwrap();
        let ((max_x, max_y), lines) = acc;
        let mut lines = lines.to_vec();
        let line_vec = val.split("->").fold(vec![], |mut acc, pstr| {
            acc.push(string_to_vec(&pstr.to_string(), ","));
            return acc;
        });

        let mut line: Line = Line {
            p1: Point { x: line_vec[0][0], y: line_vec[0][1] },
            p2: Point { x: line_vec[1][0], y: line_vec[1][1] },
        };

        let new_max_x = max(vec![max_x, line.p1.x, line.p2.x]).unwrap();
        let new_max_y = max(vec![max_y, line.p1.y, line.p2.y]).unwrap();
        lines.push(line);
        return ((new_max_x, new_max_y), lines);
    });

    return (max_point, lines);
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
    use crate::utils::print_vec;
    use super::*;

    #[test]
    fn test_is_point_on_line() {
        let point: Point = Point { x: 1, y: 2 };
        let line: Line = Line { p1: Point { x: 1, y: 1 }, p2: Point { x: 1, y: 3 } };
        // Should be on line and between
        assert_eq!(is_point_between_line(&line, &point), true);

        // Should not be on line
        assert_eq!(is_point_between_line(&line, &Point { x: 2, y: 5 }), false);

        // On line but not between
        assert_eq!(is_point_between_line(&line, &Point { x: 1, y: 5 }), false);
    }

    #[test]
    fn test_is_winning_board() {}
}





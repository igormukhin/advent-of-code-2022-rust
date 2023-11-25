use std::cmp::{max, min};
use std::fmt::Display;

#[derive(Clone, PartialEq)]
struct Coord {
    row: usize,
    col: usize
}

#[derive(Clone, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Air => write!(f, "."),
            Cell::Rock => write!(f, "#"),
            Cell::Sand => write!(f, "o"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let lines = input
        .split("\n")
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Coord>>>();

    let min_col = lines.iter()
        .map(|line| line.iter().map(|coord| coord.col).min().unwrap())
        .min()
        .unwrap();

    let max_col = lines.iter()
        .map(|line| line.iter().map(|coord| coord.col).max().unwrap())
        .max()
        .unwrap();

    let max_row = lines.iter()
        .map(|line| line.iter().map(|coord| coord.row).max().unwrap())
        .max()
        .unwrap();

    let mut map: Vec<Vec<Cell>> = vec![vec![Cell::Air; max_col - min_col + 1]; max_row + 1];

    for line in lines {
        for i in 1..line.len() {
            let from = &line[i - 1];
            let to = &line[i];
            if from.row == to.row {
                let from_col = min(from.col, to.col) - min_col;
                let to_col = max(from.col, to.col) - min_col;
                for col in from_col..=to_col {
                    map[from.row][col] = Cell::Rock;
                }
            } else {
                let from_row = min(from.row, to.row);
                let to_row = max(from.row, to.row);
                for row in from_row..=to_row {
                    map[row][from.col - min_col] = Cell::Rock;
                }
            }
        }
    }

    let pouring_from = Coord {
        row: 0,
        col: 500 - min_col
    };

    let new_height = map.len() + 2;
    let new_left_col_opp = (-(pouring_from.col as isize - new_height as isize)) as usize;
    let new_right_col = pouring_from.col + new_height;
    let new_width = new_right_col + new_left_col_opp + 1;
    let new_pouring_from = Coord {
        row: 0,
        col: pouring_from.col + new_left_col_opp
    };

    let mut big_map: Vec<Vec<Cell>> = vec![vec![Cell::Air; new_width]; new_height];
    for col in 0..new_width {
        big_map[new_height - 1][col] = Cell::Rock;
    }
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            big_map[row][col + new_left_col_opp] = map[row][col].clone();
        }
    }

    rain_it(&mut map, &pouring_from);
    rain_it(&mut big_map, &new_pouring_from);
}

fn rain_it(map: &mut Vec<Vec<Cell>>, pouring_from: &Coord) {
    let mut resting_count = 0;
    loop {
        let resting_coord = pour(&map, &pouring_from);
        match resting_coord {
            Some(coord) => {
                map[coord.row][coord.col] = Cell::Sand;
                resting_count += 1;
            },
            None => break
        }
    }

/*    for row in 0..map.len() {
        for col in 0..map[0].len() {
            print!("{}", map[row][col]);
        }
        println!();
    }*/

    println!("{}", resting_count);
}

fn pour(map: &Vec<Vec<Cell>>, start_at: &Coord) -> Option<Coord> {
    if map[start_at.row][start_at.col] != Cell::Air {
        return None;
    }

    let mut current = (*start_at).clone();
    loop {
        let next_row = current.row + 1;
        if next_row >= map.len() {
            return None;
        }

        if map[next_row][current.col] == Cell::Air {
            current.row = next_row;
        } else {
            if current.col == 0 {
                return None;
            }
            if map[next_row][current.col - 1] == Cell::Air {
                current.row = next_row;
                current.col -= 1;
            } else {
                if current.col == map[0].len() - 1 {
                    return None;
                }
                if map[next_row][current.col + 1] == Cell::Air {
                    current.row = next_row;
                    current.col += 1;
                } else {
                    return Some(current);
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Vec<Coord> {
    line
        .split(" -> ")
        .map(|coord| parse_coord(coord))
        .collect::<Vec<Coord>>()
}

fn parse_coord(coord: &str) -> Coord {
    let parts = coord.split(",")
        .map(|part| part.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Coord {
        row: parts[1],
        col: parts[0]
    }
}


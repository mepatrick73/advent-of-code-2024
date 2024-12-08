use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self};

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //    let mut content = "............
    //........0...
    //.....0......
    //.......0....
    //....0.......
    //......A.....
    //............
    //............
    //........A...
    //.........A..
    //............
    //............";
    let mut matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    matrix.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, char)| {
            if *char != '.' {
                match freq.get_mut(char) {
                    Some(locations) => {
                        locations.push((row, col));
                    }
                    None => {
                        freq.insert(*char, vec![(row, col)]);
                    }
                }
            }
        });
    });

    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in freq.into_values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let (first_row, first_col) = positions[i];
                let (second_row, second_col) = positions[j];
                let (first_row, first_col) = (first_row as i32, first_col as i32);
                let (second_row, second_col) = (second_row as i32, second_col as i32);
                let d_row = first_row - second_row;
                let d_col = first_col - second_col;
                let new_row = second_row - d_row;
                let new_col = second_col - d_col;
                if in_bounds(&matrix, new_row, new_col) {
                    println!("{}", new_row);
                    println!("{}", new_col);
                    anti_nodes.insert((new_row as usize, new_col as usize));
                }
            }
        }
    }

    println!("{:?}", anti_nodes);
    let result = anti_nodes.len();
    for (row, col) in anti_nodes.iter() {
        println!("{row} {col}");
        matrix[*row][*col] = '#';
    }
    for line in matrix.iter() {
        println!("{:?}", line);
    }

    println!("part 2 result : {result:?}");

    Ok(())
}

fn in_bounds(matrix: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < matrix.len() as i32 && col < matrix.first().unwrap().len() as i32
}
fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //    let mut content = "............
    //........0...
    //.....0......
    //.......0....
    //....0.......
    //......A.....
    //............
    //............
    //........A...
    //.........A..
    //............
    //............";
    let mut matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    matrix.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, char)| {
            if *char != '.' {
                match freq.get_mut(char) {
                    Some(locations) => {
                        locations.push((row, col));
                    }
                    None => {
                        freq.insert(*char, vec![(row, col)]);
                    }
                }
            }
        });
    });

    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for positions in freq.into_values() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let (first_row, first_col) = positions[i];
                let (second_row, second_col) = positions[j];
                let (first_row, first_col) = (first_row as i32, first_col as i32);
                let (second_row, second_col) = (second_row as i32, second_col as i32);
                let d_row = first_row - second_row;
                let d_col = first_col - second_col;
                let mut i = 0;
                loop {
                    let new_row = second_row - i * d_row;
                    let new_col = second_col - i * d_col;
                    if in_bounds(&matrix, new_row, new_col) {
                        anti_nodes.insert((new_row as usize, new_col as usize));
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{:?}", anti_nodes);
    let result = anti_nodes.len();
    for (row, col) in anti_nodes.iter() {
        println!("{row} {col}");
        matrix[*row][*col] = '#';
    }
    for line in matrix.iter() {
        println!("{:?}", line);
    }

    println!("part 2 result: {result:?}");

    Ok(())
}

fn main() -> io::Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}

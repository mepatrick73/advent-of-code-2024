use std::{collections::HashMap, fs, io};

use regex::Regex;

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let re = Regex::new(r"-?\d+").unwrap();
    let mut tuples: Vec<(i32, i32, i32, i32)> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let numbers: Vec<i32> = re
                .find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                .collect();

            if numbers.len() == 4 {
                Some((numbers[0], numbers[1], numbers[2], numbers[3]))
            } else {
                None
            }
        })
        .collect();
    let rows = 103;
    let cols = 101;

    for i in 1..10000000 {
        let mut matrix = vec![vec!['.'; 101]; 103];
        for tuple in tuples.iter_mut() {
            let (col, row, col_s, row_s) = tuple;
            *row = (*row + (*row_s)).rem_euclid(rows);
            *col = (*col + (*col_s)).rem_euclid(cols);
            matrix[*row as usize][*col as usize] = '#';
        }
        let mut neighbors = 0;

        neighbors += max_contiguous(&matrix);

        if neighbors > 10 {
            for row in matrix {
                let s: String = row.to_vec().iter().collect();
                println!("{:?}", s);
            }
            println!("easter egg is at line : {i}");
            break;
        }
    }
    Ok(())
}
fn max_contiguous(matrix: &[Vec<char>]) -> usize {
    matrix
        .iter()
        .map(|row| {
            let mut max_count = 0;
            let mut current_count = 0;

            for &ch in row {
                if ch == '#' {
                    current_count += 1;
                    max_count = max_count.max(current_count);
                } else {
                    current_count = 0;
                }
            }

            max_count
        })
        .max()
        .unwrap_or(0) // Handle case where matrix is empty
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let re = Regex::new(r"-?\d+").unwrap();
    let mut tuples: Vec<(i32, i32, i32, i32)> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let numbers: Vec<i32> = re
                .find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                .collect();

            if numbers.len() == 4 {
                Some((numbers[0], numbers[1], numbers[2], numbers[3]))
            } else {
                None
            }
        })
        .collect();
    let rows = 103;
    let cols = 101;
    let iterations = 100;

    for tuple in tuples.iter_mut() {
        let (col, row, col_s, row_s) = tuple;
        *row = (*row + (*row_s * iterations)).rem_euclid(rows);
        *col = (*col + (*col_s * iterations)).rem_euclid(cols);
    }
    let tuples = count_and_remove_duplicates(tuples);
    let q1 = tuples
        .iter()
        .filter(|tuple| tuple.1 < rows / 2 && tuple.2 < cols / 2)
        .collect::<Vec<_>>();
    let q2 = tuples
        .iter()
        .filter(|tuple| tuple.1 < (rows / 2) && tuple.2 > (cols / 2))
        .collect::<Vec<_>>();
    let q3 = tuples
        .iter()
        .filter(|tuple| tuple.1 > (rows / 2) + 1 && tuple.2 < (cols / 2))
        .collect::<Vec<_>>();
    let q4 = tuples
        .iter()
        .filter(|tuple| tuple.1 > (rows / 2) + 1 && tuple.2 > (cols / 2))
        .collect::<Vec<_>>();
    println!("{:?}", q1);
    println!("{:?}", q2);
    println!("{:?}", q3);
    println!("{:?}", q4);

    let q1 = q1.iter().fold(0, |acc, elem| acc + elem.0);
    let q2 = q2.iter().fold(0, |acc, elem| acc + elem.0);
    let q3 = q3.iter().fold(0, |acc, elem| acc + elem.0);
    let q4 = q4.iter().fold(0, |acc, elem| acc + elem.0);

    let res = q1 * q2 * q3 * q4;

    println!("Part 1 result is : {:?}", res);
    Ok(())
}

fn count_and_remove_duplicates(input: Vec<(i32, i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();

    // Count occurrences of each tuple
    for tuple in input {
        *counts.entry((tuple.0, tuple.1)).or_insert(0) += 1;
    }

    // Convert the map into a vector of 5-tuples
    counts
        .into_iter()
        .map(|(tuple, count)| (count, tuple.1, tuple.0))
        .collect()
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

use nalgebra::{DMatrix, DVector, Vector2};
use std::{collections::HashSet, fs, io};

use nalgebra::SMatrix;
use regex::Regex;

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let button_regex = Regex::new(r"X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_regex = Regex::new(r"X=([0-9]+), Y=([0-9]+)").unwrap();
    let matrix = content
        .split("\n\n")
        .map(|block| {
            let (first_button, right) = block.split_once("\n").unwrap();
            let (second_button, prize) = right.split_once("\n").unwrap();
            //First button
            let captures = button_regex.captures(first_button).unwrap();
            let (x_first, y_first) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );
            //Second button
            let captures = button_regex.captures(second_button).unwrap();
            let (x_second, y_second) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );

            //Prize
            let captures = prize_regex.captures(prize).unwrap();
            let (prize_x, prize_y) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );

            let coeff = DMatrix::from_row_slice(
                2,
                2,
                &[
                    x_first as f64,
                    x_second as f64,
                    y_first as f64,
                    y_second as f64,
                ],
            );
            let epsilon = 0.001;
            let prize = DVector::from_row_slice(&[
                (prize_x + 10000000000000) as f64,
                (prize_y + 10000000000000) as f64,
            ]);
            let the_answer = coeff.lu().solve(&prize).unwrap();

            if the_answer[0] < 0.0 || (the_answer[0] - the_answer[0].round()).abs() > epsilon {
                return 0;
            }
            if the_answer[1] < 0.0 || (the_answer[1] - the_answer[1].round()).abs() > epsilon {
                return 0;
            }

            the_answer[0].round() as i64 * 3 + the_answer[1].round() as i64
        })
        .collect::<Vec<i64>>();

    let res: i64 = matrix.into_iter().sum();
    println!("Part 2 result is : {}", res);
    Ok(())
}

type CoeffMatrix = SMatrix<i64, 2, 2>;
fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let button_regex = Regex::new(r"X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let prize_regex = Regex::new(r"X=([0-9]+), Y=([0-9]+)").unwrap();
    let matrix = content
        .split("\n\n")
        .map(|block| {
            let (first_button, right) = block.split_once("\n").unwrap();
            let (second_button, prize) = right.split_once("\n").unwrap();
            //First button
            let captures = button_regex.captures(first_button).unwrap();
            let (x_first, y_first) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );
            //Second button
            let captures = button_regex.captures(second_button).unwrap();
            let (x_second, y_second) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );

            //Prize
            let captures = prize_regex.captures(prize).unwrap();
            let (prize_x, prize_y) = (
                captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            );

            let coeff = DMatrix::from_row_slice(
                2,
                2,
                &[
                    x_first as f64,
                    x_second as f64,
                    y_first as f64,
                    y_second as f64,
                ],
            );
            let epsilon = 0.00001;
            let prize = DVector::from_row_slice(&[prize_x as f64, prize_y as f64]);
            let the_answer = coeff.lu().solve(&prize).unwrap();

            if the_answer[0] < 0.0 || (the_answer[0] - the_answer[0].round()).abs() > epsilon {
                return 0;
            }
            if the_answer[1] < 0.0 || (the_answer[1] - the_answer[1].round()).abs() > epsilon {
                return 0;
            }
            if the_answer[0].round() as i64 >= 100 && the_answer[1].round() as i64 >= 100 {
                return 0;
            }

            the_answer[0].round() as i64 * 3 + the_answer[1].round() as i64
        })
        .collect::<Vec<i64>>();

    let res: i64 = matrix.into_iter().sum();
    println!("Part 1 result is : {}", res);
    Ok(())
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

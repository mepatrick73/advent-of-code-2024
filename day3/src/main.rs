use regex::Regex;
use std::{fs, io};

fn split_by_do(content: &str) -> Vec<&str> {
    let mut res = Vec::new();
    let mut remaining = content;

    while let Some((left, rest)) = remaining.split_once("don't()") {
        res.push(left);

        if let Some((_, after_do)) = rest.split_once("do()") {
            remaining = after_do;
        } else {
            break;
        }
    }

    res.push(remaining);

    res
}

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let parsed_content = split_by_do(&content);
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let mut acc = 0;
    for activated_lines in parsed_content {
        for cap in re.captures_iter(&activated_lines) {
            let first_number = &cap[1].parse::<i32>().unwrap();
            let second_number = &cap[2].parse::<i32>().unwrap();
            acc += first_number * second_number;
        }
    }
    println!("Part 2 solution is : {}", acc);

    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    let mut acc = 0;
    for cap in re.captures_iter(&content) {
        let first_number = &cap[1].parse::<i32>().unwrap();
        let second_number = &cap[2].parse::<i32>().unwrap();
        acc += first_number * second_number;
    }
    println!("Result for part 1 is : {acc}");
    Ok(())
}

fn main() -> io::Result<()> {
    part_1();
    part_2();
    Ok(())
}

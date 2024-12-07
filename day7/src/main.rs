use std::{collections::HashSet, fs, io, thread, time::Duration};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let parsed_lines: Vec<(i64, Vec<i64>)> = content
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let left = left.trim().parse::<i64>().unwrap();
            let right = right
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (left, right)
        })
        .collect();

    let mut res = 0;

    for line in &parsed_lines {
        let total = line.0;
        let numbers = &line.1;
        let all_operations = generate_operations(numbers.len(), false);

        for operations in &all_operations {
            let result = numbers
                .iter()
                .enumerate()
                .skip(1)
                .fold(numbers[0], |acc, (i, number)| {
                    let op = operations.get(i - 1).unwrap();
                    op(acc, *number)
                });

            if result == total {
                res += result;
                break;
            }
        }
    }

    println!("Part 2 result is : {}", res);
    Ok(())
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn concat(a: i64, b: i64) -> i64 {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let result_str = format!("{}{}", a_str, b_str);
    result_str.parse::<i64>().unwrap()
}

fn generate_operations(n: usize, is_part_1: bool) -> Vec<Vec<fn(i64, i64) -> i64>> {
    let operations = if is_part_1 {
        vec![add as fn(i64, i64) -> i64, multiply]
    } else {
        vec![add as fn(i64, i64) -> i64, multiply, concat]
    };

    let mut all_operations = Vec::new();

    let num_operations = operations.len();
    let total_combinations = num_operations.pow((n - 1) as u32);

    for combo in 0..total_combinations {
        let mut current_combo = Vec::new();
        let mut combo_index = combo;

        for _ in 0..(n - 1) {
            let operation_index = combo_index % num_operations;
            current_combo.push(operations[operation_index]);
            combo_index /= num_operations;
        }

        all_operations.push(current_combo);
    }

    all_operations
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let parsed_lines: Vec<(i64, Vec<i64>)> = content
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            let left = left.trim().parse::<i64>().unwrap();
            let right = right
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (left, right)
        })
        .collect();

    let mut res = 0;

    for line in &parsed_lines {
        let total = line.0;
        let numbers = &line.1;
        let all_operations = generate_operations(numbers.len(), true);

        for operations in &all_operations {
            let result = numbers
                .iter()
                .enumerate()
                .skip(1)
                .fold(numbers[0], |acc, (i, number)| {
                    let op = operations.get(i - 1).unwrap();
                    op(acc, *number)
                });

            if result == total {
                res += result;
                break;
            }
        }
    }

    println!("Part 1 result is : {}", res);
    Ok(())
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1();
    println!("Elapsed time: {:.2?}", before.elapsed());
    part_2();
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

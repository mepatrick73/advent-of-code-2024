use std::{fs, io};

fn is_ok(vec: &[i32]) -> bool {
    if vec[0] > vec[1] {
        vec.windows(2)
            .all(|w| w[0] >= w[1] && ((w[0] - w[1]).abs() >= 1) && (w[0] - w[1]).abs() <= 3)
    } else {
        vec.windows(2)
            .all(|w| w[0] <= w[1] && ((w[0] - w[1]).abs() >= 1) && (w[0] - w[1]).abs() <= 3)
    }
}

fn check_both(vec: &[i32]) -> bool {
    vec.windows(2)
        .all(|w| w[0] >= w[1] && ((w[0] - w[1]).abs() >= 1) && (w[0] - w[1]).abs() <= 3)
        || vec
            .windows(2)
            .all(|w| w[0] <= w[1] && ((w[0] - w[1]).abs() >= 1) && (w[0] - w[1]).abs() <= 3)
}

fn is_ok2(vec: &[i32]) -> bool {
    for i in 0..vec.len() {
        let (left, right) = vec.split_at(i);
        let without_element = [&left[..], &right[1..]].concat();
        if check_both(&without_element) {
            return true;
        }
    }
    false
}

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let reports: Vec<Vec<i32>> = content
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let number_of_good_reports: usize = reports.iter().filter(|report| is_ok2(report)).count();
    println!("Part 2 result is : {:?}", number_of_good_reports);

    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let reports: Vec<Vec<i32>> = content
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let number_of_good_reports: usize = reports.iter().filter(|report| is_ok(report)).count();

    println!("Part 1 result is : {:?}", number_of_good_reports);
    Ok(())
}

fn main() -> io::Result<()> {
    part_1();
    part_2();
    Ok(())
}

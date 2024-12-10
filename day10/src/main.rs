use std::{collections::HashSet, fs, io, sync::Mutex, thread, time::Duration};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //    let content = "89010123
    //78121874
    //87430965
    //96549874
    //45678903
    //32019012
    //01329801
    //10456732";
    let matrix: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    println!("{matrix:?}");

    let mut res = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix.first().unwrap().len() {
            if matrix[row][col] == 0 {
                println!("exploring 0 path from {row} {col}");
                res += dfs2(&matrix, row as i32 - 1, col as i32, 0);
                res += dfs2(&matrix, row as i32, col as i32 + 1, 0);
                res += dfs2(&matrix, row as i32 + 1, col as i32, 0);
                res += dfs2(&matrix, row as i32, col as i32 - 1, 0);
            }
        }
    }
    println!("Part 2 result is : {}", res);
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //    let content = "89010123
    //78121874
    //87430965
    //96549874
    //45678903
    //32019012
    //01329801
    //10456732";
    let matrix: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    println!("{matrix:?}");

    let mut res = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix.first().unwrap().len() {
            if matrix[row][col] == 0 {
                println!("exploring 0 path from {row} {col}");
                let mut visited_nines: HashSet<(i32, i32)> = HashSet::new();
                res += dfs(&matrix, row as i32 - 1, col as i32, 0, &mut visited_nines);
                res += dfs(&matrix, row as i32, col as i32 + 1, 0, &mut visited_nines);
                res += dfs(&matrix, row as i32 + 1, col as i32, 0, &mut visited_nines);
                res += dfs(&matrix, row as i32, col as i32 - 1, 0, &mut visited_nines);
            }
        }
    }
    println!("Part 1 result is : {}", res);
    Ok(())
}

fn dfs2(matrix: &Vec<Vec<i32>>, row: i32, col: i32, previous_number: i32) -> i32 {
    if row < 0 || col < 0 || row >= (matrix.len() as i32) || col >= (matrix[0].len() as i32) {
        return 0;
    }
    let current_number = matrix[row as usize][col as usize];
    if current_number != previous_number + 1 {
        return 0;
    }
    println!("exploring {current_number} path from {row} {col}");
    if current_number == 9 {
        println!("bingo!");
        return 1;
    }
    let mut res = 0;
    res += dfs2(matrix, row - 1, col, current_number);
    res += dfs2(matrix, row, col + 1, current_number);
    res += dfs2(matrix, row + 1, col, current_number);
    res += dfs2(matrix, row, col - 1, current_number);
    return res;
}

fn dfs(
    matrix: &Vec<Vec<i32>>,
    row: i32,
    col: i32,
    previous_number: i32,
    visited_nines: &mut HashSet<(i32, i32)>,
) -> i32 {
    if row < 0 || col < 0 || row >= (matrix.len() as i32) || col >= (matrix[0].len() as i32) {
        return 0;
    }
    let current_number = matrix[row as usize][col as usize];
    if current_number != previous_number + 1 {
        return 0;
    }
    println!("exploring {current_number} path from {row} {col}");
    if current_number == 9 && !visited_nines.contains(&(row, col)) {
        visited_nines.insert((row, col));
        println!("bingo!");
        return 1;
    }
    let mut res = 0;
    res += dfs(matrix, row - 1, col, current_number, visited_nines);
    res += dfs(matrix, row, col + 1, current_number, visited_nines);
    res += dfs(matrix, row + 1, col, current_number, visited_nines);
    res += dfs(matrix, row, col - 1, current_number, visited_nines);
    return res;
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

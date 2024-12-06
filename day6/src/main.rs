use std::{collections::HashSet, fs, io};

use indicatif::{ProgressBar, ProgressStyle};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;

    let mut map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let res = simulate_block(&mut map);
    println!("Part 2 result is : {}", res);
    Ok(())
}

enum Position {
    Up(i32, i32),
    Right(i32, i32),
    Down(i32, i32),
    Left(i32, i32),
}

fn is_out_of_bound(map: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    let n_rows = map.len();
    let n_cols = map.first().unwrap().len();
    if row < 0 || col < 0 || row >= n_rows as i32 || col >= n_cols as i32 {
        return true;
    }
    false
}

fn is_wall(map: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    if let Some(ref line) = map.get(row as usize) {
        if let Some(char) = line.get(col as usize) {
            if *char == '#' || *char == 'O' {
                return true;
            }
        }
    }
    false
}

enum Status {
    Ok,
    Blocked,
    GotStuck,
}

fn next_move_block(map: &mut Vec<Vec<char>>, visited: &mut HashSet<(char, i32, i32)>) -> Status {
    match Position::find_guard(&map) {
        Position::Up(row, col) => {
            let current_pos = ('^', row, col);

            if visited.contains(&current_pos) {
                return Status::Blocked;
            }

            if is_wall(&map, row - 1, col) {
                map[row as usize][col as usize] = '>';
                visited.insert(current_pos);
                return Status::Ok;
            }
            if is_out_of_bound(&map, row - 1, col) {
                return Status::GotStuck;
            }
            map[row as usize][col as usize] = 'X';
            map[(row - 1) as usize][col as usize] = '^';
            visited.insert(current_pos);
            return Status::Ok;
        }
        Position::Right(row, col) => {
            let current_pos = ('>', row, col);

            if visited.contains(&current_pos) {
                return Status::Blocked;
            }

            if is_wall(&map, row, col + 1) {
                map[row as usize][col as usize] = 'v';
                visited.insert(current_pos);
                return Status::Ok;
            }
            if is_out_of_bound(&map, row, col + 1) {
                return Status::GotStuck;
            }
            map[row as usize][col as usize] = 'X';
            map[row as usize][(col + 1) as usize] = '>';
            visited.insert(current_pos);
            return Status::Ok;
        }
        Position::Down(row, col) => {
            let current_pos = ('v', row, col);

            if visited.contains(&current_pos) {
                return Status::Blocked;
            }

            if is_wall(&map, row + 1, col) {
                map[row as usize][col as usize] = '<';
                visited.insert(current_pos);
                return Status::Ok;
            }
            if is_out_of_bound(&map, row + 1, col) {
                return Status::GotStuck;
            }
            map[row as usize][col as usize] = 'X';
            map[(row + 1) as usize][col as usize] = 'v';
            visited.insert(current_pos);
            return Status::Ok;
        }
        Position::Left(row, col) => {
            let current_pos = ('<', row, col);

            if visited.contains(&current_pos) {
                return Status::Blocked;
            }

            if is_wall(&map, row, col - 1) {
                map[row as usize][col as usize] = '^';
                visited.insert(current_pos);
                return Status::Ok;
            }
            if is_out_of_bound(&map, row, col - 1) {
                return Status::GotStuck;
            }
            map[row as usize][col as usize] = 'X';
            map[row as usize][(col - 1) as usize] = '<';
            visited.insert(current_pos);
            return Status::Ok;
        }
    }
}

fn next_move(map: &mut Vec<Vec<char>>) -> bool {
    match Position::find_guard(&map) {
        Position::Up(row, col) => {
            if is_wall(&map, row - 1, col) {
                map[row as usize][col as usize] = '>';
                return true;
            }
            if is_out_of_bound(&map, row - 1, col) {
                return false;
            }
            map[row as usize][col as usize] = 'X';
            map[(row - 1) as usize][col as usize] = '^';
            return true;
        }
        Position::Right(row, col) => {
            if is_wall(&map, row, col + 1) {
                map[row as usize][col as usize] = 'v';
                return true;
            }
            if is_out_of_bound(&map, row, col + 1) {
                return false;
            }
            map[row as usize][col as usize] = 'X';
            map[row as usize][(col + 1) as usize] = '>';
            return true;
        }
        Position::Down(row, col) => {
            if is_wall(&map, row + 1, col) {
                map[row as usize][col as usize] = '<';
                return true;
            }
            if is_out_of_bound(&map, row + 1, col) {
                return false;
            }
            map[row as usize][col as usize] = 'X';
            map[(row + 1) as usize][col as usize] = 'v';
            return true;
        }
        Position::Left(row, col) => {
            if is_wall(&map, row, col - 1) {
                map[row as usize][col as usize] = '^';
                return true;
            }
            if is_out_of_bound(&map, row, col - 1) {
                return false;
            }
            map[row as usize][col as usize] = 'X';
            map[row as usize][(col - 1) as usize] = '<';
            return true;
        }
    }
}

impl Position {
    fn find_guard(map: &Vec<Vec<char>>) -> Self {
        for (idx_line, line) in map.iter().enumerate() {
            for (idx_col, tile) in line.iter().enumerate() {
                if *tile == '^' {
                    return Self::Up(idx_line as i32, idx_col as i32);
                }
                if *tile == '>' {
                    return Self::Right(idx_line as i32, idx_col as i32);
                }
                if *tile == 'v' {
                    return Self::Down(idx_line as i32, idx_col as i32);
                }
                if *tile == '<' {
                    return Self::Left(idx_line as i32, idx_col as i32);
                }
            }
        }
        panic!();
    }
}

fn simulate(map: &mut Vec<Vec<char>>) -> i32 {
    while next_move(map) {}
    let res = map
        .iter()
        .flat_map(|line| line.iter())
        .filter(|&&ch| ch == 'X')
        .count() as i32;

    res + 1
}

fn simulate_block(map: &mut Vec<Vec<char>>) -> i32 {
    let mut res: i32 = 0;

    let total_iterations = map.len() * map.first().unwrap().len();
    let progress_bar = ProgressBar::new(total_iterations as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    for row in 0..map.len() {
        for col in 0..map.first().unwrap().len() {
            progress_bar.inc(1);

            if map[row][col] == '.' {
                let mut blocked_map = map.clone();
                blocked_map[row][col] = 'O';

                let mut set = HashSet::new();
                loop {
                    match next_move_block(&mut blocked_map, &mut set) {
                        Status::Ok => {
                            continue;
                        }
                        Status::Blocked => {
                            res += 1;
                            break;
                        }
                        Status::GotStuck => {
                            break;
                        }
                    }
                }
            }
        }
    }

    progress_bar.finish_with_message("Simulation complete");
    res
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;

    let mut map: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let res = simulate(&mut map);
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

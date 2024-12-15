use std::{fs, io, thread, time::Duration};

type Matrix = Vec<Vec<char>>;
fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let (matrix, moveset) = content.split_once("\n\n").unwrap();
    let mut matrix: Matrix = matrix
        .to_string()
        .lines()
        .map(|line| {
            let mut the_chars = vec![];
            for char in line.chars() {
                if char == '#' {
                    the_chars.push('#');
                    the_chars.push('#');
                } else if char == 'O' {
                    the_chars.push('[');
                    the_chars.push(']');
                } else if char == '.' {
                    the_chars.push('.');
                    the_chars.push('.');
                } else if char == '@' {
                    the_chars.push('@');
                    the_chars.push('.');
                } else {
                    panic!("this shouldn't happen");
                }
            }
            return the_chars;
        })
        .collect();
    let moveset: Vec<char> = moveset.chars().filter(|char| *char != '\n').collect();
    for player_move in moveset {
        let (row, col) = find_player(&matrix);
        move_player(row, col, player_move, &mut matrix);
        print!("\x1B[2J\x1B[H");
        for row in matrix.iter() {
            let s: String = row.into_iter().collect();
            println!("{}", s);
        }
        thread::sleep(Duration::from_millis(350));
    }
    let mut res = 0;
    for row in 0..=(matrix.len() - 1) {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '[' {
                res += row * 100 + col;
            }
        }
    }

    println!("Part 2 solution is {res}");

    Ok(())
}

fn move_player(row: usize, col: usize, player_move: char, matrix: &mut Matrix) -> (usize, usize) {
    if player_move == '<' {
        if let Some(to_change) = gather_direction(row, col, (0, -1), matrix) {
            for coord in to_change.iter().rev() {
                matrix[coord.0][(coord.1 as i32 - 1) as usize] = matrix[coord.0][coord.1];
            }
            let first_coord = to_change[0];
            matrix[first_coord.0][first_coord.1] = '.';
            return (row, (col as i32 - 1) as usize);
        }
        return (row, col);
    }
    if player_move == '^' {
        if let Some(to_change) = gather_direction_vertical(row, col, -1, matrix) {
            let old_matrix = matrix.clone();
            for level in to_change.iter().rev() {
                for coord in level {
                    let (curr_row, curr_col) = *coord;
                    matrix[curr_row - 1][curr_col] = old_matrix[curr_row][curr_col];
                    matrix[curr_row][curr_col] = '.';
                }
            }

            return ((row as i32 - 1) as usize, col);
        }
        return (row, col);
    }
    if player_move == '>' {
        if let Some(to_change) = gather_direction(row, col, (0, 1), matrix) {
            for coord in to_change.iter().rev() {
                matrix[coord.0][(coord.1 as i32 + 1) as usize] = matrix[coord.0][coord.1];
            }
            let first_coord = to_change[0];
            matrix[first_coord.0][first_coord.1] = '.';
            return (row, (col as i32 + 1) as usize);
        }
        return (row, col);
    }
    if player_move == 'v' {
        if let Some(to_change) = gather_direction_vertical(row, col, 1, matrix) {
            let old_matrix = matrix.clone();
            for level in to_change.iter().rev() {
                for coord in level {
                    let (curr_row, curr_col) = *coord;
                    matrix[curr_row + 1][curr_col] = old_matrix[curr_row][curr_col];
                    matrix[curr_row][curr_col] = '.';
                }
            }

            return ((row as i32 + 1) as usize, col);
        }
        return (row, col);
    }
    panic!("shouldn't get here");
}

fn gather_direction_vertical(
    row: usize,
    col: usize,
    dir: i32,
    matrix: &Matrix,
) -> Option<Vec<Vec<(usize, usize)>>> {
    let mut all_levels: Vec<Vec<(usize, usize)>> = vec![];
    let mut next_level: Vec<(usize, usize)> = vec![(row, col)];
    while !next_level.is_empty() {
        let mut merged_current_level = vec![];

        for coord in next_level.iter() {
            let char = matrix[coord.0][coord.1];
            if char == '[' {
                merged_current_level.push((coord.0, coord.1));
                let new_coord = (coord.0, coord.1 + 1);
                assert_eq!(matrix[new_coord.0][new_coord.1], ']');
                merged_current_level.push((new_coord.0, new_coord.1));
            } else if char == ']' {
                merged_current_level.push((coord.0, coord.1));
                let new_coord = (coord.0, coord.1 - 1);
                assert_eq!(matrix[new_coord.0][new_coord.1], '[');
                merged_current_level.push((new_coord.0, new_coord.1));
            } else {
                merged_current_level.push((coord.0, coord.1));
            }
        }

        let mut next_next_level: Vec<(usize, usize)> = vec![];

        for coord in merged_current_level.iter() {
            let moved_char = matrix[((coord.0 as i32) + dir) as usize][coord.1];
            if moved_char == '[' || moved_char == ']' {
                next_next_level.push((((coord.0 as i32) + dir) as usize, coord.1));
            } else if moved_char == '#' {
                return None;
            }
        }
        all_levels.push(merged_current_level);
        next_level = next_next_level;
    }
    return Some(all_levels);
}

fn gather_direction(
    row: usize,
    col: usize,
    dir: (i32, i32),
    matrix: &Matrix,
) -> Option<Vec<(usize, usize)>> {
    let mut ret: Vec<(usize, usize)> = vec![];
    for i in 0..matrix.len() {
        let curr_row = ((row as i32) + (i as i32 * dir.0)) as usize;
        let curr_col = ((col as i32) + (i as i32 * dir.1)) as usize;
        let curr_char = matrix[curr_row][curr_col];
        if curr_char == '@' || curr_char == '[' || curr_char == ']' {
            ret.push((curr_row, curr_col));
        }
        if curr_char == '.' {
            return Some(ret);
        }
        if curr_char == '#' {
            return None;
        }
    }
    panic!("never found a . or # to terminate the sequences");
}

fn find_player(matrix: &Matrix) -> (usize, usize) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '@' {
                return (i, j);
            }
        }
    }
    panic!("no player found");
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

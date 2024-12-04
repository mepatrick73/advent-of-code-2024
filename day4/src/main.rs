use regex::Regex;
use std::{fs, io};

struct Kernel {
    top_left: char,
    top_right: char,
    middle: char,
    bottom_left: char,
    bottom_right: char,
}

impl Kernel {
    fn new(row: i32, col: i32, matrix: &Vec<&str>) -> Option<Kernel> {
        let rows = matrix.len();
        let cols = matrix.first().unwrap().len();
        if row <= 0 || row >= rows as i32 - 1 {
            return None;
        }
        if col <= 0 || row >= cols as i32 - 1 {
            return None;
        }
        let top_left = matrix
            .get((row - 1) as usize)?
            .chars()
            .nth((col - 1) as usize)?;
        let top_right = matrix
            .get((row - 1) as usize)?
            .chars()
            .nth((col + 1) as usize)?;
        let middle = matrix.get((row) as usize)?.chars().nth((col) as usize)?;
        let bottom_left = matrix
            .get((row + 1) as usize)?
            .chars()
            .nth((col - 1) as usize)?;
        let bottom_right = matrix
            .get((row + 1) as usize)?
            .chars()
            .nth((col + 1) as usize)?;
        Some(Kernel {
            top_left,
            top_right,
            middle,
            bottom_right,
            bottom_left,
        })
    }

    fn contains_xmas(&self) -> bool {
        let forward_char: Vec<char> = "MAS".chars().collect();
        let backward_char: Vec<char> = "SAM".chars().collect();
        let mut left_diag_ok = false;
        let mut right_diag_ok = false;

        if self.bottom_left == *forward_char.get(0).unwrap()
            && self.middle == *forward_char.get(1).unwrap()
            && self.top_right == *forward_char.get(2).unwrap()
        {
            left_diag_ok = true;
        }

        if self.bottom_left == *backward_char.get(0).unwrap()
            && self.middle == *backward_char.get(1).unwrap()
            && self.top_right == *backward_char.get(2).unwrap()
        {
            left_diag_ok = true;
        }

        if self.bottom_right == *forward_char.get(0).unwrap()
            && self.middle == *forward_char.get(1).unwrap()
            && self.top_left == *forward_char.get(2).unwrap()
        {
            right_diag_ok = true;
        }

        if self.bottom_right == *backward_char.get(0).unwrap()
            && self.middle == *backward_char.get(1).unwrap()
            && self.top_left == *backward_char.get(2).unwrap()
        {
            right_diag_ok = true;
        }

        left_diag_ok && right_diag_ok
    }
}

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let matrix: Vec<&str> = content.split("\n").collect();
    let ans = calculate_xs(matrix);

    println!("Part 2 solution is : {}", ans);

    Ok(())
}

fn calculate_xs(matrix: Vec<&str>) -> i32 {
    let forward_char: Vec<char> = "MAS".chars().collect();
    let backward_char: Vec<char> = "SAM".chars().collect();
    let rows = matrix.len();
    let cols = matrix.first().unwrap().len();

    let mut acc = 0;

    for row in 0..rows {
        for col in 0..cols {
            if let Some(ker) = Kernel::new(row as i32, col as i32, &matrix) {
                if ker.contains_xmas() {
                    acc += 1;
                }
            }
        }
    }
    acc
}

fn calculate_horizontal(matrix: Vec<&str>) -> i32 {
    let target = "XMAS";
    let target_bw = "SAMX";
    let target_chars: Vec<char> = target.chars().collect();
    let target_bw_chars: Vec<char> = target_bw.chars().collect();
    let mut count = 0;
    let mut normal_count = 0;
    let mut backward_count = 0;

    for (row_idx, row) in matrix.into_iter().enumerate() {
        let row_chars: Vec<char> = row.chars().collect();

        for (col_idx, window) in row_chars.windows(target_chars.len()).enumerate() {
            if window == target_chars {
                count += 1;
                normal_count += 1;
            }
            if window == target_bw_chars {
                count += 1;
                backward_count += 1;
                println!("{row_idx}, {col_idx}");
            }
        }
    }
    println!("normal count {normal_count} and backward_count {backward_count}");
    count
}

fn calculate_vertical(matrix: Vec<&str>) -> i32 {
    let target = "XMAS";
    let target_bw = "SAMX";
    let target_chars: Vec<char> = target.chars().collect();
    let target_bw_chars: Vec<char> = target_bw.chars().collect();
    let mut count = 0;

    let num_rows = matrix.len();
    let num_cols = if num_rows > 0 {
        matrix[0].chars().count()
    } else {
        0
    };

    for col in 0..num_cols {
        let mut column_chars: Vec<char> = Vec::new();
        for row in &matrix {
            if let Some(c) = row.chars().nth(col) {
                column_chars.push(c);
            }
        }

        for window in column_chars.windows(target_chars.len()) {
            if window == target_chars {
                count += 1;
            }
            if window == target_bw_chars {
                count += 1;
            }
        }
    }

    count
}

fn diagelem(matrix: &Vec<&str>, row: usize, col: usize) -> Option<char> {
    if row < matrix.len() {
        if let Some(c) = matrix[row].chars().nth(col) {
            return Some(c);
        }
    }
    None
}

fn calculate_diagonal(matrix: Vec<&str>) -> i32 {
    let target = "XMAS";
    let target_bw = "SAMX";
    let target_chars: Vec<char> = target.chars().collect();
    let target_bw_chars: Vec<char> = target_bw.chars().collect();
    let mut count = 0;

    let num_rows = matrix.len();
    let num_cols = if num_rows > 0 {
        matrix[0].chars().count()
    } else {
        0
    };

    for start_row in 0..num_rows {
        for start_col in 0..num_cols {
            let mut matches_forward = true;
            let mut matches_backward = true;

            for offset in 0..target_chars.len() {
                if diagelem(&matrix, start_row + offset, start_col + offset)
                    != Some(target_chars[offset])
                {
                    matches_forward = false;
                }
                if diagelem(&matrix, start_row + offset, start_col + offset)
                    != Some(target_bw_chars[offset])
                {
                    matches_backward = false;
                }
            }

            if matches_forward {
                count += 1;
            }
            if matches_backward {
                count += 1;
            }

            matches_forward = true;
            matches_backward = true;

            for offset in 0..target_chars.len() {
                if diagelem(&matrix, start_row + offset, start_col.wrapping_sub(offset))
                    != Some(target_chars[offset])
                {
                    matches_forward = false;
                }
                if diagelem(&matrix, start_row + offset, start_col.wrapping_sub(offset))
                    != Some(target_bw_chars[offset])
                {
                    matches_backward = false;
                }
            }

            if matches_forward {
                count += 1;
            }
            if matches_backward {
                count += 1;
            }
        }
    }

    count
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let matrix: Vec<&str> = content.split("\n").collect();

    let horizontal = calculate_horizontal(matrix.clone());
    let vertical = calculate_vertical(matrix.clone());
    let diagonal = calculate_diagonal(matrix.clone());

    println!(
        "Result for part 1 is : {}",
        horizontal + vertical + diagonal
    );
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

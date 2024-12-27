use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, io,
};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let mut S = (0, 0);
    let mut E = (0, 0);
    let mut matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    for (r_idx, row) in matrix.iter().enumerate() {
        for (c_idx, char) in row.iter().enumerate() {
            if *char == 'S' {
                S = (r_idx, c_idx);
            }
            if *char == 'E' {
                E = (r_idx, c_idx);
            }
        }
    }
    //println!("{S:?}, {E:?}");

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(S);
    queue.push_front((S.0, S.1, 0));
    let mut normal_time = 0;
    while !queue.is_empty() {
        let (row, col, lenght) = queue.pop_back().unwrap();
        if (row, col) == E {
            normal_time = lenght;
            break;
        }
        let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (nx, ny) in ns {
            let n_row = row as i32 + nx;
            let n_col = col as i32 + ny;
            if n_row < 0 || n_col < 0 || n_row >= rows as i32 || n_col >= cols as i32 {
                continue;
            }
            if !visited.contains(&(n_row as usize, n_col as usize))
                && matrix[n_row as usize][n_col as usize] != '#'
            {
                queue.push_front((n_row as usize, n_col as usize, lenght + 1));
                visited.insert((n_row as usize, n_col as usize));
            }
        }
    }
    let mut single_cheats: HashSet<(usize, usize)> = HashSet::new();
    let mut double_cheats: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    //println!("{normal_time}");
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '#' {
                single_cheats.insert((row, col));
            }
        }
    }
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() - 1 {
            if matrix[row][col] == '#' && matrix[row][col + 1] == '#' {
                double_cheats.insert(((row, col), (row, col + 1)));
            }
        }
    }
    let mut freq = HashMap::new();
    let single_cheat_len = single_cheats.len();
    for (index, (wall_row, wall_col)) in single_cheats.into_iter().enumerate() {
        matrix[wall_row][wall_col] = '.';
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut pred: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(S);
        queue.push_front((S.0, S.1, 0));
        let mut new_time = 0;
        while !queue.is_empty() {
            let (row, col, lenght) = queue.pop_back().unwrap();
            if (row, col) == E {
                new_time = lenght;
                break;
            }
            let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (nx, ny) in ns {
                let n_row = row as i32 + nx;
                let n_col = col as i32 + ny;
                if n_row < 0 || n_col < 0 || n_row >= rows as i32 || n_col >= cols as i32 {
                    continue;
                }
                if !visited.contains(&(n_row as usize, n_col as usize))
                    && matrix[n_row as usize][n_col as usize] != '#'
                {
                    pred.insert((n_row as usize, n_col as usize), (row, col));
                    queue.push_front((n_row as usize, n_col as usize, lenght + 1));
                    visited.insert((n_row as usize, n_col as usize));
                }
            }
        }
        matrix[wall_row][wall_col] = '#';
        *freq.entry(normal_time - new_time).or_insert(0) += 1;
        println!(
            "{}% done in single cheats",
            index as f32 / single_cheat_len as f32
        );
    }
    let res = freq
        .clone()
        .into_iter()
        .filter(|(key, _value)| *key >= 100)
        .fold(0, |acc, (_key, value)| acc + value);
    println!("day 18 part 1 result is : {:?}", res);
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let mut S = (0, 0);
    let mut E = (0, 0);
    let mut matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    for (r_idx, row) in matrix.iter().enumerate() {
        for (c_idx, char) in row.iter().enumerate() {
            if *char == 'S' {
                S = (r_idx, c_idx);
            }
            if *char == 'E' {
                E = (r_idx, c_idx);
            }
        }
    }
    //println!("{S:?}, {E:?}");

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(S);
    queue.push_front((S.0, S.1, 0));
    let mut normal_time = 0;
    while !queue.is_empty() {
        let (row, col, lenght) = queue.pop_back().unwrap();
        if (row, col) == E {
            normal_time = lenght;
            break;
        }
        let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (nx, ny) in ns {
            let n_row = row as i32 + nx;
            let n_col = col as i32 + ny;
            if n_row < 0 || n_col < 0 || n_row >= rows as i32 || n_col >= cols as i32 {
                continue;
            }
            if !visited.contains(&(n_row as usize, n_col as usize))
                && matrix[n_row as usize][n_col as usize] != '#'
            {
                queue.push_front((n_row as usize, n_col as usize, lenght + 1));
                visited.insert((n_row as usize, n_col as usize));
            }
        }
    }
    let mut single_cheats: HashSet<(usize, usize)> = HashSet::new();

    //println!("{normal_time}");
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == '#' {
                single_cheats.insert((row, col));
            }
        }
    }
    let mut freq = HashMap::new();
    let single_cheat_len = single_cheats.len();
    for (index, (wall_row, wall_col)) in single_cheats.into_iter().enumerate() {
        matrix[wall_row][wall_col] = '.';
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut pred: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(S);
        queue.push_front((S.0, S.1, 0));
        let mut new_time = 0;
        while !queue.is_empty() {
            let (row, col, lenght) = queue.pop_back().unwrap();
            if (row, col) == E {
                new_time = lenght;
                break;
            }
            let ns = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (nx, ny) in ns {
                let n_row = row as i32 + nx;
                let n_col = col as i32 + ny;
                if n_row < 0 || n_col < 0 || n_row >= rows as i32 || n_col >= cols as i32 {
                    continue;
                }
                if !visited.contains(&(n_row as usize, n_col as usize))
                    && matrix[n_row as usize][n_col as usize] != '#'
                {
                    pred.insert((n_row as usize, n_col as usize), (row, col));
                    queue.push_front((n_row as usize, n_col as usize, lenght + 1));
                    visited.insert((n_row as usize, n_col as usize));
                }
            }
        }
        matrix[wall_row][wall_col] = '#';
        *freq.entry(normal_time - new_time).or_insert(0) += 1;
        println!(
            "{}% done in single cheats",
            index as f32 / single_cheat_len as f32
        );
    }
    let res = freq
        .clone()
        .into_iter()
        .filter(|(key, _value)| *key >= 100)
        .fold(0, |acc, (_key, value)| acc + value);
    println!("day 18 part 1 result is : {:?}", res);
    Ok(())
}

fn is_sub<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

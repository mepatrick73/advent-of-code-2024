use std::{
    collections::{HashSet, VecDeque},
    fs, io,
};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let rows = 71;
    let cols = 71;
    let coordinate: Vec<Vec<usize>> = content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap() as usize)
                .collect()
        })
        .collect();
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for coord in coordinate.iter().take(1024) {
        let row = coord[1];
        let col = coord[0];
        matrix[row][col] = '#';
    }

    let mut the_corodinate = (-1, -1);
    for i in 1024..coordinate.len() {
        let coord = &coordinate[i];
        let row = coord[1];
        let col = coord[0];
        matrix[row][col] = '#';
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((0, 0));
        queue.push_front((0, 0, 0));
        let mut res = None;
        while !queue.is_empty() {
            let (row, col, lenght) = queue.pop_back().unwrap();
            if (row, col) == (rows - 1, cols - 1) {
                res = Some(lenght);
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
        if res.is_none() {
            the_corodinate = (row as i32, col as i32);
            break;
        }
    }

    for row in matrix.iter() {
        println!("{row:?}");
    }
    println!("day 18 part 2 result is : {:?}", the_corodinate);
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let rows = 71;
    let cols = 71;
    let coordinate: Vec<Vec<usize>> = content
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<i32>().unwrap() as usize)
                .collect()
        })
        .collect();
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    for coord in coordinate.iter().take(1024) {
        let row = coord[1];
        let col = coord[0];
        matrix[row][col] = '#';
    }

    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));
    queue.push_front((0, 0, 0));
    let mut res = 0;
    while !queue.is_empty() {
        let (row, col, lenght) = queue.pop_back().unwrap();
        if (row, col) == (rows - 1, cols - 1) {
            res = lenght;
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

    for row in matrix.iter() {
        println!("{row:?}");
    }
    println!("day 18 part 1 result is : {:?}", res);
    Ok(())
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

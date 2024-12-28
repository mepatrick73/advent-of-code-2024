use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, io,
};

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let (start_coordinate, end_coordinate, matrix, rows, cols) = parse_matrix(content);
    let res = cheat_distance(start_coordinate, end_coordinate, rows, cols, matrix, 20);
    println!("Day 20 part 2 result is : {res}");
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let (start_coordinate, end_coordinate, matrix, rows, cols) = parse_matrix(content);
    let res = cheat_distance(start_coordinate, end_coordinate, rows, cols, matrix, 2);
    println!("Day 20 part 1 result is : {res}");
    Ok(())
}

fn cheat_distance(
    start_coordinate: (usize, usize),
    end_coordinate: (usize, usize),
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<char>>,
    cheat_length: i32,
) -> usize {
    let shortest_path = bfs_shortest_path(start_coordinate, end_coordinate, &matrix, rows, cols);
    let normal_time = shortest_path.len() - 1;
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for i in 0..shortest_path.len() {
        let node_1 = shortest_path[i];
        for j in i + 1..shortest_path.len() {
            let node_2 = shortest_path[j];
            let man_distance = (node_1.0 as i32 - node_2.0 as i32).abs() as i32
                + (node_1.1 as i32 - node_2.1 as i32).abs() as i32;
            if man_distance <= cheat_length {
                let total_distance = i + man_distance as usize + shortest_path.len() - j - 1;
                if total_distance < normal_time && normal_time - total_distance >= 100 {
                    freq.entry(normal_time - total_distance)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    }
    let res = freq.into_iter().fold(0, |acc, (_key, value)| acc + value);

    res
}

fn bfs_shortest_path(
    start_coordinate: (usize, usize),
    end_coordinate: (usize, usize),
    matrix: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
) -> Vec<(usize, usize)> {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut pred: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    visited.insert(start_coordinate);
    queue.push_front((start_coordinate.0, start_coordinate.1, 0));
    let mut shortest_path: Vec<(usize, usize)> = vec![];
    while !queue.is_empty() {
        let (row, col, lenght) = queue.pop_back().unwrap();
        if (row, col) == end_coordinate {
            let mut curr = Some(&end_coordinate);
            while let Some(curr_node) = curr {
                shortest_path.push(*curr_node);
                curr = pred.get(&curr_node);
            }
            shortest_path.reverse();
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
                pred.insert((n_row as usize, n_col as usize), (row, col));
            }
        }
    }
    return shortest_path;
}

fn parse_matrix(content: String) -> ((usize, usize), (usize, usize), Vec<Vec<char>>, usize, usize) {
    let mut start_coordinate = (0, 0);
    let mut end_coordinate = (0, 0);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    for (r_idx, row) in matrix.iter().enumerate() {
        for (c_idx, char) in row.iter().enumerate() {
            if *char == 'S' {
                start_coordinate = (r_idx, c_idx);
            }
            if *char == 'E' {
                end_coordinate = (r_idx, c_idx);
            }
        }
    }
    (start_coordinate, end_coordinate, matrix, rows, cols)
}

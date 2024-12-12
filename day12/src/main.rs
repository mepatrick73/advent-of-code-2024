use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

fn part_2() -> io::Result<()> {
    //let content = fs::read_to_string("res/input.txt")?;
    let content = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    let matrix = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut islands: Vec<Vec<(i32, i32)>> = Vec::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if let Some(island) = visit(row as i32, col as i32, &matrix, &mut visited) {
                islands.push(island);
            }
        }
    }
    let mut res = 0;

    for island in islands.iter() {
        let (row, col) = island.first().unwrap();
        let char = matrix[*row as usize][*col as usize];
        let area = island.len();
        let perimiter = tracing_algorithmn(&matrix, &island);
        println!("current char is {char} with area {area} and perimiter {perimiter}");
        res += area as i32 * perimiter;
    }

    println!("islands are : {:?}", islands);

    println!("Part 2 result is : {}", res);
    Ok(())
}

fn visit_helper(
    row: i32,
    col: i32,
    matrix: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    islands: &mut Vec<(i32, i32)>,
    char: char,
) {
    if row < 0 || col < 0 || row >= matrix.len() as i32 || col >= matrix[0].len() as i32 {
        return;
    }

    if visited.contains(&(row, col)) {
        return;
    }

    if char != matrix[row as usize][col as usize] {
        return;
    }

    visited.insert((row, col));
    islands.push((row, col));
    let n_cs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (n_i, n_j) in n_cs.iter() {
        visit_helper(row + n_i, col + n_j, matrix, visited, islands, char);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn backward(&self) -> Self {
        self.right().right()
    }

    fn step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn t_add(lhs: (i32, i32), rhs: (i32, i32)) -> (i32, i32) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Edge {
    start: (i32, i32),
    end: (i32, i32),
    direction: Direction,
    char: char,
}

impl Edge {
    fn new(start: (i32, i32), end: (i32, i32), direction: Direction, char: char) -> Self {
        return Edge {
            start,
            end,
            direction,
            char,
        };
    }

    fn expand_start(&mut self, matrix: &Vec<Vec<char>>) -> bool {
        let possible_edge_pos = t_add(self.start, self.direction.right().step());
        let should_be_invalid = t_add(possible_edge_pos, self.direction.step());
        if is_valid(&possible_edge_pos, matrix, self.char)
            && !is_valid(&should_be_invalid, matrix, self.char)
        {
            self.start = possible_edge_pos;
            return true;
        }
        return false;
    }

    fn expand_end(&mut self, matrix: &Vec<Vec<char>>) -> bool {
        let possible_edge_pos = t_add(self.end, self.direction.left().step());
        let should_be_invalid = t_add(possible_edge_pos, self.direction.step());
        if is_valid(&possible_edge_pos, matrix, self.char)
            && !is_valid(&should_be_invalid, matrix, self.char)
        {
            self.end = possible_edge_pos;
            return true;
        }
        return false;
    }

    fn new_edge(&self, matrix: &Vec<Vec<char>>) -> Self {
        // top position is a right turn
        let top_postion = t_add(
            t_add(self.end, self.direction.step()),
            self.direction.left().step(),
        );
        if is_valid(&top_postion, matrix, self.char) {
            let new_direction = self.direction.right();
            return Edge::new(
                top_postion.clone(),
                top_postion,
                self.direction.right(),
                self.char,
            );
        } else {
            // do a left turn
            return Edge::new(self.end, self.end, self.direction.left(), self.char);
        }
    }

    fn contains(&self, edge: &Edge) -> bool {
        match self.direction {
            Direction::Up => {
                if edge.direction != Direction::Up {
                    return false;
                } else {
                    return true;
                }
            }
            Direction::Left => i,

            Direction::Down => i,
            Direction::Right => i,
        }
    }
}

fn generate_all_edges(island: &Vec<(i32, i32)>, matrix: &Vec<Vec<char>>, char: char) -> Vec<Edge> {
    let mut ret = vec![];
    for (row, col) in island {
        //Top
        if !is_valid(&t_add((*row, *col), (-1, 0)), matrix, char) {
            ret.push(Edge::new((*row, *col), (*row, *col), Direction::Up, char));
        }
        //Left
        if !is_valid(&t_add((*row, *col), (0, -1)), matrix, char) {
            ret.push(Edge::new((*row, *col), (*row, *col), Direction::Left, char));
        }
        //Bottom
        if !is_valid(&t_add((*row, *col), (1, 0)), matrix, char) {
            ret.push(Edge::new((*row, *col), (*row, *col), Direction::Down, char));
        }
        //Right
        if !is_valid(&t_add((*row, *col), (0, 1)), matrix, char) {
            ret.push(Edge::new(
                (*row, *col),
                (*row, *col),
                Direction::Right,
                char,
            ));
        }
    }
    ret
}

// another idea is to generate all edges, and merge them
// mark single edges as done. Once you've done all the edges, count the number of edges in the set
fn tracing_algorithmn(matrix: &Vec<Vec<char>>, island: &Vec<(i32, i32)>) -> i32 {
    let mut edges: HashSet<Edge> = HashSet::new();
    let start = *island.first().unwrap();
    let (row, col) = *island.first().unwrap();
    let char = matrix[row as usize][col as usize];
    let mut curr_edge = Edge::new(start, start, Direction::Up, char);
    //setup start of edge
    loop {
        if !curr_edge.expand_start(&matrix) {
            //   println!("{curr_edge:?}");
            break;
        }
    }
    //setup end of edge
    loop {
        //println!("should only be called once");
        if !curr_edge.expand_end(&matrix) {
            //    println!("yup");
            //    println!("{curr_edge:?}");
            break;
        }
    }

    loop {
        edges.insert(curr_edge.clone());
        println!("should only be called once");
        curr_edge = curr_edge.new_edge(&matrix);
        loop {
            if !curr_edge.expand_end(&matrix) {
                //        println!("yup");
                //        println!("{curr_edge:?}");
                break;
            }
        }
        //println!("{curr_edge:?}");
        //panic!("testing");
        if edges.contains(&curr_edge) {
            let all_edges = generate_all_edges(island, matrix, char);
            println!("all edges {all_edges:?}");
            println!("built edges edges {edges:?}");
            break;
        }
    }

    edges.len() as i32
}

fn is_valid(pos: &(i32, i32), matrix: &Vec<Vec<char>>, char: char) -> bool {
    let (r, c) = *pos;
    r >= 0
        && r < matrix.len() as i32
        && c >= 0
        && c < matrix[0].len() as i32
        && matrix[r as usize][c as usize] == char
}

fn calculate_perimeter(island: &Vec<(i32, i32)>, matrix: &Vec<Vec<char>>) -> i32 {
    let mut perimiter = 0;
    for (row, col) in island {
        let n_cs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (n_i, n_j) in n_cs.iter() {
            perimiter += check_neighbourg(*row, *col, *n_i, *n_j, matrix);
        }
    }
    perimiter
}

fn check_neighbourg(row: i32, col: i32, ni: i32, nj: i32, matrix: &Vec<Vec<char>>) -> i32 {
    let curr_char = matrix[row as usize][col as usize];
    if row + ni < 0
        || col + nj < 0
        || row + ni >= matrix.len() as i32
        || col + nj >= matrix[0].len() as i32
    {
        return 1;
    }
    if curr_char == matrix[(row + ni) as usize][(col + nj) as usize] {
        return 0;
    }
    1
}

fn visit(
    row: i32,
    col: i32,
    matrix: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
) -> Option<Vec<(i32, i32)>> {
    let mut islands = vec![];
    let char = matrix[row as usize][col as usize];
    visit_helper(row, col, matrix, visited, &mut islands, char);
    if islands.len() != 0 {
        return Some(islands);
    }
    None
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //    let content = "RRRRIICCFF
    //RRRRIICCCF
    //VVRRRCCFFF
    //VVRCCCJFFF
    //VVVVCJJCFE
    //VVIVCCJJEE
    //VVIIICJJEE
    //MIIIIIJJEE
    //MIIISIJEEE
    //MMMISSJEEE
    //";
    let matrix = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut islands: Vec<Vec<(i32, i32)>> = Vec::new();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if let Some(island) = visit(row as i32, col as i32, &matrix, &mut visited) {
                islands.push(island);
            }
        }
    }
    let mut res = 0;

    for island in islands.iter() {
        let area = island.len();
        let perimiter = calculate_perimeter(&island, &matrix);
        res += area as i32 * perimiter;
    }

    println!("islands are : {:?}", islands);

    println!("Part 1 result is : {}", res);
    Ok(())
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    //part_1()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

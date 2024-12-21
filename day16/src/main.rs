use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt, fs, io, thread,
    time::Duration,
    u32::MAX,
};

type Matrix = Vec<Vec<char>>;
type Coord = (usize, usize, Direction);
type Weigth = i32;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Edge {
    node_to: Coord,
    cost: u32,
}

#[derive(Debug)]
struct AdjList {
    graph: HashMap<Coord, Vec<Edge>>,
}

type StartNode = Coord;
type EndNode = (usize, usize);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn get_rotations_cost(&self) -> [(Direction, u32); 3] {
        match self {
            Self::Up => [(Self::Left, 1000), (Self::Right, 1000), (Self::Down, 2000)],
            Self::Left => [(Self::Up, 1000), (Self::Down, 1000), (Self::Right, 2000)],
            Self::Down => [(Self::Right, 1000), (Self::Left, 1000), (Self::Up, 2000)],
            Self::Right => [(Self::Down, 1000), (Self::Up, 1000), (Self::Left, 2000)],
        }
    }

    fn to_direction(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Left => (0, -1),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
        }
    }

    fn directions() -> [Self; 4] {
        [Self::Up, Self::Left, Self::Down, Self::Right]
    }
}

struct PopulatedGraph {
    adj_list: AdjList,
    start_node: StartNode,
    end_node: EndNode,
}

impl fmt::Debug for PopulatedGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PopulatedGraph {{")?;
        writeln!(f, "  adj_list:")?;
        for (key, value) in &self.adj_list.graph {
            writeln!(f, "    {:?}: {:?}", key, value)?;
        }
        writeln!(f, "  start_node: {:?}", self.start_node)?;
        writeln!(f, "  end_node: {:?}", self.end_node)?;
        write!(f, "}}")
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct PQNode {
    cost: u32,
    coord: Coord,
}

impl PopulatedGraph {
    fn djikstra(&mut self) -> usize {
        let mut distance: HashMap<Coord, usize> = HashMap::new();
        let mut previous: HashMap<Coord, Coord> = HashMap::new();
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        for coord in self.adj_list.graph.keys() {
            distance.insert(coord.clone(), u32::MAX as usize);
            pq.push(Reverse(PQNode {
                coord: coord.clone(),
                cost: MAX,
            }));
        }
        *distance.get_mut(&self.start_node).unwrap() = 0;
        pq.push(Reverse(PQNode {
            cost: 0,
            coord: self.start_node.clone(),
        }));
        while !pq.is_empty() {
            let pq_node = pq.pop().unwrap().0;
            let u = pq_node.coord;
            let node_cost = pq_node.cost;
            if u.0 == self.end_node.0 && u.1 == self.end_node.1 {
                let mut curr = u;
                while previous.contains_key(&curr) {
                    curr = previous.get(&curr).unwrap().clone();
                }
                return node_cost as usize;
            }
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u.clone());
            for neighbour in self.adj_list.graph.get(&u).unwrap() {
                let v = neighbour.node_to.clone();
                let v_cost = neighbour.cost;
                if visited.contains(&v) {
                    continue;
                }
                let alt = *distance.get(&u).unwrap() + v_cost as usize;
                if alt < *distance.get(&v).unwrap() {
                    *distance.get_mut(&v).unwrap() = alt;
                    previous.insert(v.clone(), u.clone());
                    pq.push(Reverse(PQNode {
                        cost: alt as u32,
                        coord: v,
                    }));
                }
            }
        }
        panic!("we shouldn't get here");
    }

    fn djikstra_modif(&mut self) -> (usize, HashSet<(usize, usize)>) {
        let mut distance: HashMap<Coord, usize> = HashMap::new();
        let mut previous: HashMap<Coord, Vec<Coord>> = HashMap::new();
        let mut pq = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut first_to_reach = None;
        let mut candidates = vec![];
        for coord in self.adj_list.graph.keys() {
            distance.insert(coord.clone(), u32::MAX as usize);
            previous.insert(coord.clone(), vec![]);
            pq.push(Reverse(PQNode {
                coord: coord.clone(),
                cost: MAX,
            }));
        }
        *distance.get_mut(&self.start_node).unwrap() = 0;
        pq.push(Reverse(PQNode {
            cost: 0,
            coord: self.start_node.clone(),
        }));
        while !pq.is_empty() {
            let pq_node = pq.pop().unwrap().0;
            let u = pq_node.coord;
            let node_cost = pq_node.cost;
            if u.0 == self.end_node.0 && u.1 == self.end_node.1 {
                match first_to_reach {
                    None => {
                        first_to_reach = Some(node_cost as usize);
                        candidates.push(u.clone());
                    }
                    Some(first_to_reach) => {
                        if !visited.contains(&u) && node_cost as usize == first_to_reach {
                            candidates.push(u.clone());
                        }
                    }
                }
            }
            visited.insert(u.clone());
            for neighbour in self.adj_list.graph.get(&u).unwrap() {
                let v = neighbour.node_to.clone();
                let v_cost = neighbour.cost;
                let alt = *distance.get(&u).unwrap() + v_cost as usize;
                if visited.contains(&u) && alt == *distance.get(&v).unwrap() {
                    previous.get_mut(&v).unwrap().push(u.clone());
                }
                if alt < *distance.get(&v).unwrap() {
                    *distance.get_mut(&v).unwrap() = alt;
                    *previous.get_mut(&v).unwrap() = vec![u.clone()];
                    pq.push(Reverse(PQNode {
                        cost: alt as u32,
                        coord: v,
                    }));
                }
            }
        }
        let mut dfs_stack = VecDeque::new();
        let mut dfs_visited = HashSet::new();
        let mut visited_set = HashSet::new();

        for candidate in candidates {
            dfs_stack.push_front(candidate.clone());
            dfs_visited.insert(candidate.clone());
        }

        while !dfs_stack.is_empty() {
            let curr = dfs_stack.pop_front().unwrap();
            dfs_visited.insert(curr.clone());
            if !visited_set.contains(&(curr.0, curr.1)) {
                visited_set.insert((curr.0, curr.1));
            }
            for prev in previous.get(&curr).unwrap() {
                if !dfs_visited.contains(prev) {
                    dfs_stack.push_front(prev.clone());
                    dfs_visited.insert(prev.clone());
                }
            }
        }

        (visited_set.len(), visited_set)
    }
}

impl AdjList {
    fn get_neighbour(
        row: usize,
        col: usize,
        direction: Direction,
        matrix: &Matrix,
    ) -> Option<Edge> {
        let (d_row, d_col) = direction.to_direction();
        let n_row = row as i32 + d_row;
        let n_col = col as i32 + d_col;
        let is_out_of_bounds = n_row < 0
            || n_col < 0
            || n_row >= matrix.len() as i32
            || n_col >= matrix[0].len() as i32;

        if is_out_of_bounds {
            return None;
        }
        if !is_wall(matrix[n_row as usize][n_col as usize]) {
            return Some(Edge {
                node_to: (n_row as usize, n_col as usize, direction),
                cost: 1,
            });
        }
        None
    }

    fn get_neighbouring_edges(row: usize, col: usize, matrix: &Matrix) -> Vec<(Coord, Vec<Edge>)> {
        let mut edges = vec![];
        for direction in Direction::directions() {
            let mut dir_edges = vec![];
            if let Some(edge) = Self::get_neighbour(row, col, direction.clone(), &matrix) {
                dir_edges.push(edge);
            }
            for (n_direction, cost) in direction.get_rotations_cost() {
                dir_edges.push(Edge {
                    node_to: (row, col, n_direction.clone()),
                    cost,
                });
            }
            edges.push(((row, col, direction), dir_edges));
        }
        edges
    }

    fn populate_from_matrix(matrix: &Matrix) -> PopulatedGraph {
        let mut graph = HashMap::new();
        let mut start_node = None;
        let mut end_node = None;
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                let elem = matrix[row][col];

                if is_start(elem) {
                    start_node = Some((row, col, Direction::Right));
                }

                if is_end(elem) {
                    end_node = Some((row, col));
                }

                if !is_wall(elem) {
                    let coord_edges_pairs = Self::get_neighbouring_edges(row, col, &matrix);
                    for (coord, edges) in coord_edges_pairs {
                        graph.insert(coord, edges);
                    }
                }
            }
        }
        let start_node = start_node.expect("start_node not found");
        let end_node = end_node.expect("end_not found");
        return PopulatedGraph {
            adj_list: AdjList { graph },
            start_node,
            end_node,
        };
    }
}

fn is_wall(elem: char) -> bool {
    elem == '#'
}

fn is_end(elem: char) -> bool {
    elem == 'E'
}

fn is_start(elem: char) -> bool {
    elem == 'S'
}
fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let matrix: Matrix = content.lines().map(|line| line.chars().collect()).collect();
    let mut graph = AdjList::populate_from_matrix(&matrix);
    let res = graph.djikstra();

    println!("{:?}", res);
    Ok(())
}

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let matrix: Matrix = content.lines().map(|line| line.chars().collect()).collect();
    let mut graph = AdjList::populate_from_matrix(&matrix);
    let res = graph.djikstra_modif();

    println!("{:?}", res);
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

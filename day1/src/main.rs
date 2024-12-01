use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
type LeftColumn = Vec<LocationId>;
type RightColumn = Vec<LocationId>;
type LocationId = i32;

fn split_columns(columns_string: &str) -> (LeftColumn, RightColumn) {
    let (left, right): (Vec<_>, Vec<_>) = columns_string
        .split_whitespace()
        .map(|item| item.parse::<LocationId>().unwrap())
        .enumerate()
        .partition(|&(index, _)| index % 2 == 0);

    (
        left.into_iter().map(|(_, value)| value).collect(),
        right.into_iter().map(|(_, value)| value).collect(),
    )
}

fn part_1() -> io::Result<()> {
    type LeftAmount = i32;
    type RightAmount = i32;
    let mut file = fs::File::open("res/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let (mut left_column, mut right_column) = split_columns(&content);

    left_column.sort();
    right_column.sort();

    let result: i32 = left_column
        .iter()
        .zip(right_column.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("part 1 total_distance: {result:?}");

    Ok(())
}

type LeftAmount = i32;
type RightAmount = i32;
type Column = Vec<i32>;
fn part_2() -> io::Result<()> {
    fn populate_map(
        mut map: HashMap<LocationId, (LeftAmount, RightAmount)>,
        column: Column,
        update_fn: &impl Fn(&mut (LeftAmount, RightAmount)),
    ) -> HashMap<LocationId, (LeftAmount, RightAmount)> {
        for id in column {
            map.entry(id).or_insert((0, 0)).apply(&update_fn);
        }
        map
    }

    trait Apply {
        fn apply(&mut self, f: &dyn Fn(&mut Self));
    }

    impl<T> Apply for T {
        fn apply(&mut self, f: &dyn Fn(&mut T)) {
            f(self);
        }
    }

    let content = fs::read_to_string("res/input.txt")?;
    let (mut left_column, mut right_column) = split_columns(&content);

    left_column.sort_unstable();
    right_column.sort_unstable();

    let sim_map = HashMap::new();
    let sim_map = populate_map(sim_map, left_column, &|(left, _)| *left += 1);
    let sim_map = populate_map(sim_map, right_column, &|(_, right)| *right += 1);

    let acc: i32 = sim_map
        .into_iter()
        .map(|(id, (left, right))| id * left * right)
        .sum();

    println!("Part 2 result: {acc}");

    Ok(())
}

fn main() -> io::Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, io,
};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let mut split_content = content.split("\n\n");

    let rules = split_content.next().unwrap();
    let mut updates: Vec<Vec<i32>> = split_content
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let pairs: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .collect();
    let mut filtered: Vec<Vec<i32>> = updates
        .into_iter()
        .filter(|update| !match_rule(&pairs, &update))
        .collect();

    filtered
        .iter_mut()
        .for_each(|update| *update = topological_sort(&pairs, update.clone()));

    let middle_sum: i32 = filtered.iter().map(|update| update[update.len() / 2]).sum();

    println!("Part 2 result is  : {:?}", middle_sum);

    Ok(())
}

fn match_rule(rules: &Vec<(i32, i32)>, updates: &Vec<i32>) -> bool {
    for (before, after) in rules {
        if let Some(before_index) = updates.iter().position(|x| x == before) {
            if let Some(after_index) = updates.iter().position(|x| x == after) {
                if before_index > after_index {
                    return false;
                }
            }
        }
    }
    true
}

fn topological_sort(rules: &Vec<(i32, i32)>, mut updates: Vec<i32>) -> Vec<i32> {
    let mut rule_count_map: HashMap<i32, i32> = HashMap::new();
    updates.iter().for_each(|update| {
        rule_count_map.insert(*update, 0);
    });
    for i in 0..updates.len() {
        for j in 0..updates.len() {
            if i == j {
                continue;
            }
            if rules.contains(&(updates[i], updates[j])) {
                *rule_count_map.get_mut(&(updates[i].clone())).unwrap() += 1;
            }
        }
    }
    updates.sort_by(|a, b| {
        rule_count_map
            .get(&b)
            .unwrap()
            .cmp(rule_count_map.get(&a).unwrap())
    });
    return updates;
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let mut split_content = content.split("\n\n");

    let rules = split_content.next().unwrap();
    let updates: Vec<Vec<i32>> = split_content
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let pairs: Vec<(i32, i32)> = rules
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            let first = parts.next().unwrap().parse::<i32>().unwrap();
            let second = parts.next().unwrap().parse::<i32>().unwrap();
            (first, second)
        })
        .collect();

    let filtered: Vec<Vec<i32>> = updates
        .into_iter()
        .filter(|update| match_rule(&pairs, &update))
        .collect();

    let middle_sum: i32 = filtered.iter().map(|update| update[update.len() / 2]).sum();

    println!("Part 1 result is  : {:?}", middle_sum);

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

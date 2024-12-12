use std::{collections::HashMap, io};

fn do_stone(stone: i64, turns_left: i64, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(val) = memo.get(&(stone, turns_left)) {
        return *val;
    }
    if turns_left <= 0 {
        return 1;
    }
    let mut res = 0;
    let str_repr = stone.to_string();
    if stone == 0 {
        res += do_stone(1, turns_left - 1, memo);
    } else if str_repr.len() % 2 == 0 {
        let str_repr = stone.to_string();
        let mid = str_repr.len() / 2;
        let (left_str, right_str) = str_repr.split_at(mid);

        let left_num: i64 = left_str.parse().unwrap();
        let right_num: i64 = right_str.parse().unwrap();
        res += do_stone(left_num, turns_left - 1, memo);
        res += do_stone(right_num, turns_left - 1, memo);
    } else {
        res += do_stone(stone * 2024, turns_left - 1, memo);
    }
    memo.insert((stone, turns_left), res);
    res
}

fn part_2() -> io::Result<()> {
    let content = "7725 185 2 132869 0 1840437 62 26310";
    let stones = content
        .split_whitespace()
        .map(|char| char.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut memo: HashMap<(i64, i64), i64> = HashMap::new();

    let mut res: i64 = 0;
    for stone in stones.iter() {
        res += do_stone(*stone, 75, &mut memo);
    }
    println!("Part 2 result is : {}", res);
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = "7725 185 2 132869 0 1840437 62 26310";
    let stones = content
        .split_whitespace()
        .map(|char| char.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut memo: HashMap<(i64, i64), i64> = HashMap::new();

    let mut res: i64 = 0;
    for stone in stones.iter() {
        res += do_stone(*stone, 25, &mut memo);
    }
    println!("Part 1 result is : {}", res);
    Ok(())
}

fn main() -> io::Result<()> {
    use std::time::Instant;

    let before = Instant::now();
    part_1()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    part_2()?;
    println!("Elapsed time: {:.2?}", before.elapsed());
    Ok(())
}

use std::{collections::HashMap, fs, io};

fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let (substrings, strings) = content.split_once("\n\n").unwrap();
    let substrings: Vec<&str> = substrings.split(", ").collect();
    let strings: Vec<&str> = strings.lines().collect();

    fn try_string<'a>(
        string: &'a str,
        position: usize,
        substrings: &[&str],
        memo: &mut HashMap<usize, i64>,
    ) -> i64 {
        if position == string.len() {
            return 1;
        }
        if let Some(&cached) = memo.get(&position) {
            return cached;
        }

        let mut tot = 0;
        for &substring in substrings {
            if string[position..].starts_with(substring) {
                tot += try_string(string, position + substring.len(), substrings, memo);
            }
        }

        memo.insert(position, tot);
        tot
    }

    let mut res = 0;
    for string in strings {
        let mut memo = HashMap::new();
        res += try_string(string, 0, &substrings, &mut memo);
    }

    println!("Day 19 part 2 result : {res}");
    Ok(())
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    let (substrings, strings) = content.split_once("\n\n").unwrap();
    let substrings: Vec<&str> = substrings.split(", ").collect();
    let strings: Vec<&str> = strings.lines().collect();

    fn try_string<'a>(string: &'a str, position: usize, substrings: &[&str]) -> bool {
        if position == string.len() {
            return true;
        }

        for &substring in substrings {
            if string[position..].starts_with(substring) {
                if try_string(string, position + substring.len(), substrings) {
                    return true;
                }
            }
        }

        return false;
    }

    let mut res = 0;
    for string in strings {
        if try_string(string, 0, &substrings) {
            res += 1;
        }
    }

    println!("Day 19 part 1 result: {res}");
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

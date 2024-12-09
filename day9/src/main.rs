use std::io::{self};
use std::{fmt, fs};

#[derive(PartialEq)]
pub enum Element {
    Block { id: usize },
    FreeSpace,
}

#[derive(PartialEq, Clone)]
pub enum File {
    Block { id: usize, size: usize },
    FreeSpace { size: usize },
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File::Block { id, size } => {
                for _ in 0..*size {
                    write!(f, "{} ", id)?;
                }
                Ok(())
            }
            File::FreeSpace { size } => {
                for _ in 0..*size {
                    write!(f, ".")?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Block { id } => write!(f, "{}", id),
            Element::FreeSpace => write!(f, "."),
        }
    }
}

fn part_1() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //let content = "2333133121414131402";
    let mut parsed_diskmap: Vec<Element> = Vec::new();
    for (idx, char) in content.trim().chars().enumerate() {
        //println!("{char}");
        let char = char.to_digit(10).unwrap();
        let block_time = idx % 2 == 0;
        if block_time {
            let block_id = idx / 2;
            for _ in 0..char {
                parsed_diskmap.push(Element::Block { id: block_id });
            }
        } else {
            for _ in 0..char {
                parsed_diskmap.push(Element::FreeSpace);
            }
        }
    }

    let mut left = -1;
    let mut right = parsed_diskmap.len() as i32;
    while left < right {
        //println!(" left {left}, right {right}");
        let mut should_fully_stop = false;
        left += 1;
        right -= 1;
        if (left - right).abs() <= 1 {
            break;
        }
        // find the next free space
        while parsed_diskmap[left as usize] != Element::FreeSpace {
            if left as usize == parsed_diskmap.len() - 1 {
                should_fully_stop = true;
                break;
            }
            left += 1;
        }
        if should_fully_stop {
            break;
        }
        // find the next used block
        while match parsed_diskmap[right as usize] {
            Element::FreeSpace => true,
            Element::Block { id: _ } => false,
        } {
            if right as usize == 0 {
                should_fully_stop = true;
                break;
            }
            right -= 1;
        }
        if should_fully_stop {
            break;
        }

        parsed_diskmap.swap(left as usize, right as usize);
        //        println!("{:?}", parsed_diskmap);
    }

    let mut result = 0;
    for (idx, elem) in parsed_diskmap.iter().enumerate() {
        match elem {
            Element::FreeSpace => {
                break;
            }
            Element::Block { id: the_id } => {
                result += idx * the_id;
            }
        }
    }
    //println!("{:?}", parsed_diskmap);

    println!("part 1 result : {result:?}");

    Ok(())
}

fn in_bounds(matrix: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    row >= 0 && col >= 0 && row < matrix.len() as i32 && col < matrix.first().unwrap().len() as i32
}
fn part_2() -> io::Result<()> {
    let content = fs::read_to_string("res/input.txt")?;
    //let content = "2333133121414131402";
    let mut parsed_diskmap: Vec<File> = Vec::new();
    for (idx, char) in content.trim().chars().enumerate() {
        //println!("{char}");
        let char = char.to_digit(10).unwrap();
        let block_time = idx % 2 == 0;
        if block_time {
            let block_id = idx / 2;
            parsed_diskmap.push(File::Block {
                id: block_id,
                size: char as usize,
            });
        } else {
            parsed_diskmap.push(File::FreeSpace {
                size: char as usize,
            });
        }
    }

    parsed_diskmap.reverse();
    let mut left_idx = 0;
    while left_idx < parsed_diskmap.len() {
        let current_left_block = parsed_diskmap[left_idx].clone();
        //println!("consdering this block {:?}", current_left_block);
        match current_left_block {
            File::FreeSpace { size: _ } => {
                left_idx += 1;
                continue;
            }
            File::Block {
                id: _,
                size: left_block_size,
            } => {
                let mut right_idx = parsed_diskmap.len() - 1;
                while right_idx != 0 {
                    if left_idx >= right_idx {
                        break;
                    }
                    let mut move_on_from_right = false;
                    let current_right_block = parsed_diskmap[right_idx].clone();
                    match current_right_block {
                        File::Block { id, size } => right_idx -= 1,
                        File::FreeSpace {
                            size: free_space_size,
                        } => {
                            if left_block_size < free_space_size {
                                let new_free_space = free_space_size - left_block_size;
                                let new_space = File::FreeSpace {
                                    size: new_free_space,
                                };
                                let replacement_space = File::FreeSpace {
                                    size: left_block_size,
                                };
                                parsed_diskmap[left_idx] = replacement_space;
                                parsed_diskmap.splice(
                                    right_idx..=right_idx,
                                    vec![new_space, current_left_block.clone()],
                                );
                                //parsed_diskmap.reverse();
                                //println!("{:?}", parsed_diskmap);
                                //parsed_diskmap.reverse();
                                move_on_from_right = true;
                            } else if left_block_size == free_space_size {
                                parsed_diskmap.swap(left_idx, right_idx);
                                move_on_from_right = true;
                            } else {
                                right_idx -= 1;
                            }
                        }
                    };
                    if move_on_from_right {
                        break;
                    }
                }
            }
        }
        //println!("Ok next round");

        //println!("Press Enter to replace the element at index 2 with 99 and 100...");
        //let mut input = String::new();
        //io::stdin()
        //    .read_line(&mut input)
        //    .expect("Failed to read input");

        left_idx += 1;
    }

    parsed_diskmap.reverse();
    let mut the_final_showdown: Vec<i32> = Vec::new();
    for file in parsed_diskmap.iter() {
        match file {
            File::FreeSpace { size } => {
                for _ in 0..*size {
                    the_final_showdown.push(0);
                }
            }
            File::Block { id, size } => {
                for _ in 0..*size {
                    the_final_showdown.push(*id as i32);
                }
            }
        }
    }

    let result = the_final_showdown
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, id)| acc + (idx as i64 * *id as i64));
    println!("Part 2 results are : {:?}", result);

    Ok(())
}

fn main() -> io::Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}

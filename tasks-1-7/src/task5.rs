use std::collections::LinkedList;
use std::{fs, iter};

pub(crate) fn runA() {
    let input = fs::read_to_string("data/input5.txt").unwrap();

    let stacks_count = 9;
    let mut stacks : Vec<LinkedList<u8>> = iter::repeat(LinkedList::new())
        .take(stacks_count)
        .collect();

    let mut setup_finished = false;
    for line in input.lines() {
        let bytes = line.as_bytes();

        if line.is_empty() {
            setup_finished = true;

        } else if setup_finished == false {
            if bytes[1] != '1' as u8 {
                for i in 0..stacks_count {
                    let crate_idx = 1 + (i * 4);
                    if crate_idx >= bytes.len() {
                        break;
                    }
                    let letter = bytes[crate_idx];
                    if letter != ' ' as u8 {
                        stacks[i].push_back(letter);
                    }
                }
            }

        } else {
            let parts: Vec<&str> = line.split(' ').collect();
            let how_many = parts[1].parse::<usize>().unwrap();
            let from_stack = parts[3].parse::<usize>().unwrap();
            let to_stack = parts[5].parse::<usize>().unwrap();

            for _i in 0..how_many {
                let letter = stacks[from_stack - 1].pop_front().unwrap();
                stacks[to_stack - 1].push_front(letter);
            }
        }
    }

    let str = stacks.iter()
        .map(|stack| { *stack.front().unwrap() as char })
        .collect::<String>();

    println!("Task 5a: {}", str);
}

pub(crate) fn runB() {
    let input = fs::read_to_string("data/input5.txt").unwrap();

    let stacks_count = 9;
    let mut stacks : Vec<LinkedList<u8>> = iter::repeat(LinkedList::new())
        .take(stacks_count)
        .collect();

    let mut setup_finished = false;
    for line in input.lines() {
        let bytes = line.as_bytes();

        if line.is_empty() {
            setup_finished = true;

        } else if setup_finished == false {
            if bytes[1] != '1' as u8 {
                for i in 0..stacks_count {
                    let crate_idx = 1 + (i * 4);
                    if crate_idx >= bytes.len() {
                        break;
                    }
                    let letter = bytes[crate_idx];
                    if letter != ' ' as u8 {
                        stacks[i].push_back(letter);
                    }
                }
            }

        } else {
            let parts: Vec<&str> = line.split(' ').collect();
            let how_many = parts[1].parse::<usize>().unwrap();
            let from_stack = parts[3].parse::<usize>().unwrap();
            let to_stack = parts[5].parse::<usize>().unwrap();

            let mut reversed: Vec<u8> = Vec::with_capacity(how_many);
            for _i in 0..how_many {
                let letter = stacks[from_stack - 1].pop_front().unwrap();
                reversed.push(letter);
            }
            reversed.iter().rev().for_each(|letter| {
                stacks[to_stack - 1].push_front(*letter);
            });
        }
    }

    let str = stacks.iter()
        .map(|stack| { *stack.front().unwrap() as char })
        .collect::<String>();

    println!("Task 5b: {}", str);
}

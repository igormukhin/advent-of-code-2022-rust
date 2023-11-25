use std::cmp::{max, Ordering};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Item::Number(num) => num.to_string(),
            Item::List(items) => {
                let mut result = String::new();
                result.push('[');
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        result.push(',');
                    }
                    result.push_str(&item.to_string());
                }
                result.push(']');
                result
            }
        };
        write!(f, "{}", str)
    }
}

impl Item {
    fn len(&self) -> usize {
        match self {
            Item::Number(_) => 1,
            Item::List(items) => items.len(),
        }
    }

    fn get(&self, idx: usize) -> Option<&Item> {
        match self {
            Item::Number(_) => if idx == 0 { Some(self) } else { None },
            Item::List(items) => items.get(idx),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    task_a(&input);
    task_b(&input);
}

fn task_a(input: &String) {
    let sum = input.split("\n\n").map(|group| {
        group.split("\n").map(|line| parse_line(line)).collect::<Vec<Item>>()
    }).collect::<Vec<Vec<Item>>>().iter().enumerate()
        .map(|(i, group)| {
            if compare_items(&group[0], &group[1]) == Ordering::Less {
                i + 1
            } else {
                0
            }
        }).sum::<usize>();
    println!("{}", sum);
}

fn task_b(input: &String) {
    let mut items = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect::<Vec<Item>>();
    let item_extra_1 = parse_line("[[2]]");
    let item_extra_2 = parse_line("[[6]]");
    items.push(item_extra_1.clone());
    items.push(item_extra_2.clone());

    items.sort_unstable_by(compare_items);

    let item_extra_1_idx = 1 + items.iter().position(|item| item == &item_extra_1).unwrap();
    let item_extra_2_idx = 1 + items.iter().position(|item| item == &item_extra_2).unwrap();
    println!("{}", item_extra_1_idx * item_extra_2_idx);
}

fn parse_line(line: &str) -> Item {
    let tokens = tokenize_line(line);
    let (root, _) = parse_tokens(&tokens, 1);
    root
}

fn tokenize_line(block: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut current = String::new();

    for ch in block.chars() {
        match ch {
            ',' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current = String::new();
                }
            },
            '[' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current = String::new();
                }

                tokens.push("[".into());
            },
            ']' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current = String::new();
                }

                tokens.push("]".into());
            },
            _ => {
                current.push(ch);
            }
        };
    }
    tokens
}

fn parse_tokens(tokens: &Vec<String>, idx: usize) -> (Item, usize) {
    let mut current = idx;
    let mut items: Vec<Item> = vec![];

    while current < tokens.len() {
        if tokens[current] == "[" {
            let (item, new_current) = parse_tokens(tokens, current + 1);
            items.push(item);
            current = new_current;
        } else if tokens[current] == "]" {
            return (Item::List(items), current + 1);
        } else {
            items.push(Item::Number(tokens[current].parse::<i32>().unwrap()));
            current += 1;
        }
    };

    panic!("Invalid tokens: {:?} started at {:?}", tokens, idx);
}

fn compare_items(left: &Item, right: &Item) -> Ordering {
    let max_len = max(left.len(), right.len());

    for i in 0..max_len {
        let left_item_opt = left.get(i);
        let right_item_opt = right.get(i);

        if left_item_opt.is_none() {
            return Ordering::Less;
        } else if right_item_opt.is_none() {
            return Ordering::Greater;
        } else {
            let left_item = left_item_opt.unwrap();
            let right_item = right_item_opt.unwrap();

            if let (Item::Number(left_num), Item::Number(right_num)) = (left_item, right_item) {
                if left_num < right_num {
                    return Ordering::Less;
                } else if left_num > right_num {
                    return Ordering::Greater;
                }
            } else {
                let order = compare_items(&left_item, &right_item);
                if order != Ordering::Equal {
                    return order;
                }
            }
        }
    }

    Ordering::Equal
}

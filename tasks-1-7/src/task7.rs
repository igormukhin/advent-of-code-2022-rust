use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Path {
    dirs: Vec<String>
}

impl Path {
    fn root() -> Path {
        Path {
            dirs: Vec::new()
        }
    }

    fn parent(&self) -> Path {
        let mut dirs = self.dirs.clone();
        dirs.pop();
        Path {
            dirs
        }
    }

    fn child(&self, dir: &str) -> Path {
        let mut dirs = self.dirs.clone();
        dirs.push(dir.to_string());
        Path {
            dirs
        }
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        if (&self.dirs).is_empty() {
            return "/".to_string();
        }

        let mut s = String::new();
        for dir in &self.dirs {
            s.push_str("/");
            s.push_str(dir);
        }
        s
    }
}

pub(crate) fn run() {
    let input = fs::read_to_string("../../data/input7.txt").unwrap();

    let mut curr = Path::root();
    let mut dir_sizes: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    curr = curr.parent();
                } else if tokens[2] != "/" {
                    curr = curr.child(tokens[2]);
                }
            }
        } else if tokens[0] != "dir" {
            let size = tokens[0].parse::<u64>().unwrap();
            let mut path = curr.clone();
            loop {
                let path_str = path.to_string();
                *dir_sizes.entry(path_str.clone()).or_insert(0) += size;
                if path_str == "/" {
                    break;
                }
                path = path.parent();
            }
        }
    }

    let task_a = dir_sizes.values().filter(|&x| *x <= 100000).sum::<u64>();
    println!("Task 7a: {}", task_a);

    let total: u64 = 70000000;
    let used: u64 = dir_sizes["/"];
    let free = total - used;
    let free_needed = 30000000;
    let to_free_min = free_needed - free;
    let mut min_dir_to_delete: u64 = 0;
    dir_sizes.values().for_each(|&x| {
        if x >= to_free_min && (min_dir_to_delete == 0 || x < min_dir_to_delete) {
            min_dir_to_delete = x;
        }
    });
    println!("Task 7b: {}", min_dir_to_delete);
}

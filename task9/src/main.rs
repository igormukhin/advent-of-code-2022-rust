use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn step(&self, dir: u8) -> Coord {
        match dir {
            b'U' => Coord { x: self.x, y: self.y + 1 },
            b'D' => Coord { x: self.x, y: self.y - 1 },
            b'L' => Coord { x: self.x - 1, y: self.y },
            b'R' => Coord { x: self.x + 1, y: self.y },
            _ => panic!("Invalid direction"),
        }
    }
}

fn tail_catch_up(tail: &Coord, head: &Coord) -> Coord {
    return if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
        tail.clone()
    } else if head.x == tail.x {
        // vertical step needed
        if head.y > tail.y {
            tail.step(b'U')
        } else {
            tail.step(b'D')
        }
    } else if head.y == tail.y {
        // horizontal step needed
        if head.x > tail.x {
            tail.step(b'R')
        } else {
            tail.step(b'L')
        }
    } else { // diagonal step needed
        let new_x = if head.x > tail.x {
            tail.x + 1
        } else {
            tail.x - 1
        };
        let new_y = if head.y > tail.y {
            tail.y + 1
        } else {
            tail.y - 1
        };
        Coord { x: new_x, y: new_y }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    task_a(&input);
    task_b(&input);
}

fn task_a(input: &String) {
    let mut tail_visited: HashSet<Coord> = HashSet::new();
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    for line in input.lines() {
        let dir = line.as_bytes()[0];
        let steps = line[2..].parse::<i32>().unwrap();
        for _ in 0..steps {
            head = head.step(dir);
            tail = tail_catch_up(&tail, &head);
            tail_visited.insert(tail.clone());
        }
    }

    println!("Task 9a: {}", tail_visited.len());
}

fn task_b(input: &String) {
    let mut tail_visited: HashSet<Coord> = HashSet::new();

    let mut rope = [Coord { x: 0, y: 0 }; 10];
    for line in input.lines() {
        let dir = line.as_bytes()[0];
        let steps = line[2..].parse::<i32>().unwrap();
        for _ in 0..steps {
            for i in 0..rope.len() {
                if i == 0 {
                    rope[i] = rope[i].step(dir);
                } else {
                    rope[i] = tail_catch_up(&rope[i], &rope[i - 1]);
                }
            }
            tail_visited.insert(rope[rope.len() - 1].clone());
        }
    }

    println!("Task 9b: {}", tail_visited.len());
}

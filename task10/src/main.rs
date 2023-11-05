#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    AddX(i32),
    AddXFinish(i32),
    GotoNext
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(parts[1].parse::<i32>().unwrap()),
            _ => panic!("Unknown instruction")
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let milestones = vec![20, 60, 100, 140, 180, 220];
    let mut register_x = 1;
    let mut cycle = 1;
    let mut total = 0;
    let mut screen = vec![false; 240];
    let mut pending_x = 0;

    for line in input.lines() {
        let mut instruction = Instruction::parse(line);
        while instruction != Instruction::GotoNext {
            if pending_x != 0 {
                register_x += pending_x;
                pending_x = 0;
            }

            instruction = match instruction {
                Instruction::Noop => Instruction::GotoNext,
                Instruction::AddX(x) => Instruction::AddXFinish(x),
                Instruction::AddXFinish(x) => {
                    pending_x = x;
                    Instruction::GotoNext
                },
                _ => panic!("Unexpected instruction")
            };

            if milestones.contains(&cycle) {
                total += cycle * register_x;
            }

            let pixel_index = cycle - 1;
            let pixel_col_index = pixel_index % 40;
            if pixel_col_index >= register_x - 1 && pixel_col_index <= register_x + 1 {
                screen[pixel_index as usize] = true;
            }

            cycle += 1;
        }

    }

    println!("Task 10a: {}", total);

    println!("Task 10b:");
    screen.iter()
        .map(|p| if *p { '#' } else { ' ' })
        .collect::<String>()
        .as_str()
        .as_bytes()
        .chunks(40)
        .for_each(|x| println!("{}", std::str::from_utf8(x).unwrap()));
}

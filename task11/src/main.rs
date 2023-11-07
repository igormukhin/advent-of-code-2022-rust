#[derive(Debug, Clone)]
enum WorryOperation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl WorryOperation {
    fn apply(&self, x: &i64) -> i64 {
        match self {
            WorryOperation::Add(y) => x + y,
            WorryOperation::Multiply(y) => x * y,
            WorryOperation::Square => x * x,
        }
    }
}

#[derive(Debug, Clone)]
struct Handover {
    if_divisible_by: i64,
    on_true_throw_to: usize,
    on_false_throw_to: usize,
}

#[derive(Debug, Clone)]
struct MonkeyBrain {
    worry_operation: WorryOperation,
    handover: Handover,
}

#[derive(Debug)]
struct Monkey {
    brain: MonkeyBrain,
    items: Vec<i64>,
    inspections: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    {
        let mut monkeys: Vec<Monkey> = Vec::new();
        parse_input(&input, &mut monkeys);
        let monkey_business = calc_monkey_business(&mut monkeys, 20, 3);
        println!("Task 11a: {}", monkey_business);
    }

    {
        let mut monkeys: Vec<Monkey> = Vec::new();
        parse_input(&input, &mut monkeys);
        let monkey_business = calc_monkey_business(&mut monkeys, 10000, 1);
        println!("Task 11b: {}", monkey_business);
    }
}

fn calc_monkey_business(monkeys: &mut Vec<Monkey>, rounds: i32, relief_divisor: i64) -> usize {
    let dividers_product = monkeys.iter()
        .map(|x| x.brain.handover.if_divisible_by)
        .product::<i64>();

    for _ in 1..=rounds {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.drain(..).collect::<Vec<i64>>();
            monkeys[i].inspections += items.len();
            let brain = monkeys[i].brain.clone();

            for item in items {
                let inspected_item = brain.worry_operation.apply(&item);
                let relieved_item = (inspected_item / relief_divisor) % dividers_product;
                let target_idx = if relieved_item % brain.handover.if_divisible_by == 0 {
                    brain.handover.on_true_throw_to
                } else {
                    brain.handover.on_false_throw_to
                };
                monkeys[target_idx].items.push(relieved_item);
            }
        }
    }

    let mut inspections: Vec<usize> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn parse_input(input: &String, monkeys: &mut Vec<Monkey>) {
    for monkey_lines in input.split("\n\n") {
        let mut monkey = Monkey {
            brain: MonkeyBrain {
                worry_operation: WorryOperation::Add(0),
                handover: Handover {
                    if_divisible_by: 1,
                    on_true_throw_to: 0,
                    on_false_throw_to: 0,
                },
            },
            items: Vec::new(),
            inspections: 0,
        };

        for line in monkey_lines.lines() {
            if line.contains("Starting items:") {
                line[(line.find(":").unwrap() + 2)..]
                    .split(", ")
                    .map(|x| x.parse::<i64>().unwrap())
                    .for_each(|x| monkey.items.push(x));
            } else if line.contains("Operation:") {
                let operand = line[(line.rfind(" ").unwrap() + 1)..].to_string();
                if line.contains("+") {
                    monkey.brain.worry_operation = WorryOperation::Add(operand.parse().unwrap());
                } else if line.contains("*") {
                    if operand == "old" {
                        monkey.brain.worry_operation = WorryOperation::Square;
                    } else {
                        monkey.brain.worry_operation = WorryOperation::Multiply(operand.parse().unwrap());
                    }
                }
            } else if line.contains("Test: divisible by ") {
                let operand = line[(line.rfind(" ").unwrap() + 1)..].parse::<i64>().unwrap();
                monkey.brain.handover.if_divisible_by = operand;
            } else if line.contains("throw to monkey") {
                let to_idx = line[(line.rfind(" ").unwrap() + 1)..].parse::<usize>().unwrap();
                if line.contains("true:") {
                    monkey.brain.handover.on_true_throw_to = to_idx;
                } else {
                    monkey.brain.handover.on_false_throw_to = to_idx;
                }
            }
        }

        monkeys.push(monkey);
    }
}

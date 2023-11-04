pub(crate) fn run() {
    let input = std::fs::read_to_string("data/input4.txt").unwrap();

    let mut free_elfs = 0;
    let mut overlaps = 0;
    for line in input.lines() {
        let nums = line.split(|c| c == '-' || c == ',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let pair1 = (nums[0], nums[1]);
        let pair2 = (nums[2], nums[3]);

        if fully_contains(pair1, pair2) || fully_contains(pair2, pair1) {
            free_elfs += 1;
        }

        if partly_contains(pair1, pair2) || partly_contains(pair2, pair1) {
            overlaps += 1;
        }
    }

    println!("Task 4a: {}", free_elfs);
    println!("Task 4b: {}", overlaps);
}

fn fully_contains(pair: (i32, i32), contained: (i32, i32)) -> bool {
    pair.0 <= contained.0 && pair.1 >= contained.1
}

fn partly_contains(pair: (i32, i32), contained: (i32, i32)) -> bool {
    (pair.0 <= contained.0 && pair.1 >= contained.0)
        || (pair.0 <= contained.1 && pair.1 >= contained.1)
}
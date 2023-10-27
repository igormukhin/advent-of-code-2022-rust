pub(crate) fn task2() {
    let input = std::fs::read_to_string("data/input2.txt").unwrap();

    let mut total_score_a: i32 = 0;
    let mut total_score_b: i32 = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        let opp_move: i32 = (bytes[0] - 'A' as u8) as i32;
        let my_code: i32 = (bytes[2] - 'X' as u8) as i32;

        // Task A
        total_score_a += my_code + 1;
        if opp_move == my_code {
            total_score_a += 3;
        } else if my_code == (opp_move + 1) % 3 {
            total_score_a += 6;
        }

        // Task B
        match my_code {
            0 => {
                total_score_b += ((opp_move + 2) % 3) + 1;
            }
            1 => {
                total_score_b += 3 + (opp_move + 1);
            }
            2 => {
                total_score_b += 6 + ((opp_move + 1) % 3) + 1;
            }
            _ => {
                panic!("Invalid move");
            }
        }
    }

    println!("Task 2a: {}", total_score_a);
    println!("Task 2b: {}", total_score_b);
}


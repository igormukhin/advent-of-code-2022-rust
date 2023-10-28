pub(crate) fn run() {
    let input = std::fs::read_to_string("data/input1.txt").unwrap();

    let mut most = 0;
    let mut most3 = [0, 0, 0];
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            if  sum > most {
                most = sum;
            }
            replace_least(&mut most3, sum);
            sum = 0;
            continue;
        }

        let num: i32 = line.parse().unwrap();
        sum += num;
    }

    println!("Task 1a: {}", most);
    println!("Task 1b: {}", most3.iter().sum::<i32>());
}

fn replace_least(arr: &mut [i32], val: i32) {
    let mut min_idx = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[min_idx] {
            min_idx = i;
        }
    }
    if arr[min_idx] < val || arr[min_idx] == 0 {
        arr[min_idx] = val;
    }
}


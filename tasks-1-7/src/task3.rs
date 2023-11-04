pub(crate) fn run() {
    let input = std::fs::read_to_string("data/input3.txt").unwrap();

    let mut stuff = [0u8; 52];
    let mut sum: usize = 0;
    for line in input.lines() {
        stuff.fill(0);

        let bytes = line.as_bytes();
        for i in 0..(bytes.len() / 2) {
            stuff[thing_to_idx(bytes[i])] += 1;
        }

        for i in (bytes.len() / 2)..bytes.len() {
            let idx = thing_to_idx(bytes[i]);
            if stuff[idx] != 0 {
                sum += idx + 1;
                break;
            }
        }
    }

    println!("Task 3a: {}", sum);

    let mut group_stuff = [0u8; 52];
    let mut elf_in_group = 0;
    let mut group_sum = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            group_stuff[thing_to_idx(bytes[i])] |= 1 << elf_in_group
        }

        if elf_in_group == 2 {
            for i in 0..group_stuff.len() {
                if group_stuff[i] == 0b111 {
                    group_sum += i + 1;
                    break;
                }
            }

            group_stuff.fill(0);
        }
        elf_in_group = (elf_in_group + 1) % 3;
    }

    println!("Task 3b: {}", group_sum);
}

fn thing_to_idx(thing: u8) -> usize {
    match thing {
        b'a'..=b'z' => (thing - b'a') as usize,
        b'A'..=b'Z' => (thing - b'A') as usize + 26,
        _ => panic!("Invalid thing"),
    }
}
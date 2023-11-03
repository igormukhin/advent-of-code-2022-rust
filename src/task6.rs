use std::fs;

struct Marker {
    counters: Vec<u8>
}

impl Marker {
    fn new() -> Marker {
        Marker {
            counters: vec![0; ('z' as u8 - 'a' as u8 + 1) as usize]
        }
    }

    fn add(&mut self, c: u8) {
        self.counters[(c - 'a' as u8) as usize] += 1;
    }

    fn remove(&mut self, c: u8) {
        self.counters[(c - 'a' as u8) as usize] -= 1;
    }

    fn is_unique(&self) -> bool {
        self.counters.iter().all(|&x| x <= 1)
    }
}

pub(crate) fn run() {
    let input = fs::read_to_string("data/input6.txt").unwrap();

    println!("Task 6a: {}", find_marker(input.as_bytes(), 4).unwrap());
    println!("Task 6b: {}", find_marker(input.as_bytes(), 14).unwrap());
}

fn find_marker(bytes: &[u8], len: usize) -> Option<usize> {
    let mut marker = Marker::new();
    for i in 0..bytes.len() {
        marker.add(bytes[i]);
        if i >= len {
            marker.remove(bytes[i - len]);

            if marker.is_unique() {
                return Some(i + 1);
            }
        }
    }
    None
}

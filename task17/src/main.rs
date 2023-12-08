use std::collections::HashMap;

fn main() {
    let winds = include_str!("../input.txt");

    let stone_templates = vec![
        vec![0b11110],
        vec![0b01000, 0b11100, 0b01000],
        // upside-down
        vec![0b11100, 0b00100, 0b00100],
        vec![0b10000, 0b10000, 0b10000, 0b10000],
        vec![0b11000, 0b11000],
    ];

    part1(winds, &stone_templates);
    part2(winds, &stone_templates);
}

fn part1(winds: &str, stone_templates: &Vec<Vec<i32>>) {
    let mut floor: Vec<i32> = vec![];
    drop_stones(&mut floor, stone_templates, winds, &mut 0, 2022);

    println!("Part 1: {}", floor.len());
}

fn part2(winds: &str, stone_templates: &Vec<Vec<i32>>) {
    let mut cave: Vec<i32> = vec![];
    let mut wind_idx = 0;
    let mut template_idx = 0;
    let tail_height = 50;
    let mut positions_map: HashMap<(usize, usize), Vec<(usize, u64)>> = HashMap::new();
    let mut repetition_skipped = false;
    let mut skipped_height = 0u64;
    let stones_max = 1000000000000u64;
    let mut stone_idx = 0u64;
    while stone_idx < stones_max {
        template_idx = template_idx % stone_templates.len();
        drop_stone(&mut cave, &stone_templates[template_idx], winds, &mut wind_idx);
        wind_idx = wind_idx % winds.len();

        if !repetition_skipped && cave.len() >= tail_height {
            let key = (template_idx, wind_idx);
            if positions_map.contains_key(&key) {
                let positions = positions_map[&key].clone();
                for (prev_cave_len, prev_stone_idx) in positions {
                    let mut matching = true;
                    for i in 0..tail_height {
                        if cave[cave.len() - 1 - i] != cave[prev_cave_len - 1 - i] {
                            matching = false;
                            break;
                        }
                    }

                    if matching {
                        println!("Match found between");
                        println!("Curr: height: {}, stone: {}, wind: {}", cave.len(), stone_idx, wind_idx);
                        println!("Prev: height: {}, stone: {}, wind: {}", prev_cave_len, prev_stone_idx, wind_idx);
                        //display(&cave);

                        let plate_height = cave.len() - prev_cave_len;
                        let plate_stones = stone_idx - prev_stone_idx;
                        let plates_left = (stones_max - stone_idx - 1) / plate_stones;
                        skipped_height = plates_left * plate_height as u64;
                        stone_idx += plates_left * plate_stones;

                        repetition_skipped = true;
                        break;
                    }
                }
            }

            positions_map.entry(key).or_insert(vec![]).push((cave.len(), stone_idx));
        }

        stone_idx += 1;
        template_idx += 1;
    }

    println!("Part 2: {}", cave.len() as u64 + skipped_height);
}

fn drop_stones(floor: &mut Vec<i32>, stone_templates: &Vec<Vec<i32>>, winds: &str, wind_idx: &mut usize, drop_count: usize) {
    for stone_idx in 0..drop_count {
        let stone_template = &stone_templates[stone_idx % stone_templates.len()];
        drop_stone(floor, stone_template, winds, wind_idx);
    }
}

fn drop_stone(floor: &mut Vec<i32>, stone_template: &Vec<i32>, winds: &str, wind_idx: &mut usize) {
    let mut stone = vec![0, 0, 0];
    stone_template.iter().for_each(|&s| {
        stone.push(s);
    });

    for shift in 0.. {
        if winds.len() > 0 {
            let wind = winds.as_bytes()[*wind_idx % winds.len()];
            if wind != b'>' && wind != b'<' {
                panic!("Invalid wind: {}", wind);
            }
            *wind_idx += 1;

            let check_bit = if wind == b'<' { 0b1000000 } else { 0b0000001 };
            let can_hit_wall = stone.iter().any(|&s| s & check_bit != 0);
            if !can_hit_wall {
                let can_hit_floored = stone.iter().enumerate().any(|(row, &s)| {
                    let base_idx = floor.len() as i32 - shift + row as i32;
                    if base_idx < 0 || base_idx >= floor.len() as i32 {
                        return false;
                    }

                    let blown = if wind == b'<' { s << 1 } else { s >> 1 };
                    if blown & floor[base_idx as usize] != 0 {
                        return true;
                    }

                    return false;
                });

                if !can_hit_floored {
                    stone.iter_mut().for_each(|s|
                        if wind == b'<' { *s <<= 1 } else { *s >>= 1 });
                }
            }
        }

        let floor_len = floor.len();
        let can_move = stone.iter().enumerate().all(|(row, &s)| {
            let base_idx = floor_len as i32 - 1 - shift + row as i32;
            if base_idx < 0 {
                return s == 0;
            }
            if base_idx >= floor_len as i32 {
                return true;
            }

            if s & floor[base_idx as usize] != 0 {
                return false;
            }

            return true;
        });

        if !can_move {
            stone.iter().enumerate().for_each(|(row, &s)| {
                let base_idx = floor_len as i32 - shift + row as i32;
                if base_idx < 0 {
                    return;
                } else if base_idx < floor_len as i32 {
                    floor[base_idx as usize] |= s;
                } else {
                    floor.push(s);
                }
            });

            break;
        }
    }
}

fn display(floor: &Vec<i32>) {
    for row in (0..floor.len()).rev() {
        let floor_byte = floor[row];
        println!("{:08b}", floor_byte as u8);
    }
}
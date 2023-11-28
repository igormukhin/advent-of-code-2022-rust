use std::cmp::{max, min};
use regex::Regex;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Reading {
    sensor: Coord,
    beacon: Coord,
}

#[derive(Debug, Copy, Clone)]
struct LineSegment {
    start: i32,
    end: i32,
}

impl LineSegment {
    fn squash_into(&self, min_val: i32, max_val: i32) -> Option<LineSegment> {
        let start = max(self.start, min_val);
        let end = min(self.end, max_val);
        if start < end {
            Some(LineSegment { start, end })
        } else {
            None
        }
    }

    fn length(&self) -> i32 {
        self.end - self.start + 1
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(
        "Sensor at x=(?P<sensor_x>-?\\d+), y=(?P<sensor_y>-?\\d+): \
        closest beacon is at x=(?P<beacon_x>-?\\d+), y=(?P<beacon_y>-?\\d+)").unwrap();
    let readings = input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let sensor = Coord {
                x: caps["sensor_x"].parse().unwrap(),
                y: caps["sensor_y"].parse().unwrap(),
            };
            let beacon = Coord {
                x: caps["beacon_x"].parse().unwrap(),
                y: caps["beacon_y"].parse().unwrap(),
            };

            Reading { sensor, beacon }
        })
        .collect::<Vec<_>>();

    //
    task_a(&readings);
    task_b(&readings);
}

fn task_b(readings: &Vec<Reading>) {
    let min = 0;
    let max = 4000000;
    for y in min..=max {
        let mut segments = get_segments(readings, y);
        segments.sort_by_key(|ls| ls.start);

        let filtered = filter_to_range(&segments, min, max);
        let merged = merge_sorted_segments(&filtered);
        if merged.len() > 1 {
            let x = merged[0].end + 1;
            println!("({}, {}) = {}", x, y, x as u64 * 4000000 + y as u64);
            break;
        }
    }
}

fn task_a(readings: &Vec<Reading>) {
    let main_line = 2000000;

    let mut segments = get_segments(readings, main_line);
    segments.sort_by_key(|ls| ls.start);

    let merged = merge_sorted_segments(&segments);
    let sum = merged.iter().map(|ls| ls.length()).sum::<i32>();
    let beacons_on_mainline = readings.iter()
        .filter(|r| r.beacon.y == main_line)
        .map(|r| r.beacon.x)
        .collect::<std::collections::HashSet<_>>()
        .len() as i32;

    println!("{}", sum - beacons_on_mainline);
}

fn get_segments(readings: &Vec<Reading>, check_row_y: i32) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    for reading in readings {
        let mdist = (reading.sensor.x - reading.beacon.x).abs()
            + (reading.sensor.y - reading.beacon.y).abs();
        let to_mainline = (check_row_y - reading.sensor.y).abs();
        if to_mainline > mdist {
            continue;
        } else {
            let rest = mdist - to_mainline;
            segments.push(LineSegment {
                start: reading.sensor.x - rest,
                end: reading.sensor.x + rest,
            });
        }
    }
    segments
}

fn filter_to_range(segments: &Vec<LineSegment>, min_val: i32, max_val: i32) -> Vec<LineSegment> {
    let mut result = Vec::new();
    for ls in segments {
        if ls.start > max_val {
            break;
        } else if ls.end < min_val {
            continue;
        } else {
            // squashing:
            let squashed = ls.squash_into(min_val, max_val);
            if let Some(ls) = squashed {
                result.push(ls);
            }
        }
    }
    result
}

fn merge_sorted_segments(segments: &Vec<LineSegment>) -> Vec<LineSegment> {
    let mut result = Vec::new();
    if segments.len() == 0 {
        return result;
    }

    let mut current = segments[0];
    for ls in segments {
        if ls.start > current.end + 1 {
            result.push(current);
            current = ls.clone();
        } else if ls.end > current.end {
            current.end = ls.end;
        }
    }
    result.push(current);
    result
}

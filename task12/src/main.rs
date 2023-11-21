use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn neighbours(&self) -> Vec<Coord> {
        vec![
            Coord { x: self.x - 1, y: self.y },
            Coord { x: self.x + 1, y: self.y },
            Coord { x: self.x, y: self.y - 1 },
            Coord { x: self.x, y: self.y + 1 }
        ]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Pixel {
    Start,
    Finish,
    Road { height: i32 }
}

impl Pixel {
    fn elevation(&self) -> i32 {
        match self {
            Pixel::Start => 0,
            Pixel::Road { height } => *height,
            Pixel::Finish => 'z' as i32 - 'a' as i32
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Cell {
    coord: Coord,
    pixel: Pixel
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = parse_input(input);

    // task A
    let start_coord = *map.iter().find(|(_, pixel)| **pixel == Pixel::Start).unwrap().0;
    println!("Task A: {:?}", shortest_path(&map, &vec![start_coord]).unwrap());

    // task B
    let low_coords = map.iter()
        .filter(|(_, pixel)| pixel.elevation() == 0)
        .map(|(coord, _)| coord.clone())
        .collect::<Vec<Coord>>();
    println!("Task B: {:?}", shortest_path(&map, &low_coords).unwrap());
}

fn shortest_path(map: &HashMap<Coord, Pixel>, start_coords: &Vec<Coord>) -> Option<i32> {
    let mut queue = start_coords.clone();

    let mut costs: HashMap<Coord, i32> = HashMap::new();
    start_coords.iter().for_each(|coord| { costs.insert(*coord, 0); });

    while !queue.is_empty() {
        let coord = queue.remove(0);
        let cost = costs[&coord];

        for neighbour in coord.neighbours() {
            if !map.contains_key(&neighbour) {
                continue;
            }

            if map[&coord].elevation() < map[&neighbour].elevation() - 1 {
                continue;
            }

            let new_cost = cost + 1;

            let neighbour_cost = match costs.get(&neighbour) {
                Some(cost) => *cost,
                None => i32::MAX
            };

            if new_cost < neighbour_cost {
                costs.insert(neighbour, new_cost);
                queue.push(neighbour);
            }
        }
    }

    let finish_coord = *map.iter().find(|(_, pixel)| **pixel == Pixel::Finish).unwrap().0;
    costs.get(&finish_coord).map(|cost| *cost)
}

fn parse_input(input: String) -> HashMap<Coord, Pixel> {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, pixel) in line.chars().enumerate() {
            let coord = Coord { x: x as i32, y: y as i32 };
            let pixel = match pixel {
                'S' => Pixel::Start,
                'E' => Pixel::Finish,
                _ => Pixel::Road { height: pixel as i32 - 'a' as i32 }
            };

            map.insert(coord, pixel);
        }
    }

    map
}

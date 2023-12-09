use std::collections::{HashSet, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Cube { x: i32, y: i32, z: i32 }

impl Cube {
    fn neighbours(&self) -> Vec<Cube> {
        vec![
            Cube { x: self.x + 1, y: self.y, z: self.z },
            Cube { x: self.x - 1, y: self.y, z: self.z },
            Cube { x: self.x, y: self.y + 1, z: self.z },
            Cube { x: self.x, y: self.y - 1, z: self.z },
            Cube { x: self.x, y: self.y, z: self.z + 1 },
            Cube { x: self.x, y: self.y, z: self.z - 1 },
        ]
    }
}

struct RectBody {
    min: Cube,
    max: Cube,
}

impl RectBody {
    fn includes(&self, cube: &Cube) -> bool {
        self.min.x <= cube.x && cube.x <= self.max.x
            && self.min.y <= cube.y && cube.y <= self.max.y
            && self.min.z <= cube.z && cube.z <= self.max.z
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let cubes = input.lines().map(|line| {
        let mut coords = line.split(",")
            .map(|token| token.parse().unwrap())
            .collect::<Vec<i32>>();
        Cube {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }).collect::<HashSet<Cube>>();

    let non_touching_edges = cubes.iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| !cubes.contains(c))
        .count();
    println!("Part 1: {}", non_touching_edges);

    let first_cube = cubes.iter().next().unwrap().clone();
    let outer_body = Some(cubes.iter().fold(
        RectBody { min: first_cube, max: first_cube },
        |acc, cube| RectBody {
            min: Cube {
                x: acc.min.x.min(cube.x),
                y: acc.min.y.min(cube.y),
                z: acc.min.z.min(cube.z),
            },
            max: Cube {
                x: acc.max.x.max(cube.x),
                y: acc.max.y.max(cube.y),
                z: acc.max.z.max(cube.z),
            },
        }
    )).map(|rect_body| RectBody {
        // expand by one in all directions
        min: Cube {
            x: rect_body.min.x - 1,
            y: rect_body.min.y - 1,
            z: rect_body.min.z - 1,
        },
        max: Cube {
            x: rect_body.max.x + 1,
            y: rect_body.max.y + 1,
            z: rect_body.max.z + 1,
        },
    }).unwrap();

    let mut flooded = calc_flooded(&cubes, &outer_body);
    let surface_area = cubes.iter()
        .flat_map(|c| c.neighbours())
        .filter(|c| flooded.contains(c))
        .count();

    println!("Part 2: {}", surface_area);
}

fn calc_flooded(cubes: &HashSet<Cube>, outer_body: &RectBody) -> HashSet<Cube> {
    let mut queue = VecDeque::new();
    queue.push_back(outer_body.min);
    let mut flooded = HashSet::new();

    while let Some(cube) = queue.pop_front() {
        if !outer_body.includes(&cube) || cubes.contains(&cube) || flooded.contains(&cube) {
            continue;
        }

        flooded.insert(cube);
        queue.extend(cube.neighbours().iter());
    }

    flooded
}

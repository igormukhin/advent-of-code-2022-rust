struct Coord(usize, usize);

pub fn main() {
    let input = include_str!("../input.txt");

    let forest: Vec<Vec<u8>> = input.lines()
        .map(|line| line.as_bytes().iter().map(|c| c - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let size = forest.len();

    task_a(&forest, &size);
    task_b(&forest, &size);
}

fn task_a(forest: &Vec<Vec<u8>>, size: &usize) {
    let mut visibles: Vec<Vec<bool>> = vec![vec![false; *size]; *size];

    mark_visibles(forest, &mut visibles, size,
                  |Coord(x, y)| Coord(*x, *y));
    mark_visibles(forest, &mut visibles, size,
                  |Coord(x, y)| Coord(*x, *size - 1 - *y));
    mark_visibles(forest, &mut visibles, size,
                  |Coord(x, y)| Coord(*y, *x));
    mark_visibles(forest, &mut visibles, size,
                  |Coord(x, y)| Coord(*size - 1 - *y, *x));

    let count = visibles.iter().map(|row| row.iter().filter(|&&b| b).count()).sum::<usize>();
    println!("Task 8a: {}", count);
}

fn mark_visibles(forest: &Vec<Vec<u8>>, visibles: &mut Vec<Vec<bool>>,
                 size: &usize,
                 coord_mapper: impl Fn(&Coord) -> Coord) {
    for i in 0..*size {
        let mut highest: Option<u8> = None;
        for j in 0..*size {
            let coord = coord_mapper(&Coord(i, j));
            let visible = match(&highest, forest[coord.0][coord.1]) {
                (None, _) => true,
                (Some(m), h) => h > *m,
            };
            if visible {
                highest = Some(forest[coord.0][coord.1]);
                visibles[coord.0][coord.1] = true;
            }
        }
    }
}

fn task_b(forest: &Vec<Vec<u8>>, size: &usize) {
    let max = (0..*size).map(|i|
                    (0..*size).map(|j|
                        scenic_score(forest, size, &Coord(i, j))
                    ).max().unwrap()
                ).max().unwrap();
    println!("Task 8b: {}", max);
}

fn scenic_score(forest: &Vec<Vec<u8>>, size: &usize, pos: &Coord) -> usize {
    return
        visible_trees(forest, size, pos, |x, y| (*x as isize - 1, *y as isize)) *
        visible_trees(forest, size, pos, |x, y| (*x as isize, *y as isize - 1)) *
        visible_trees(forest, size, pos, |x, y| (*x as isize + 1, *y as isize)) *
        visible_trees(forest, size, pos, |x, y| (*x as isize, *y as isize + 1));
}

fn visible_trees(forest: &Vec<Vec<u8>>, size: &usize, starting_pos: &Coord,
                 stepper: impl Fn(&isize, &isize) -> (isize, isize)) -> usize {
    let mut pos = (starting_pos.0 as isize, starting_pos.1 as isize);
    let my_height = forest[starting_pos.0][starting_pos.1];
    let mut count = 0;
    loop {
        pos = stepper(&pos.0, &pos.1);
        if pos.0 < 0 || pos.0 >= *size as isize
            || pos.1 < 0 || pos.1 >= *size as isize {
            break;
        }
        count += 1;
        if forest[pos.0 as usize][pos.1 as usize] >= my_height {
            break;
        }
    }
    count
}
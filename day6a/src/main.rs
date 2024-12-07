use std::{collections::HashSet, fs};

enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn rotate(&mut self) {
        use Direction::*;
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }
}

fn get_guard_position(m: &[Vec<char>]) -> (usize, usize) {
    for (i, row) in m.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return (j, i);
            }
        }
    }
    panic!("failed to find the guard !!!");
}

fn move_to(direction: &Direction, from: (usize, usize)) -> (usize, usize) {
    use Direction::*;
    match direction {
        North => (from.0, from.1 - 1),
        South => (from.0, from.1 + 1),
        East => (from.0 + 1, from.1),
        West => (from.0 - 1, from.1),
    }
}

fn within_bounds(pos: (usize, usize), width: usize, height: usize) -> bool {
    (0..width).contains(&pos.0) && (0..height).contains(&pos.1)
}

fn main() {
    let input = fs::read_to_string("inputs/6.txt").unwrap();

    let matrix: Vec<Vec<char>> = input.lines().map(|n| n.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();

    let mut seen = HashSet::new();

    let mut current_direction = Direction::North;
    let mut current_position = get_guard_position(&matrix);

    while within_bounds(current_position, width, height) {
        if !seen.contains(&current_position) {
            println!("seen {:?}", current_position);
            seen.insert(current_position);
        }

        let forward = move_to(&current_direction, current_position);
        if within_bounds(forward, width, height) && matrix[forward.1][forward.0] == '#' {
            current_direction.rotate();
        } else {
            current_position = forward;
        }
    }

    println!("{}", seen.len());
}

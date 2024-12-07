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

fn get_guard_position(m: &[Vec<char>]) -> (i64, i64) {
    for (i, row) in m.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                return (j as i64, i as i64);
            }
        }
    }
    panic!("failed to find the guard !!!");
}

fn move_to(direction: &Direction, from: (i64, i64)) -> (i64, i64) {
    use Direction::*;
    match direction {
        North => (from.0, from.1 - 1),
        South => (from.0, from.1 + 1),
        East => (from.0 + 1, from.1),
        West => (from.0 - 1, from.1),
    }
}

fn within_bounds(pos: (i64, i64), width: i64, height: i64) -> bool {
    (0..width).contains(&pos.0) && (0..height).contains(&pos.1)
}

fn main() {
    let input = fs::read_to_string("inputs/6.txt").unwrap();

    let matrix: Vec<Vec<char>> = input.lines().map(|n| n.chars().collect()).collect();
    let height = matrix.len() as i64;
    let width = matrix[0].len() as i64;

    let mut seen = HashSet::new();

    let mut current_direction = Direction::North;
    let mut current_position = get_guard_position(&matrix);

    while within_bounds(current_position, width, height) {
        seen.insert(current_position);

        let forward = move_to(&current_direction, current_position);
        if within_bounds(forward, width, height)
            && matrix[forward.1 as usize][forward.0 as usize] == '#'
        {
            current_direction.rotate();
        } else {
            current_position = forward;
        }
    }

    println!("{} positions", seen.len());
}

use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
                return (j.try_into().unwrap(), i.try_into().unwrap());
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct GuardState {
    direction: Direction,
    position: (i64, i64),
}

fn main() {
    let input = fs::read_to_string("inputs/6.txt").unwrap();

    let mut matrix: Vec<Vec<char>> = input.lines().map(|n| n.chars().collect()).collect();

    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut state_matrix: HashMap<(i64, i64), HashSet<Direction>> = HashMap::new();

    let mut potentials = HashSet::new();

    let mut current_direction = Direction::North;
    let initial_position = get_guard_position(&matrix);
    let mut current_position = initial_position;

    while within_bounds(current_position, width, height) {
        potentials.insert(current_position);

        state_matrix
            .entry(current_position)
            .or_default()
            .insert(current_direction);

        let forward = move_to(&current_direction, current_position);
        if within_bounds(forward, width, height)
            && matrix[forward.1 as usize][forward.0 as usize] == '#'
        {
            current_direction.rotate();
        } else {
            current_position = forward;
        }
    }

    potentials.remove(&initial_position);

    let mut option_count = 0;

    for (pot_x, pot_y) in potentials {
        println!("trying ({}, {})", pot_x, pot_y);
        matrix[pot_y as usize][pot_x as usize] = '#';

        let mut seen_states = HashSet::new();

        let mut current_position = initial_position;
        let mut current_direction = Direction::North;

        let mut is_loop = false;

        while within_bounds(current_position, width, height) {
            let current_state = GuardState {
                direction: current_direction,
                position: current_position,
            };

            if seen_states.contains(&current_state) {
                is_loop = true;
                break;
            }
            seen_states.insert(current_state);

            let forward = move_to(&current_direction, current_position);
            if within_bounds(forward, width, height)
                && matrix[forward.1 as usize][forward.0 as usize] == '#'
            {
                current_direction.rotate();
            } else {
                current_position = forward;
            }
        }

        if is_loop {
            println!("LOOP!");
            option_count += 1;
        }

        matrix[pot_y as usize][pot_x as usize] = '.';
    }

    println!("{} options", option_count);
}

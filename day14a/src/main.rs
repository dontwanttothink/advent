use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn extract_numbers(s: &str) -> Vec<i64> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[derive(Debug)]
struct Robot {
    initial_position: (i64, i64),
    velocity: (i64, i64),
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const ELAPSED_SECONDS: i64 = 100;

enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
    Limbo,
}
impl From<(i64, i64)> for Quadrant {
    fn from((x, y): (i64, i64)) -> Self {
        const V_MIDDLE: i64 = HEIGHT / 2;
        const H_MIDDLE: i64 = WIDTH / 2;

        if x > H_MIDDLE && y < V_MIDDLE {
            Self::First
        } else if x < H_MIDDLE && y < V_MIDDLE {
            Self::Second
        } else if x < H_MIDDLE && y > V_MIDDLE {
            Self::Third
        } else if x > H_MIDDLE && y > V_MIDDLE {
            Self::Fourth
        } else {
            Self::Limbo
        }
    }
}

fn main() {
    let file = File::open("inputs/14.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut robots = vec![];
    for line in reader.lines() {
        let line = line.expect("Failed to read a line.");

        let robot = extract_numbers(&line);
        robots.push(Robot {
            initial_position: (robot[0], robot[1]),
            velocity: (robot[2], robot[3]),
        })
    }

    let mut new_positions = vec![];
    for robot in robots {
        let delta = (
            robot.velocity.0 * ELAPSED_SECONDS,
            robot.velocity.1 * ELAPSED_SECONDS,
        );

        let new_position = (
            (robot.initial_position.0 + delta.0).rem_euclid(WIDTH),
            (robot.initial_position.1 + delta.1).rem_euclid(HEIGHT),
        );

        new_positions.push(new_position);
    }

    let mut first_quad = 0;
    let mut second_quad = 0;
    let mut third_quad = 0;
    let mut fourth_quad = 0;

    for position in new_positions {
        match Quadrant::from(position) {
            Quadrant::First => first_quad += 1,
            Quadrant::Second => second_quad += 1,
            Quadrant::Third => third_quad += 1,
            Quadrant::Fourth => fourth_quad += 1,
            Quadrant::Limbo => {}
        }
    }

    let safety_factor = first_quad * second_quad * third_quad * fourth_quad;

    println!("A safety factor of {} ", safety_factor);
}

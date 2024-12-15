use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use dialoguer::Confirm;

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

    // Throw experimentation, I saw the robots get closer to each other
    // horizontally every 101 steps (the width!) starting at second
    // 28. The first time I wrote down, though, was 230.
    let mut elapsed_seconds = 230;
    loop {
        let mut matrix = [[0; HEIGHT as usize]; WIDTH as usize];
        for robot in &robots {
            let delta = (
                robot.velocity.0 * elapsed_seconds,
                robot.velocity.1 * elapsed_seconds,
            );

            let new_position = (
                (robot.initial_position.0 + delta.0).rem_euclid(WIDTH),
                (robot.initial_position.1 + delta.1).rem_euclid(HEIGHT),
            );

            matrix[new_position.0 as usize][new_position.1 as usize] += 1;
        }

        println!("\nAfter {} seconds:", elapsed_seconds);

        for row in &matrix {
            for freq in row {
                print!("{}", if *freq == 0 { ' ' } else { '█' })
            }
            println!();
        }

        let confirmation = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();

        if !confirmation {
            break;
        }

        elapsed_seconds += 101;
    }
}

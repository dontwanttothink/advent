use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Obstacle {
    Box,
    Wall,
}

fn throw_ray(
    matrix: &[Vec<Option<Obstacle>>],
    step: (i64, i64),
    position: (i64, i64),
) -> Option<(i64, i64)> {
    let mut current = (position.0 + step.0, position.1 + step.1);
    let mut out = None;

    let height = matrix.len().try_into().unwrap();
    let width = matrix[0].len().try_into().unwrap();

    while (0..height).contains(&current.1)
        && (0..width).contains(&current.0)
        && (matrix[current.1 as usize][current.0 as usize] == Some(Obstacle::Box))
    {
        current = (current.0 + step.0, current.1 + step.1);
        if (0..height).contains(&current.1)
            && (0..width).contains(&current.0)
            && matrix[current.1 as usize][current.0 as usize].is_none()
        {
            out = Some(current);
        }
    }

    out
}

fn main() {
    let input = fs::read_to_string("inputs/15.txt").expect("Failed to read file");
    let (matrix_str, movements_str) = {
        let mut parts = input.split("\n\n");
        (parts.next().unwrap(), parts.next().unwrap())
    };

    let mut current_position: (i64, i64) = {
        let mut pos = None;
        'search: for (y, line) in matrix_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    pos = Some((x, y));
                    break 'search;
                }
            }
        }
        let pos = pos.unwrap();
        (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
    };
    let mut matrix: Vec<Vec<Option<Obstacle>>> = matrix_str
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == '#' {
                        Some(Obstacle::Wall)
                    } else if c == 'O' {
                        Some(Obstacle::Box)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let movements = movements_str.lines().flat_map(|l| {
        l.chars().map(|c| {
            if c == '^' {
                (0, -1)
            } else if c == '<' {
                (-1, 0)
            } else if c == '>' {
                (1, 0)
            } else if c == 'v' {
                (0, 1)
            } else {
                panic!("Invalid movement found")
            }
        })
    });

    let height = matrix.len().try_into().unwrap();
    let width = matrix[0].len().try_into().unwrap();

    for step in movements {
        if let Some(space) = throw_ray(&matrix, step, current_position) {
            let mut current = (space.0 - step.0, space.1 - step.1);

            while (0..height).contains(&current.1)
                && (0..width).contains(&current.0)
                && (matrix[current.1 as usize][current.0 as usize] == Some(Obstacle::Box))
            {
                let empty = (current.0 + step.0, current.1 + step.1);
                matrix[empty.1 as usize][empty.0 as usize] =
                    matrix[current.1 as usize][current.0 as usize];
                matrix[current.1 as usize][current.0 as usize] = None;

                current = (current.0 - step.0, current.1 - step.1);
            }
        }
        // if free at step then move
        let maybe_new_pos = (current_position.0 + step.0, current_position.1 + step.1);
        if (0..height).contains(&maybe_new_pos.1)
            && (0..width).contains(&maybe_new_pos.0)
            && matrix[maybe_new_pos.1 as usize][maybe_new_pos.0 as usize].is_none()
        {
            current_position = maybe_new_pos;
        }
    }

    let mut sum_of_gps_coordinates = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, obs) in row.iter().enumerate() {
            print!(
                "{}",
                if *obs == Some(Obstacle::Box) {
                    'O'
                } else if *obs == Some(Obstacle::Wall) {
                    '#'
                } else if (j, i)
                    == (
                        current_position.0.try_into().unwrap(),
                        current_position.1.try_into().unwrap()
                    )
                {
                    '@'
                } else {
                    '.'
                }
            );

            if *obs == Some(Obstacle::Box) {
                let gps_coordinate = i * 100 + j;
                sum_of_gps_coordinates += gps_coordinate;
            }
        }
        println!();
    }
    println!("A sum of GPS coordinates of {}", sum_of_gps_coordinates);
}

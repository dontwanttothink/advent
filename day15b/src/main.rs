use std::fs;

fn can_move_in(step: (i64, i64), position: (i64, i64), matrix: &[Vec<char>]) -> bool {
    let pos_x: usize = position.0.try_into().unwrap();
    let pos_y: usize = position.1.try_into().unwrap();

    if matrix[pos_y][pos_x] == '#' {
        // We can't move a wall.
        return false;
    }

    let height: i64 = matrix[0].len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let new_positions = [
        Some((position.0 + step.0, position.1 + step.1)),
        //
        // This might be a box, and boxes are thick. In that case, we need to
        // consider the possible position of the pair, since they move together.
        {
            // We only consider both if the transformation is vertical. Otherwise,
            // this would get us in an infinite loop. If the transformation is
            // horizontal, checking either one is enough, because the coordinates
            // are parallel to the movement that they're making. Does that make
            // sense? Sorry

            if step.1 == 0 {
                None
            } else {
                let matching_pair = if matrix[pos_y][pos_x] == '[' {
                    Some((position.0 + 1, position.1))
                } else if matrix[pos_y][pos_x] == ']' {
                    Some((position.0 - 1, position.1))
                } else {
                    None
                };
                matching_pair.map(|p| (p.0 + step.0, p.1 + step.1))
            }
        },
    ];

    for new_pos in new_positions {
        if new_pos.is_none() {
            continue;
        }
        let new_pos = new_pos.unwrap();

        if !(0..width).contains(&new_pos.0) || !(0..height).contains(&new_pos.1) {
            // We can't move outside the matrix.
            return false;
        }

        let new_pos_x: usize = new_pos.0.try_into().unwrap();
        let new_pos_y: usize = new_pos.1.try_into().unwrap();

        // We can move into an empty space, and we can also move into a space that, itself, can move.
        if !(matrix[new_pos_y][new_pos_x] == '.' || can_move_in(step, new_pos, matrix)) {
            // Neither option worked, so we return false.
            return false;
        }
    }
    true // We can move both necessary ones.
}

fn do_move_in(step: (i64, i64), position: (i64, i64), matrix: &mut [Vec<char>]) {
    let pos_x: usize = position.0.try_into().unwrap();
    let pos_y: usize = position.1.try_into().unwrap();

    let new_positions = if matrix[pos_y][pos_x] == '.' || step.1 == 0 {
        [Some((position.0 + step.0, position.1 + step.1)), None]
    } else if matrix[pos_y][pos_x] == ']' {
        [
            Some((position.0 + step.0, position.1 + step.1)),
            Some((position.0 - 1 + step.0, position.1 + step.1)),
        ]
    } else if matrix[pos_y][pos_x] == '[' {
        [
            Some((position.0 + step.0, position.1 + step.1)),
            Some((position.0 + 1 + step.0, position.1 + step.1)),
        ]
    } else {
        panic!(
            "unexpected combination! {} {:?}",
            matrix[pos_y][pos_x], step
        );
    };

    for new_pos in new_positions {
        if new_pos.is_none() {
            continue;
        }
        let new_pos = new_pos.unwrap();
        let old_pos = (new_pos.0 - step.0, new_pos.1 - step.1);

        let new_x: usize = new_pos.0.try_into().unwrap();
        let new_y: usize = new_pos.1.try_into().unwrap();

        let old_x: usize = old_pos.0.try_into().unwrap();
        let old_y: usize = old_pos.1.try_into().unwrap();

        // We can move this directly if the new position just has empty space.
        if matrix[new_y][new_x] == '.' {
        } else {
            // Otherwise, we need to move the object blocking us.
            do_move_in(step, new_pos, matrix);
        }
        let to_move = matrix[old_y][old_x];
        matrix[old_y][old_x] = '.';
        matrix[new_y][new_x] = to_move;
    }
}

fn main() {
    let input = fs::read_to_string("inputs/15.txt").expect("Failed to read file");
    let (matrix_str, movements_str) = {
        let mut parts = input.split("\n\n");
        (parts.next().unwrap(), parts.next().unwrap())
    };

    let mut matrix: Vec<Vec<char>> = matrix_str
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| {
                    if c == '#' {
                        "##".chars()
                    } else if c == 'O' {
                        "[]".chars()
                    } else if c == '.' {
                        "..".chars()
                    } else if c == '@' {
                        "@.".chars()
                    } else {
                        panic!("Found invalid character");
                    }
                })
                .collect()
        })
        .collect();

    let mut current_position: (i64, i64) = {
        let mut pos = None;
        'search: for (y, line) in matrix.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch == '@' {
                    pos = Some((x, y));
                    matrix[y][x] = '.';
                    break 'search;
                }
            }
        }
        let pos = pos.unwrap();
        (pos.0.try_into().unwrap(), pos.1.try_into().unwrap())
    };

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
                panic!("Invalid movement")
            }
        })
    });

    for step in movements {
        if can_move_in(step, current_position, &matrix) {
            do_move_in(step, current_position, &mut matrix);
            current_position = (current_position.0 + step.0, current_position.1 + step.1);
        }
    }

    for (y, row) in matrix.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if (x as i64, y as i64) != current_position {
                print!("{}", ch);
            } else {
                print!("@");
            }
        }
        println!();
    }

    let mut sum_of_gps_coordinates = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            if *ch == '[' {
                let gps_coordinate = y * 100 + x;
                sum_of_gps_coordinates += gps_coordinate;
            }
        }
    }

    println!("A sum of GPS coordinates of {}", sum_of_gps_coordinates);
}

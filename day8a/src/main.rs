use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|n| n.chars().collect()).collect();

    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut radio_positions: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let x: i64 = x.try_into().unwrap();
            let y: i64 = y.try_into().unwrap();

            if ch != '.' {
                radio_positions.entry(ch).or_default().push((x, y));
            }
        }
    }

    let mut antinode_positions = HashSet::new();

    for (_ch, positions) in radio_positions {
        for first_position in &positions {
            for second_position in &positions {
                if first_position != second_position {
                    let difference = (
                        second_position.0 - first_position.0,
                        second_position.1 - first_position.1,
                    );

                    let antinode_position = (
                        second_position.0 + difference.0,
                        second_position.1 + difference.1,
                    );

                    if (0..width).contains(&antinode_position.0)
                        && (0..height).contains(&antinode_position.1)
                    {
                        antinode_positions.insert(antinode_position);
                    }
                }
            }
        }
    }

    println!("{} unique antinode positions", antinode_positions.len());
}

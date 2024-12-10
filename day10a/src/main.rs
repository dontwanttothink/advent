use std::{collections::HashSet, fs};

fn get_neighbors((x, y): (i64, i64)) -> [(i64, i64); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn get_score(trailhead: (i64, i64), matrix: &Vec<Vec<i64>>, width: i64, height: i64) -> i64 {
    let mut found = 0;
    let mut to_explore = vec![trailhead];

    let mut seen = HashSet::new();
    seen.insert(trailhead);

    while !to_explore.is_empty() {
        let mut new_to_explore = vec![];

        for source in to_explore {
            let source_altitude = matrix[source.1 as usize][source.0 as usize];
            for neighbor in get_neighbors(source) {
                if !seen.contains(&neighbor)
                    && (0..width).contains(&neighbor.0)
                    && (0..height).contains(&neighbor.1)
                    && matrix[neighbor.1 as usize][neighbor.0 as usize] == source_altitude + 1
                {
                    let neighbor_altitude = matrix[neighbor.1 as usize][neighbor.0 as usize];
                    if neighbor_altitude == 9 {
                        found += 1;
                    } else {
                        new_to_explore.push(neighbor);
                    }
                    seen.insert(neighbor);
                }
            }
        }

        to_explore = new_to_explore;
    }

    found
}

fn main() {
    let input = fs::read_to_string("inputs/10.txt").expect("Failed to read input");
    let matrix: Vec<Vec<i64>> = input
        .lines()
        .map(|n| {
            n.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut trailheads = vec![];
    for (y, line) in matrix.iter().enumerate() {
        for (x, &altitude) in line.iter().enumerate() {
            let x: i64 = x.try_into().unwrap();
            let y: i64 = y.try_into().unwrap();
            if altitude == 0 {
                trailheads.push((x, y));
            }
        }
    }

    let mut trailhead_scores = 0;
    for trailhead in trailheads {
        trailhead_scores += get_score(trailhead, &matrix, width, height);
    }

    println!("A total trailhead score of {}", trailhead_scores);
}

use std::{collections::HashMap, fs};

fn get_neighbors((x, y): (i64, i64)) -> [(i64, i64); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

fn lies_inside((x, y): (i64, i64), matrix: &Vec<Vec<i64>>) -> bool {
    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();
    (0..width).contains(&x) && (0..height).contains(&y)
}

// Since all the paths have the same length, we only care about the number of paths.
fn get_rating(
    origin: (i64, i64),
    matrix: &Vec<Vec<i64>>,
    memo: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    let origin_x: usize = origin.0.try_into().unwrap();
    let origin_y: usize = origin.1.try_into().unwrap();
    if let Some(&ans) = memo.get(&origin) {
        return ans;
    }
    if matrix[origin_y][origin_x] == 9 {
        return 1;
    }

    let mut path_count = 0;
    for neighbor in get_neighbors(origin) {
        if !lies_inside(neighbor, matrix)
            || matrix[neighbor.1 as usize][neighbor.0 as usize] != matrix[origin_y][origin_x] + 1
        {
            continue;
        }
        path_count += get_rating(neighbor, matrix, memo);
    }

    memo.insert(origin, path_count);
    path_count
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

    let mut memo = HashMap::new();
    let mut trailhead_ratings = 0;
    for trailhead in trailheads {
        trailhead_ratings += get_rating(trailhead, &matrix, &mut memo);
    }

    println!("A total trailhead rating of {}", trailhead_ratings);
}

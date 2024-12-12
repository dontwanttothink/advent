use std::{collections::HashSet, fs};

fn get_neighbors((x, y): (i64, i64)) -> [(i64, i64); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[derive(Debug)]
struct SearchResult {
    perimeter: i64,
    area: i64,
}
fn search_from(
    source: (i64, i64),
    matrix: &[Vec<char>],
    seen: &mut HashSet<(i64, i64)>,
) -> SearchResult {
    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut to_explore = vec![source];
    seen.insert(source);

    let mut perimeter = 0;
    let mut area = 0;

    while !to_explore.is_empty() {
        let mut new_to_explore = vec![];
        for origin in to_explore {
            area += 1;
            let (o_x, o_y) = origin;
            for neighbor in get_neighbors(origin) {
                let (n_x, n_y) = neighbor;
                if !((0..width).contains(&n_x) && (0..height).contains(&n_y)) {
                    perimeter += 1;
                } else if matrix[n_y as usize][n_x as usize] != matrix[o_y as usize][o_x as usize] {
                    perimeter += 1;
                } else if !seen.contains(&neighbor) {
                    new_to_explore.push(neighbor);
                    seen.insert(neighbor);
                }
            }
        }
        to_explore = new_to_explore;
    }

    SearchResult { perimeter, area }
}

fn main() {
    let input = fs::read_to_string("inputs/12.txt").expect("Failed to read file");
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut seen = HashSet::new();

    let mut total_cost = 0;

    for row in 0..height {
        for col in 0..width {
            let coord = (col, row);
            if !seen.contains(&coord) {
                let SearchResult { perimeter, area } = search_from(coord, &matrix, &mut seen);
                total_cost += perimeter * area;
            }
        }
    }

    println!("A total cost of {}", total_cost);
}

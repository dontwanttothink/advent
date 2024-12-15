use std::{collections::HashSet, fs};

fn get_neighbors((x, y): (i64, i64)) -> [(i64, i64); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[derive(Debug)]
struct SearchResult {
    side_count: i64,
    area: i64,
}
fn search_from(
    source: (i64, i64),
    matrix: &[Vec<char>],
    seen: &mut HashSet<(i64, i64)>,
) -> SearchResult {
    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let pot_type = matrix[source.1 as usize][source.0 as usize];

    let mut to_explore = vec![source];
    seen.insert(source);

    let mut area = 0;
    let mut peripheric_pots = HashSet::new();

    while !to_explore.is_empty() {
        let mut new_to_explore = vec![];
        for origin in to_explore {
            area += 1;
            for neighbor in get_neighbors(origin) {
                let (n_x, n_y) = neighbor;
                if !((0..width).contains(&n_x) && (0..height).contains(&n_y))
                    || matrix[n_y as usize][n_x as usize] != pot_type
                {
                    peripheric_pots.insert(origin);
                } else if !seen.contains(&neighbor) {
                    new_to_explore.push(neighbor);
                    seen.insert(neighbor);
                }
            }
        }
        to_explore = new_to_explore;
    }

    let mut vertical_borders = HashSet::new();
    let mut horizontal_borders = HashSet::new();

    let is_foreign = |x: i64, y: i64| {
        !((0..width).contains(&x) && (0..height).contains(&y))
            || matrix[y as usize][x as usize] != pot_type
    };

    for (p_x, p_y) in peripheric_pots {
        if is_foreign(p_x + 1, p_y) {
            vertical_borders.insert((p_x + 1, p_y));
        }
        if is_foreign(p_x - 1, p_y) {
            vertical_borders.insert((p_x, p_y));
        }
        if is_foreign(p_x, p_y + 1) {
            horizontal_borders.insert((p_x, p_y + 1));
        }
        if is_foreign(p_x, p_y - 1) {
            horizontal_borders.insert((p_x, p_y));
        }
    }

    let mut vertical_count = 0;
    {
        let mut vertical_borders = vertical_borders.clone();
        while let Some(&border) = vertical_borders.iter().next() {
            let mut boundary = border;
            while vertical_borders.contains(&(boundary.0, boundary.1 + 1)) {
                boundary = (boundary.0, boundary.1 + 1);
            }

            for i in 0.. {
                let candidate = (boundary.0, boundary.1 - i);

                if vertical_borders.contains(&candidate) {
                    vertical_borders.remove(&candidate);
                } else {
                    break;
                }

                if horizontal_borders.contains(&candidate)
                    || horizontal_borders.contains(&(candidate.0 - 1, candidate.1))
                {
                    break;
                }
            }
            vertical_count += 1;
        }
    }

    let mut horizontal_count = 0;
    while let Some(&border) = horizontal_borders.iter().next() {
        let mut boundary = border;
        while horizontal_borders.contains(&(boundary.0 + 1, boundary.1)) {
            boundary = (boundary.0 + 1, boundary.1);
        }

        for i in 0.. {
            let candidate = (boundary.0 - i, boundary.1);

            if horizontal_borders.contains(&candidate) {
                horizontal_borders.remove(&candidate);
            } else {
                break;
            }

            if vertical_borders.contains(&candidate)
                || vertical_borders.contains(&(candidate.0, candidate.1 - 1))
            {
                break;
            }
        }
        horizontal_count += 1;
    }

    let side_count = vertical_count + horizontal_count;

    // dbg!(vertical_count, horizontal_count, pot_type);
    SearchResult { side_count, area }
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
                let SearchResult {
                    side_count: perimeter,
                    area,
                } = search_from(coord, &matrix, &mut seen);
                total_cost += perimeter * area;
            }
        }
    }

    println!("A revised total cost of {}", total_cost);
}

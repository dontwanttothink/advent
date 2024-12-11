use std::fs;

fn simulate(stones: &[i64]) -> Vec<i64> {
    let mut out = vec![];
    for &stone in stones {
        let stone_digits = stone.checked_ilog10().map(|digs| digs + 1);

        if stone == 0 {
            out.push(1);
        } else if stone_digits.unwrap() % 2 == 0 {
            let stone_digits = stone_digits.unwrap();
            out.push(stone / 10_i64.pow(stone_digits / 2));
            out.push(stone % 10_i64.pow(stone_digits / 2));
        } else {
            out.push(stone * 2024);
        }
    }
    out
}

fn main() {
    let stones_raw = fs::read_to_string("inputs/11.txt").expect("Failed to read file");
    let mut stones: Vec<i64> = stones_raw
        .split_whitespace()
        .map(|n| n.parse::<i64>().expect("Failed to parse a number"))
        .collect();

    for _ in 0..25 {
        stones = simulate(&stones);
    }

    println!("{} stones", stones.len());
}

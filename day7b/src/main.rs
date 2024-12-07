use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::prelude::*;

fn is_possibly_true(result: i64, nums: &[i64]) -> bool {
    if nums.len() == 1 {
        return result == nums[0];
    }

    let last_i = nums.len() - 1;
    let last_denary_digits = nums[last_i].checked_ilog10().unwrap();
    let place_value_factor = 10_i64.pow(last_denary_digits + 1);

    is_possibly_true(result - nums[last_i], &nums[..last_i])
        || (result % nums[last_i] == 0 && is_possibly_true(result / nums[last_i], &nums[..last_i]))
        || ((result - nums[last_i]) % place_value_factor == 0
            && is_possibly_true(
                (result - nums[last_i]) / place_value_factor,
                &nums[..last_i],
            ))
    // the concatenation operator is the same as multiplying the whole thing by
    // 10^(floor(log10(nums[last_i])) + 1) and adding nums[last_i].
}

fn main() {
    let file = File::open("inputs/7.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let calibration_result: i64 = reader
        .lines()
        .par_bridge()
        .map(|line| {
            let line = line.expect("Failed to read line");

            let (result, nums) = {
                let mut parts = line.split(':');
                let part_one = parts.next().expect("Failed to parse a line");
                let part_two = parts.next().expect("Failed to parse a line");

                let result = part_one.parse::<i64>().expect("Failed to parse a result");

                let nums: Vec<i64> = part_two
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().expect("Failed to parse an operand"))
                    .collect();

                (result, nums)
            };

            if is_possibly_true(result, &nums) {
                result
            } else {
                0
            }
        })
        .sum();

    println!(
        "A revised total calibration result of {}",
        calibration_result
    );
}

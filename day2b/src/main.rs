use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::RangeInclusive;

fn is_safe(nums: &[i32]) -> bool {
    let differences_lin: Vec<i32> = nums.windows(2).map(|d| d[1] - d[0]).collect();

    let mut differences = BTreeMap::new();
    for d in &differences_lin {
        *differences.entry(*d).or_insert(0) += 1;
    }

    let mut answer_for_range = |range: RangeInclusive<i32>| {
        if differences.is_empty()
            || range.contains(differences.first_key_value().unwrap().0)
                && range.contains(differences.last_key_value().unwrap().0)
        {
            return true;
        }

        for i in 0..nums.len() {
            // try ignoring 'i':

            // * the delta at position `i - 1`, which might not exist, corresponds
            //   to the delta _to_ this value
            // * the delta at position `i`, which might not exist, corresponds
            //   to the delta _from_ this value

            if i > 0 {
                // `i - 1` exists; remove it
                let obsolete_d = differences_lin[i - 1];
                *differences.get_mut(&obsolete_d).unwrap() -= 1;
                if differences[&obsolete_d] == 0 {
                    differences.remove(&obsolete_d);
                }
            }
            if i < differences_lin.len() {
                // `i` exists; adjust it or remove it if it is now undefined

                // in either case, we remove the old one
                let obsolete_d = differences_lin[i];
                *differences.get_mut(&obsolete_d).unwrap() -= 1;
                if differences[&obsolete_d] == 0 {
                    differences.remove(&obsolete_d);
                }

                if i > 0 {
                    // it should be adjusted to nums[i + 1] - num[i - 1], since
                    // nums[i] is no longer being considered
                    *differences.entry(nums[i + 1] - nums[i - 1]).or_insert(0) += 1;
                }
            }

            let answer_now = range.contains(differences.first_key_value().unwrap().0)
                && range.contains(differences.last_key_value().unwrap().0);

            // now we revert all the changes lol
            //
            if i > 0 {
                let new_d = differences_lin[i - 1];
                *differences.entry(new_d).or_insert(0) += 1;
            }
            if i < differences_lin.len() {
                let new_d = differences_lin[i];
                *differences.entry(new_d).or_insert(0) += 1;

                if i > 0 {
                    let obsolete_d = nums[i + 1] - nums[i - 1];
                    *differences.get_mut(&obsolete_d).unwrap() -= 1;
                    if differences[&obsolete_d] == 0 {
                        differences.remove(&obsolete_d);
                    }
                }
            }

            if answer_now {
                return true;
            }
        }
        false
    };

    answer_for_range(-3..=-1) || answer_for_range(1..=3)
}

fn main() -> io::Result<()> {
    let input = File::open("inputs/2.txt")?;
    let reader = BufReader::new(input);

    let mut count = 0;
    for line in reader.lines().map_while(Result::ok) {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse a number"))
            .collect();

        if is_safe(&nums) {
            count += 1;
        }
    }

    println!("{} tolerable reports", count);
    Ok(())
}

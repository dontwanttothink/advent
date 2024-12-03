use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let input = File::open("inputs/2.txt")?;
    let reader = BufReader::new(input);

    let mut count = 0;
    for line in reader.lines().map_while(Result::ok) {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Failed to parse a number"))
            .collect();

        let differences = nums.windows(2).map(|n| n[1] - n[0]);

        if differences.clone().all(|d| (1..=3).contains(&d))
            || differences.clone().all(|d| (-3..=-1).contains(&d))
        {
            count += 1;
        }
    }

    println!("{}", count);
    Ok(())
}

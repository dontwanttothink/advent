use std::{
    fs::File,
    io::{self, Read},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let mut file = File::open("inputs/3.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let enabled_regex = Regex::new(r"(^|do\(\))[\s\S]*?(don't\(\))").unwrap();
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for enabled_segment in enabled_regex.find_iter(&input) {
        for captures in mul_regex.captures_iter(enabled_segment.as_str()) {
            let first: i64 = captures[1].parse().unwrap();
            let second: i64 = captures[2].parse().unwrap();

            sum += first * second;
        }
    }

    println!("{}", sum);

    Ok(())
}

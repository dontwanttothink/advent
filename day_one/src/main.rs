use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;

fn main() -> io::Result<()> {
    let mut list_one = vec![];
    let mut list_two = vec![];

    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    for (i, line) in reader.lines().map_while(Result::ok).enumerate() {
        let mut words = line.split_whitespace();
        let one = words
            .next()
            .unwrap_or_else(|| panic!("Missing first item in line {}", i))
            .to_owned();
        let two = words
            .next()
            .unwrap_or_else(|| panic!("Missing second item in line {}", i))
            .to_owned();

        let one = one
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse the first item in line {}", i));
        let two = two
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse the second item in line {}", i));

        list_one.push(one);
        list_two.push(two);
    }

    list_one.sort_unstable();
    list_two.sort_unstable();

    let mut difference = 0;
    for (first, second) in zip(list_one, list_two) {
        difference += (first - second).abs();
    }

    println!("{}", difference);

    Ok(())
}

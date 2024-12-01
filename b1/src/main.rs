use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut list_one = vec![];
    let mut list_two = HashMap::new();

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
        *list_two.entry(two).or_insert(0) += 1;
    }

    let mut similarity = 0;

    for i in list_one {
        similarity += i * list_two.get(&i).unwrap_or(&0);
    }

    println!("{}", similarity);

    Ok(())
}

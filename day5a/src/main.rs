use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct RequirementParsingError;
struct Requirement(i64, i64);
impl TryFrom<&str> for Requirement {
    type Error = RequirementParsingError;

    fn try_from(req: &str) -> Result<Self, Self::Error> {
        let mut page_indexes = req.split('|').map(|n| n.parse::<i64>());

        let first = page_indexes.next();
        let second = page_indexes.next();
        if first.is_none() || second.is_none() {
            return Err(RequirementParsingError);
        }

        let first = first.unwrap();
        let second = second.unwrap();
        if first.is_err() || second.is_err() {
            return Err(RequirementParsingError);
        }

        let first = first.unwrap();
        let second = second.unwrap();
        Ok(Requirement(first, second))
    }
}

enum Section {
    Rules,
    Instances,
}

fn main() {
    let file = File::open("inputs/5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut requirements: Vec<Requirement> = vec![];
    let mut sum_of_middle = 0;

    let mut current_section = Section::Rules;
    for line in reader.lines().map_while(Result::ok) {
        if line.is_empty() {
            current_section = Section::Instances;
            continue;
        }

        match current_section {
            Section::Rules => {
                requirements.push(line.as_str().try_into().unwrap());
            }
            Section::Instances => {
                let page_nums: Vec<i64> =
                    line.split(',').map(|n| n.parse::<i64>().unwrap()).collect();

                let mut page_locations = HashMap::new();
                for (i, page) in page_nums.iter().enumerate() {
                    page_locations.insert(page, i);
                }

                let mut is_valid = true;
                for Requirement(first, second) in &requirements {
                    if !(page_locations.contains_key(first) && page_locations.contains_key(second))
                    {
                        continue;
                    }
                    if page_locations[first] > page_locations[second] {
                        is_valid = false;
                    }
                }
                if is_valid {
                    sum_of_middle += page_nums[page_nums.len() / 2];
                }
            }
        }
    }

    println!("{}", sum_of_middle);
}

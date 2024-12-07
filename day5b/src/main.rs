use std::{
    collections::{HashMap, HashSet},
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

#[derive(Default)]
struct PageVertex {
    from: HashSet<i64>,
    to: HashSet<i64>,
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
                let mut relevant_requirements = vec![];
                for requirement in &requirements {
                    let Requirement(first, second) = requirement;

                    if !(page_locations.contains_key(first) && page_locations.contains_key(second))
                    {
                        continue;
                    }

                    relevant_requirements.push(requirement);

                    if page_locations[first] > page_locations[second] {
                        is_valid = false;
                    }
                }
                if !is_valid {
                    // produce a valid ordering. they are guaranteed to be
                    // unique apparently. maybe that tells us something about
                    // an easier way of doing this. but i don't see it, sadly.

                    // we are going to represent these as a directed graph
                    // and sorting it topologically

                    let mut vertices = HashMap::new();
                    for &page_num in &page_nums {
                        vertices.insert(page_num, PageVertex::default());
                    }
                    for &Requirement(first, second) in relevant_requirements {
                        vertices.get_mut(&first).unwrap().to.insert(second);
                        vertices.get_mut(&second).unwrap().from.insert(first);
                    }

                    let mut start_vertices_ids = vec![];
                    for (id, vertex) in &vertices {
                        if vertex.from.is_empty() {
                            start_vertices_ids.push(*id);
                        }
                    }
                    let mut topological_sort = vec![];

                    while let Some(s_id) = start_vertices_ids.pop() {
                        topological_sort.push(s_id);

                        let s_vertex = vertices.remove(&s_id).unwrap();
                        for to_id in s_vertex.to {
                            vertices.get_mut(&to_id).unwrap().from.remove(&s_id);
                            if vertices[&to_id].from.is_empty() {
                                start_vertices_ids.push(to_id);
                            }
                        }
                    }

                    sum_of_middle += topological_sort[topological_sort.len() / 2];
                }
            }
        }
    }

    println!("Rearranged, a middle-page-sum of {}", sum_of_middle);
}

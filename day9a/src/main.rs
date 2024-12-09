use std::{fs, iter::zip};

#[derive(Clone, Copy)]
enum DiskSpace {
    Occupied,
    Free,
}
use DiskSpace::*;

fn main() {
    let disk_map = {
        let mut raw = fs::read_to_string("inputs/9.txt").unwrap();
        raw.pop();
        raw
    };
    let mut disk: Vec<Option<i64>> = vec![];

    let mut current_id = 0;
    for (kind, length) in zip(
        [Occupied, Free].into_iter().cycle(),
        disk_map.chars().map(|d| d.to_digit(10).unwrap()),
    ) {
        match kind {
            Occupied => {
                for _ in 0..length {
                    disk.push(Some(current_id));
                }
                current_id += 1;
            }
            Free => {
                for _ in 0..length {
                    disk.push(None);
                }
            }
        }
    }

    let mut current_empty = 0;
    while disk[current_empty].is_some() {
        current_empty += 1;
    }
    let mut current_nonempty = disk.len() - 1;
    while disk[current_nonempty].is_none() {
        current_nonempty -= 1;
    }

    while current_empty < current_nonempty {
        disk[current_empty] = disk[current_nonempty];
        disk[current_nonempty] = None;

        while disk[current_empty].is_some() {
            current_empty += 1;
        }
        while disk[current_nonempty].is_none() {
            current_nonempty -= 1;
        }
    }

    let mut checksum = 0;
    for (i, item) in disk.iter().enumerate() {
        let i: i64 = i.try_into().unwrap();

        if item.is_none() {
            break;
        }
        let item = item.unwrap();
        checksum += i * item;
    }

    println!("A checksum of {}", checksum);
}

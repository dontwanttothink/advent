use indicatif::ProgressIterator;
use std::{collections::BTreeMap, fs, iter::zip};

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

    let mut free_lengths = BTreeMap::new();
    let mut occupied_lengths = BTreeMap::new();
    let mut current_id = 0;
    for (kind, length) in zip(
        [Occupied, Free].into_iter().cycle(),
        disk_map.chars().map(|d| d.to_digit(10).unwrap()),
    ) {
        match kind {
            Occupied => {
                occupied_lengths.insert(disk.len(), length as usize);
                for _ in 0..length {
                    disk.push(Some(current_id));
                }
                current_id += 1;
            }
            Free => {
                free_lengths.insert(disk.len(), length as usize);
                for _ in 0..length {
                    disk.push(None);
                }
            }
        }
    }

    for (&i, &o_len) in occupied_lengths.iter().rev().progress() {
        let mut maybe_spot = None;
        for (&j, &f_len) in &free_lengths {
            if j > i {
                break;
            }
            if f_len >= o_len {
                maybe_spot = Some((j, f_len));
                break;
            }
        }

        if let Some((spot, s_length)) = maybe_spot {
            let mut k = spot;
            let mut l = i;

            while l < i + o_len {
                disk[k] = disk[l];
                disk[l] = None;
                k += 1;
                l += 1;
            }

            free_lengths.remove(&spot);
            free_lengths.insert(k, s_length - o_len);
        }
    }

    let mut checksum = 0;
    for (i, item) in disk.iter().enumerate() {
        if item.is_none() {
            continue;
        }

        let item = item.unwrap();
        let i: i64 = i.try_into().unwrap();
        checksum += i * item;
    }

    println!("A revised checksum of {}", checksum);
}

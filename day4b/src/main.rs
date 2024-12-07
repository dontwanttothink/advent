use std::{
    fs::File,
    io::{self, Read},
};

const XMAS_PATTERNS: [[[std::option::Option<char>; 3]; 3]; 4] = [
    [
        [Some('M'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('S')],
    ],
    [
        [Some('S'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('M')],
    ],
    [
        [Some('S'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('M')],
    ],
    [
        [Some('M'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('S')],
    ],
];

fn main() -> io::Result<()> {
    let mut file = File::open("inputs/4.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height = matrix.len();
    let width = matrix[0].len();

    let mut count = 0;
    for i in 0..=width - 3 {
        for j in 0..=height - 3 {
            for mas_pattern in XMAS_PATTERNS {
                let mut is_match = true;
                for k in 0..3 {
                    for l in 0..3 {
                        if let Some(ch) = mas_pattern[l][k] {
                            is_match = is_match && ch == matrix[j + l][i + k];
                        }
                    }
                }
                if is_match {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}

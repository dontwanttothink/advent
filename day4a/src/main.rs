use std::{
    fs::File,
    io::{self, Read},
    ops::RangeInclusive,
};

const FORW: &str = "XMAS";
const BACK: &str = "SAMX";

struct Domain(RangeInclusive<i32>, RangeInclusive<i32>);
struct Line {
    /// A function that takes two values (say, `c` and `x`) and returns a
    /// coordinate pair (say, `(x, 3x + c)`). Fixing the first parameter
    /// (say, `c`) to a constant gives a linear function.
    given: Box<dyn Fn(i32, i32) -> (i32, i32)>,

    /// A function that takes two values, `a` and `b`, and returns the set of
    /// input pairs to `.given()` such that every point in the rectangle with
    /// vertices at `(0, 0)` and `(a, b)` is yielded exactly once.
    domain: Box<dyn Fn(i32, i32) -> Domain>,
}

impl Line {
    fn get_domain(&self, a: i32, b: i32) -> Domain {
        (self.domain)(a, b)
    }

    fn given(&self, c: i32, z: i32) -> (i32, i32) {
        (self.given)(c, z)
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("inputs/4.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let matrix: Vec<&str> = input.lines().collect();

    let height = {
        let h: i32 = matrix.len().try_into().unwrap();
        h - 1
    };
    let width = {
        let w: i32 = matrix[0].len().try_into().unwrap();
        w - 1
    };

    let horizontal = Line {
        given: Box::new(|c, x| (x, c)),
        domain: Box::new(|a, b| Domain(0..=b, 0..=a)),
    };
    let vertical = Line {
        given: Box::new(|c, y| (c, y)),
        domain: Box::new(|a, b| Domain(0..=a, 0..=b)),
    };
    let downward = Line {
        given: Box::new(|c, x| (x, -x + c)),
        domain: Box::new(|a, b| Domain(0..=a + b, 0..=a)),
    };
    let upward = Line {
        given: Box::new(|c, x| (x, x + c)),
        domain: Box::new(|a, b| Domain(-a..=b, 0..=a)),
    };

    let lines = [horizontal, vertical, downward, upward];

    let mut count = 0;
    for line in lines {
        let Domain(d_first, d_second) = line.get_domain(width, height);

        for c in d_first.clone() {
            let mut contents = String::new();

            for z in d_second.clone() {
                let (x, y) = line.given(c, z);

                if (0..=width).contains(&x) && (0..=height).contains(&y) {
                    let x = x as usize;
                    let y = y as usize;

                    contents.push(matrix[y].as_bytes()[x] as char)
                }
            }

            let match_count = contents.matches(FORW).count() + contents.matches(BACK).count();
            count += match_count;
        }
    }

    println!("{} instances of XMAS!", count);

    Ok(())
}

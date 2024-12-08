use std::{
    collections::{HashMap, HashSet},
    fs, ops,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point(i64, i64);
impl Point {
    fn gcd(&self) -> i64 {
        let mut a = self.0.abs();
        let mut b = self.1.abs();
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
}
impl ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}
impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1)
    }
}
impl ops::DivAssign<i64> for Point {
    fn div_assign(&mut self, other: i64) {
        *self = Self(self.0 / other, self.1 / other);
    }
}

fn main() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|n| n.chars().collect()).collect();

    let height: i64 = matrix.len().try_into().unwrap();
    let width: i64 = matrix[0].len().try_into().unwrap();

    let mut radio_positions: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            let x: i64 = x.try_into().unwrap();
            let y: i64 = y.try_into().unwrap();

            if ch != '.' {
                radio_positions.entry(ch).or_default().push(Point(x, y));
            }
        }
    }

    let mut antinode_positions = HashSet::new();

    for (_ch, positions) in radio_positions {
        for &first_pos in &positions {
            for &second_pos in &positions {
                if first_pos == second_pos {
                    continue;
                }

                let mut delta = second_pos - first_pos;
                // needed for correctness, though makes no difference in my
                // data
                delta /= delta.gcd();

                let mut antinode_pos = first_pos;
                while (0..width).contains(&antinode_pos.0) && (0..height).contains(&antinode_pos.1)
                {
                    antinode_positions.insert(antinode_pos);
                    antinode_pos += delta;
                }
            }
        }
    }

    println!(
        "{} revised unique antinode positions",
        antinode_positions.len()
    );
}

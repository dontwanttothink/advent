// Pushing A is more expensive than pushing B.

use rational::Rational;
use std::fs;

fn extract_numbers(s: &str) -> Vec<i64> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[derive(Debug)]
struct Machine {
    a_button: (i64, i64),
    b_button: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn min_tokens_needed(&self) -> Option<i64> {
        println!("");

        let mut matrix = [
            [
                Rational::from(self.a_button.0),
                Rational::from(self.b_button.0),
                Rational::from(self.prize.0),
            ], // X
            [
                Rational::from(self.a_button.1),
                Rational::from(self.b_button.1),
                Rational::from(self.prize.1),
            ], // Y
        ];

        let mut to_fill = 0;
        for unknown in 0..2 {
            let mut pivot = None;
            for row in to_fill..2 {
                if matrix[row][unknown] != 0 {
                    matrix.swap(row, to_fill);
                    pivot = Some(to_fill);
                    to_fill += 1;
                    break;
                }
            }

            if let Some(pivot) = pivot {
                let pivot_value = matrix[pivot][unknown];
                for col in 0..3 {
                    matrix[pivot][col] /= pivot_value;
                }

                for row in (pivot + 1)..2 {
                    let factor = matrix[row][unknown];
                    for col in 0..3 {
                        matrix[row][col] -= matrix[pivot][col] * factor;
                    }
                }
            }
        }

        for row in (0..2).rev() {
            let mut leading_one = None;
            for col in 0..2 {
                if matrix[row][col] == 1 {
                    leading_one = Some(col);
                    break;
                }
            }

            if let Some(leading_one) = leading_one {
                for r in 0..row {
                    let factor = matrix[r][leading_one];
                    for c in 0..3 {
                        matrix[r][c] -= matrix[row][c] * factor;
                    }
                }
            }
        }

        if matrix[1][0] == 0 && matrix[1][1] == 0 && matrix[1][2] != 0 {
            // inconsistent system
            return None;
        }
        if matrix[0][1] != 0 {
            // dependent system

            let coefficient = matrix[0][1];
            let value = matrix[0][2];

            let b = coefficient.denominator() * (value / coefficient.numerator());
            let a = value - coefficient * b;

            assert_eq!(b.denominator(), 1);
            assert_eq!(a.denominator(), 1);

            let a = a.numerator();
            let b = b.numerator();

            return Some((3 * a + b).try_into().unwrap());
        }
        // independent system

        let a = matrix[0][2];
        let b = matrix[1][2];

        if a.denominator() == 1 && b.denominator() == 1 && a >= 0 && b >= 0 {
            let a = a.numerator();
            let b = b.numerator();
            Some((3 * a + b).try_into().unwrap())
        } else {
            // only solutions are not positive integers
            None
        }
    }
}

fn main() {
    let file = fs::read_to_string("inputs/13.txt").expect("Failed to read file");
    let lines: Vec<&str> = file.lines().collect();

    let mut machines = vec![];

    for description in lines.chunks(4) {
        let a = extract_numbers(description[0]);
        let b = extract_numbers(description[1]);
        let p = extract_numbers(description[2]);

        machines.push(Machine {
            a_button: (a[0], a[1]),
            b_button: (b[0], b[1]),
            prize: (p[0], p[1]),
        });
    }

    let mut total_tokens_needed = 0;
    for machine in machines {
        if let Some(tokens) = machine.min_tokens_needed() {
            total_tokens_needed += tokens;
        }
    }
    println!("{} tokens are needed", total_tokens_needed);
}

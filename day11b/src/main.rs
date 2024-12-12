use std::{collections::HashMap, fs};

fn count_stones(stone: i64, after: usize, memo: &mut HashMap<(i64, usize), usize>) -> usize {
    if let Some(&ans) = memo.get(&(stone, after)) {
        return ans;
    }
    if after == 0 {
        return 1;
    }

    let stone_digits = stone.checked_ilog10().map(|digs| digs + 1);
    let count = if stone == 0 {
        count_stones(1, after - 1, memo)
    } else if stone_digits.unwrap() % 2 == 0 {
        let stone_digits = stone_digits.unwrap();
        count_stones(stone / 10_i64.pow(stone_digits / 2), after - 1, memo)
            + count_stones(stone % 10_i64.pow(stone_digits / 2), after - 1, memo)
    } else {
        count_stones(stone * 2024, after - 1, memo)
    };

    memo.insert((stone, after), count);
    count
}

fn main() {
    let stones_raw = fs::read_to_string("inputs/11.txt").expect("Failed to read file");
    let stones: Vec<i64> = stones_raw
        .split_whitespace()
        .map(|n| n.parse::<i64>().expect("Failed to parse a number"))
        .collect();

    let mut memo = HashMap::new();
    let mut final_count = 0;
    for stone in stones {
        final_count += count_stones(stone, 75, &mut memo);
    }

    println!("{} stones", final_count);
}

// The following isn't that relevant to my solution, but
// I'll offer it here nonetheless.
//
// The number of denary digits of a number is given by:
// floor(log10(n)) + 1
//
// The number of denary digits of a number after multiplying
// it by 2024 is given by: floor(log10(n) + log10(2024)) + 1
//
// Note that log10(2024) < 3.31.
//
// To calculate floor(log10(n) + log10(2024)), we can add
// the integer part and the decimal part separately. If the
// decimal part is less than one, it has no effect, and the
// answer is the sum of the integer parts. If it is equal to
// one one or more, the answer is the sum of the integer
// parts plus one.
//
// (If four digits are added to a numeral with an odd number
// of digits, it remains with an odd number of digits. If
// three are added, the number of digits becomes even.)
//
// This means that a number that has just been multiplied by
// 2024 and that remains with an odd number of digits must
// have a logarithmic fractional part smaller than 3.31.
// This is because its fractional part before multiplying
// was less than 1 —well, it's fractional after all!
//
// This is how we know that you have to multiply a number
// with an odd number of digits at most twice by 2024 to get
// a number with an even number of digits.
//
// If this explanation isn't clear, I am sorry. I could try
// clarifying it later, though frankly I would be very
// surprised if I actually did.
//
// Other intuitive explanations might exist. I don't know.
// For example, log10(2024^2) < 6.62.

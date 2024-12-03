// I wrote this as an exercise. I'm not sure whether it's correct.
// I wish I knew a little about writing parsers.

use std::{
    fs::File,
    io::{self, Read},
    mem,
};

const PARTS: [&[char]; 3] = [&['m', 'u', 'l', '('], &[','], &[')']];

struct DraftExpression {
    text: String,
    part: usize,
    index: usize,
}

#[derive(PartialEq, Eq)]
struct Expecting {
    digit: bool,
    char: Option<char>,
}

impl Expecting {
    fn nothing() -> Self {
        Expecting {
            digit: false,
            char: None,
        }
    }

    fn digit() -> Self {
        Expecting {
            digit: true,
            char: None,
        }
    }

    fn char(ch: char) -> Self {
        Expecting {
            digit: false,
            char: Some(ch),
        }
    }
}

impl DraftExpression {
    fn new() -> Self {
        Self {
            text: String::default(),
            part: 0,
            index: 0,
        }
    }

    fn get_text(&mut self) -> String {
        let text = mem::take(&mut self.text);
        self.clear();
        text
    }

    fn expecting(&self) -> Expecting {
        let current_part = PARTS[self.part];
        if (0..current_part.len()).contains(&self.index) {
            return Expecting::char(current_part[self.index]);
        }

        if self.part == PARTS.len() - 1 {
            return Expecting::nothing();
        }

        if self.index == current_part.len() {
            return Expecting::digit();
        }

        Expecting {
            digit: true,
            char: Some(PARTS[self.part + 1][0]),
        }
    }

    fn push(&mut self, ch: char) {
        let expectation = self.expecting();

        let is_right_digit = expectation.digit && ch.is_ascii_digit();
        let is_right_char = expectation.char.is_some_and(|c| c == ch);

        if is_right_digit || is_right_char {
            self.text.push(ch);
            self.index += 1;

            if is_right_char && expectation.digit {
                self.part += 1;
                self.index = 1;
            }
        } else if PARTS[0][0] == ch {
            self.clear();
            self.text.push(PARTS[0][0]);
            self.index += 1;
        } else {
            self.clear();
        }
    }

    fn is_ready(&self) -> bool {
        self.expecting() == Expecting::nothing()
    }

    fn clear(&mut self) {
        self.text.clear();
        self.part = 0;
        self.index = 0;
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("inputs/3.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut expressions = vec![];
    let mut current_expression = DraftExpression::new();

    for char in input.chars() {
        current_expression.push(char);
        if current_expression.is_ready() {
            expressions.push(current_expression.get_text())
        }
    }

    Ok(())
}

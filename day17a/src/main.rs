use std::fs;

fn extract_numbers(s: &str) -> Vec<i64> {
    s.split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[derive(Debug)]
struct Memory {
    register_a: i64,
    register_b: i64,
    register_c: i64,
}

#[derive(Debug, Clone, Copy)]
struct InvalidComboOperand;
fn evaluate_combo(operand: i64, memory: &Memory) -> Result<i64, InvalidComboOperand> {
    if (0..=3).contains(&operand) {
        Ok(operand)
    } else if operand == 4 {
        Ok(memory.register_a)
    } else if operand == 5 {
        Ok(memory.register_b)
    } else if operand == 6 {
        Ok(memory.register_c)
    } else {
        Err(InvalidComboOperand)
    }
}

fn main() {
    let input = fs::read_to_string("inputs/17.txt").expect("Failed to read file");

    let (mut memory, program) = {
        let mut lines = input.lines().filter(|l| !l.is_empty()).map(extract_numbers);

        let register_a = lines.next().expect("Missing register A information")[0];
        let register_b = lines.next().expect("Missing register B information")[0];
        let register_c = lines.next().expect("Missing register C information")[0];
        let program = lines.next().expect("Missing program information");

        (
            Memory {
                register_a,
                register_b,
                register_c,
            },
            program,
        )
    };

    let mut has_printed = false;
    let mut instruction_pointer = 0;
    while instruction_pointer < program.len() {
        let instruction = program[instruction_pointer];
        let &operand = program
            .get(instruction_pointer + 1)
            .expect("Failed to read operand");

        match instruction {
            0 => {
                let a = memory.register_a;

                let exp = evaluate_combo(operand, &memory)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let b = 2_i64.pow(exp);

                memory.register_a = a / b;
            }
            1 => {
                let a = memory.register_b;
                let b = operand;

                memory.register_b = a ^ b;
            }
            2 => {
                let a = evaluate_combo(operand, &memory).unwrap();
                memory.register_b = a.rem_euclid(8);
            }
            3 => {
                if memory.register_a != 0 {
                    let a = operand.try_into().unwrap();
                    instruction_pointer = a;
                    continue;
                }
            }
            4 => {
                let a = memory.register_b;
                let b = memory.register_c;
                memory.register_b = a ^ b;
            }
            5 => {
                let a = evaluate_combo(operand, &memory).unwrap().rem_euclid(8);

                if has_printed {
                    print!(",{}", a);
                } else {
                    print!("{}", a);
                    has_printed = true;
                }
            }
            6 => {
                let a = memory.register_a;

                let exp = evaluate_combo(operand, &memory)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let b = 2_i64.pow(exp);

                memory.register_b = a / b;
            }
            7 => {
                let a = memory.register_a;

                let exp = evaluate_combo(operand, &memory)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let b = 2_i64.pow(exp);

                memory.register_c = a / b;
            }
            _ => {
                panic!("Invalid opcode! {}", instruction);
            }
        }

        instruction_pointer += 2;
    }
    println!();
}

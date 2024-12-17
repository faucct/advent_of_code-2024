use std::{u128, usize};

fn sum(reader: impl std::io::BufRead) -> u128 {
    let mut lines = reader.lines().map(|line| line.unwrap());
    regex::Regex::new("Register A: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    regex::Regex::new("Register B: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    regex::Regex::new("Register C: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    lines.next();
    let program = String::from(lines.next().unwrap().strip_prefix("Program: ").unwrap());
    let instructions = program
        .split(",")
        .map(|program| program.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    fn rec(instructions: &[u8], outputs: &[u8], answer: u128) -> u128 {
        if let Some(output) = outputs.split_last() {
            'tail: for tail in 0..1 << 3 {
                let answer = (answer << 3) | tail;
                let mut a = answer;
                let mut b = 0;
                let mut c = 0;
                let mut instruction_pointer = 0;
                while instruction_pointer < instructions.len() {
                    let opcode = instructions[instruction_pointer];
                    instruction_pointer += 1;
                    let literal = instructions[instruction_pointer];
                    let combo = match literal {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        4 => a,
                        5 => b,
                        6 => c,
                        _ => panic!(),
                    };
                    instruction_pointer += 1;
                    match opcode {
                        0 => {
                            a /= 2u128.pow(combo as u32);
                        }
                        1 => {
                            b ^= literal as u128;
                        }
                        2 => {
                            b = combo % 8;
                        }
                        3 => {
                            if a != 0 {
                                instruction_pointer = literal as usize;
                            }
                        }
                        4 => {
                            b ^= c;
                        }
                        5 => {
                            if (combo % 8) as u8 == *output.0 {
                                let rec = rec(instructions, output.1, answer);
                                if rec != u128::MAX {
                                    return rec;
                                }
                            } else {
                                continue 'tail;
                            }
                        }
                        6 => {
                            b = a / 2u128.pow(combo as u32);
                        }
                        7 => {
                            c = a / 2u128.pow(combo as u32);
                        }
                        _ => panic!(),
                    }
                }
            }
            u128::MAX
        } else {
            answer
        }
    }
    rec(&instructions, &instructions, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            117440,
            sum("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

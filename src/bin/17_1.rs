use std::usize;

fn sum(reader: impl std::io::BufRead) -> String {
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut a = regex::Regex::new("Register A: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    let mut b = regex::Regex::new("Register B: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    let mut c = regex::Regex::new("Register C: (\\d+)")
        .unwrap()
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u128>()
        .unwrap();
    lines.next();
    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|program| program.parse::<u128>().unwrap())
        .collect::<Vec<_>>();
    let mut instruction_pointer = 0;
    let mut output = Vec::new();
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        instruction_pointer += 1;
        let literal = program[instruction_pointer];
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
                b ^= literal;
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
                output.push(format!("{}", combo % 8));
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
    output.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            "4,6,3,5,6,3,5,2,1,0",
            sum("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
                .as_bytes())
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            "4,2,5,6,7,7,7,7,3,1,0",
            sum("Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

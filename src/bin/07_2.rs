fn sum(reader: impl std::io::BufRead) -> i128 {
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                return 0;
            }
            let (test_value, numbers) = line.split_once(": ").unwrap();
            let test_value = test_value.parse::<i128>().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|number| number.parse::<i128>().unwrap())
                .collect::<Vec<_>>();
            fn rec(test_value: i128, mul: i128, numbers: &[i128]) -> bool {
                if test_value < mul {
                    return false;
                }
                if let Some((&head, tail)) = numbers.split_first() {
                    rec(test_value, mul.saturating_mul(10i128.saturating_pow(head.ilog10() + 1)).saturating_add(head), tail)
                        || rec(test_value, head.saturating_add(mul), tail)
                        || rec(test_value, mul.saturating_mul(head), tail)
                } else {
                    test_value == mul
                }
            }

            if rec(test_value as i128, numbers[0], &numbers[1..]) {
                test_value
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            11387,
            sum("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

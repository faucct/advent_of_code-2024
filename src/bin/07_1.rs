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
            fn rec(test_value: i128, first: i128, numbers: &[i128]) -> bool {
                if test_value < 0 {
                    return false;
                }
                if let Some((&last, rest)) = numbers.split_last() {
                    if test_value % last == 0 && rec(test_value / last, first, rest) {
                        return true;
                    }
                    rec(test_value.saturating_sub(last), first, rest)
                } else {
                    test_value == first
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
            3749,
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

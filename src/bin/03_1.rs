use regex::Regex;

fn sum(mut reader: impl std::io::BufRead) -> i32 {
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    Regex::new("mul\\((\\d+),(\\d+)\\)")
        .unwrap()
        .captures_iter(&string)
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            161,
            sum(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

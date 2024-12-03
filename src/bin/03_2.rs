use regex::Regex;

fn sum(mut reader: impl std::io::BufRead) -> i32 {
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    let mut dont = false;
    Regex::new("mul\\((\\d+),(\\d+)\\)|don't\\(\\)|do\\(\\)")
        .unwrap()
        .captures_iter(&string)
        .map(|capture| {
            match capture.get(0).unwrap().as_str() {
                "do()" => dont = false,
                "don't()" => dont = true,
                _ => {
                    if !dont {
                        return capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                            * capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    }
                }
            };
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            48,
            sum(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

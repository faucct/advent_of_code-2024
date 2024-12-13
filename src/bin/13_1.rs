fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut sum = 0;
    let mut lines = reader.lines();
    let a_regex = regex::Regex::new("Button A: X\\+(\\d+), Y\\+(\\d+)").unwrap();
    let b_regex = regex::Regex::new("Button B: X\\+(\\d+), Y\\+(\\d+)").unwrap();
    let prize_regex = regex::Regex::new("Prize: X=(\\d+), Y=(\\d+)").unwrap();
    while let Some(a) = lines.next() {
        let a = a.unwrap();
        if a.is_empty() {
            break;
        }
        let a = a_regex.captures(&a).unwrap();
        let a = (
            a.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            a.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        );
        let b = lines.next().unwrap().unwrap();
        let b = b_regex.captures(&b).unwrap();
        let b = (
            b.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            b.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        );
        let prize = lines.next().unwrap().unwrap();
        let prize = prize_regex.captures(&prize).unwrap();
        let prize = (
            prize.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            prize.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        );
        lines.next();
        let mut b_times = (prize.0 / b.0).max(prize.1 / b.1);
        let mut remaining = (prize.0 - b_times * b.0, prize.1 - b_times * b.1);
        let tokens = (0..=(prize.0 / a.0).min(prize.1 / a.1))
            .map(|a_times| {
                while remaining.0 < 0 || remaining.1 < 0 {
                    b_times -= 1;
                    remaining.0 += b.0;
                    remaining.1 += b.1;
                }
                let tokens = if remaining == (0, 0) {
                    a_times * 3 + b_times
                } else {
                    i32::MAX
                };
                remaining.0 -= a.0;
                remaining.1 -= a.1;
                tokens
            })
            .min()
            .unwrap();
        if tokens != i32::MAX {
            sum += tokens;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            480,
            sum("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

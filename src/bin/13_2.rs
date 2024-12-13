fn sum(reader: impl std::io::BufRead) -> i64 {
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
            a.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            a.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let b = lines.next().unwrap().unwrap();
        let b = b_regex.captures(&b).unwrap();
        let b = (
            b.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            b.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        );
        let prize = lines.next().unwrap().unwrap();
        let prize = prize_regex.captures(&prize).unwrap();
        let prize = (
            prize.get(1).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
            prize.get(2).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000,
        );
        lines.next();
        // i * a.0 + j * b.0 == prize.0
        // i * a.1 + j * b.1 == prize.1
        // i = (prize.0 - j * b.0) / a.0
        // (prize.0 - j * b.0) * a.1 + j * b.1 * a.0 == prize.1 * a.0
        // j == (prize.1 * a.0 - prize.0 * a.1) / (b.1 * a.0 - b.0 * a.1)
        let div = b.1 * a.0 - b.0 * a.1;
        if div == 0 {
            if a.0 < 3 * b.0 {
                if prize.0 % b.0 == 0 {
                    let times = prize.0 / b.0;
                    if times * b.1 == prize.1 {
                        sum += times;
                    }
                }
            } else {
                let a_times = prize.0 / a.0;
                let b_times = (prize.0 - a_times * a.0) / b.0;
                if a_times * a.0 + b_times * b.0 == prize.0
                    && a_times * a.1 + b_times * b.1 == prize.1
                {
                    sum += 3 * a_times + b_times;
                }
            }
        } else {
            let up = prize.1 * a.0 - prize.0 * a.1;
            if up % div == 0 {
                let j = up / div;
                let up = prize.0 - j * b.0;
                if up % a.0 == 0 {
                    let i = up / a.0;
                    sum += i * 3 + j;
                }
            }
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
            875318608908,
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

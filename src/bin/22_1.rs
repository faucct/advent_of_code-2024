use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    reader
        .lines()
        .map(|buyer| {
            let buyer = buyer.unwrap();
            if buyer.is_empty() {
                return 0;
            }
            let mut buyer = buyer.parse::<usize>().unwrap();
            for _ in 0..2000 {
                buyer ^= buyer * 64;
                buyer %= 16777216;
                buyer ^= buyer / 32;
                buyer %= 16777216;
                buyer ^= buyer * 2048;
                buyer %= 16777216;
            }
            buyer
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            37327623,
            sum("1
10
100
2024"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

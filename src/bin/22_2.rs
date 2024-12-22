fn sum(reader: impl std::io::BufRead) -> i64 {
    let mut sequences = std::collections::HashMap::new();
    let mut buyers = 0;
    for buyer in reader.lines() {
        let buyer = buyer.unwrap();
        if buyer.is_empty() {
            return 0;
        }
        buyers += 1;
        let buyer = buyer.parse::<i64>().unwrap();
        let mut sequence = [i8::MAX; 4];
        let mut prev = buyer;
        for _ in 0..2000 {
            let mut next = prev;
            next ^= next * 64;
            next %= 16777216;
            next ^= next / 32;
            next %= 16777216;
            next ^= next * 2048;
            next %= 16777216;
            sequence = [
                sequence[1],
                sequence[2],
                sequence[3],
                ((next % 10) - (prev % 10)) as i8,
            ];
            if sequence[0] != i8::MAX {
                let prices = sequences.entry(sequence).or_insert(Vec::new());
                if prices.len() < buyers {
                    while prices.len() + 1 < buyers {
                        prices.push(0);
                    }
                    prices.push(next % 10);
                }
            }
            prev = next;
        }
    }
    sequences
        .into_values()
        .map(|prices| prices.into_iter().sum())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            23,
            sum("1
2
3
2024"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

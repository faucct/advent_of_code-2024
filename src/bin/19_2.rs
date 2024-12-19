use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut lines = reader.lines();
    let towels = lines.next().unwrap().unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    lines.next().unwrap().unwrap();
    let mut dp = Vec::new();
    lines
        .map(|line| {
            let line = line.unwrap();
            dp.clear();
            for i in 0..line.len() {
                let prefix = &line[..i + 1];
                let mut sum = 0;
                for towel in &towels {
                    if prefix.ends_with(towel) {
                        sum += if prefix.len() == towel.len() {
                            1
                        } else {
                            dp[prefix.len() - towel.len() - 1]
                        };
                    }
                }
                dp.push(sum);
            }
            *dp.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            16,
            sum("r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

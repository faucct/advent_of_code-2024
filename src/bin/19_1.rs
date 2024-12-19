use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut lines = reader.lines();
    let towels = lines.next().unwrap().unwrap();
    let towels = towels.split(", ").collect::<Vec<_>>();
    lines.next().unwrap().unwrap();
    fn rec(design: &str, towels: &Vec<&str>) -> bool {
        if design.is_empty() {
            true
        } else {
            for towel in towels {
                if let Some(tail) = design.strip_prefix(towel) {
                    if rec(tail, towels) {
                        return true;
                    }
                }
            }
            false
        }
    }
    lines
        .map(|line| {
            let line = line.unwrap();
            let line = line.as_str();
            if rec(&line, &towels) {
                1
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
    fn example1() {
        assert_eq!(
            6,
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

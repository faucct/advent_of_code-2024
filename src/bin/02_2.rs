fn sum(reader: impl std::io::BufRead) -> usize {
    reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            if line.is_empty() {
                return false;
            }
            let mut levels = line
                .split_ascii_whitespace()
                .map(|level| level.parse::<i32>().unwrap());
            let mut prev = levels.next().unwrap();
            let diffs = levels
                .map(|next| next - std::mem::replace(&mut prev, next))
                .collect::<Vec<_>>();
            fn kind(diff: i32) -> usize {
                if (-3..=-1).contains(&diff) {
                    2
                } else if (1..=3).contains(&diff) {
                    1
                } else {
                    0
                }
            }
            fn all_good(kinds: [i32; 3]) -> bool {
                kinds[0] == 0 && (kinds[1] == 0 || kinds[2] == 0)
            }
            let mut kinds = [0; 3];
            for &diff in &diffs {
                kinds[kind(diff)] += 1;
            }
            if all_good(kinds) {
                return true;
            }
            let mut diffs = diffs.into_iter();
            let mut prev = diffs.next().unwrap();
            kinds[kind(prev)] -= 1;
            if all_good(kinds) {
                return true;
            }
            kinds[kind(prev)] += 1;
            for next in diffs {
                kinds[kind(prev)] -= 1;
                kinds[kind(next)] -= 1;
                kinds[kind(next + prev)] += 1;
                if all_good(kinds) {
                    return true;
                }
                kinds[kind(prev)] += 1;
                kinds[kind(next)] += 1;
                kinds[kind(next + prev)] -= 1;
                prev = next;
            }
            kinds[kind(prev)] -= 1;
            if all_good(kinds) {
                return true;
            }
            false
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            4,
            sum("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

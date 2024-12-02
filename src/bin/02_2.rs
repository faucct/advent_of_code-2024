fn sum(reader: impl std::io::BufRead) -> usize {
    reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| {
            if line.is_empty() {
                return false;
            }
            let levels = line
                .split_ascii_whitespace()
                .map(|level| level.parse::<i32>().unwrap());
            (0..levels.clone().count()).any(|removed| {
                let mut levels =
                    levels.clone()
                        .enumerate()
                        .filter_map(|(i, level)| if removed == i { None } else { Some(level) });
                let start = levels.next().unwrap();
                let mut prev = levels.next().unwrap();
                if !(-3..=3).contains(&(prev - start)) || prev == start {
                    return false;
                }
                let increasing = prev - start > 0;
                levels.all(|next| {
                    if !if increasing { 1..=3 } else { -3..=-1 }.contains(&(next - prev)) {
                        return false;
                    }
                    prev = next;
                    true
                })
            })
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

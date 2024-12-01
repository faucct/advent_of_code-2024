fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut lefts = Vec::new();
    let mut rights = std::collections::HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let (left, right) = line.split_once(" ").unwrap();
            lefts.push(left.trim().parse::<i32>().unwrap());
            *rights.entry(right.trim().parse::<i32>().unwrap()).or_insert(0i32) += 1;
        }
    }
    lefts.into_iter().map(|left| left * *rights.get(&left).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            31,
            sum("3   4
4   3
2   5
1   3
3   9
3   3"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

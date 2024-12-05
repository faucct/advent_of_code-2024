fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut sum = 0;
    let mut rules = std::collections::HashMap::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        if let Some((before, after)) = line.split_once("|") {
            rules
                .entry(before.parse::<i32>().unwrap())
                .or_insert(std::collections::HashSet::new())
                .insert(after.parse::<i32>().unwrap());
        } else {
            let mut pages = Vec::new();
            let mut incorrect = false;
            for next in line.split(",") {
                let next = next.parse::<i32>().unwrap();
                let rules = rules.get(&next);
                if let Some(j) = pages.iter().enumerate().find_map(|(j, prev)| {
                    if let Some(rules) = rules {
                        if rules.contains(prev) {
                            return Some(j);
                        }
                    }
                    None
                }) {
                    incorrect = true;
                    pages.insert(j, next);
                } else {
                    pages.push(next);
                }
            }
            if incorrect {
                sum += pages[pages.len() / 2];
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            123,
            sum("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

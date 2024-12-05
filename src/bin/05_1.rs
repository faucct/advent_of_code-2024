fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut sum = 0;
    let mut rules = Vec::new();
    for line in reader.lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            continue;
        }
        if let Some((before, after)) = line.split_once("|") {
            rules.push((
                before.parse::<i32>().unwrap(),
                after.parse::<i32>().unwrap(),
            ));
        } else {
            let mut pages = std::collections::HashMap::new();
            for (i, page) in line.split(",").enumerate() {
                let page = page.parse::<i32>().unwrap();
                pages.entry(page).or_insert(i);
            }
            if rules.iter().all(|&(before, after)| {
                if let (Some(before), Some(after)) = (pages.get(&before), pages.get(&after)) {
                    if after <= before {
                        return false;
                    }
                }
                true
            }) {
                for (&page, &i) in pages.iter() {
                    if i * 2 + 1 == pages.len() {
                        sum += page;
                    }
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
    fn example() {
        assert_eq!(
            143,
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

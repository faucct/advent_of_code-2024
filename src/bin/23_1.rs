fn sum(reader: impl std::io::BufRead) -> i64 {
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut graph = std::collections::HashMap::new();
    for line in &lines {
        if !line.is_empty() {
            let (from, to) = line.split_once("-").unwrap();
            graph
                .entry(from)
                .or_insert(std::collections::HashSet::new())
                .insert(to);
            graph
                .entry(to)
                .or_insert(std::collections::HashSet::new())
                .insert(from);
        }
    }
    let mut count = 0;
    for (from, tos) in &graph {
        for to1 in tos {
            for to2 in tos {
                if to1 != to2
                    && from < to1
                    && to1 < to2
                    && [from, to1, to2].into_iter().any(|s| s.starts_with("t"))
                    && graph[to1].contains(to2)
                {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            7,
            sum("kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

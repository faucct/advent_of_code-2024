fn sum(reader: impl std::io::BufRead) -> String {
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
    fn rec<'b, 'a: 'b>(
        max: &mut std::collections::HashSet<&'b str>,
        graph: &std::collections::HashMap<&'a str, std::collections::HashSet<&'a str>>,
        lan: &mut std::collections::HashSet<&'b str>,
        keys: &[&'a str],
    ) {
        if let Some((key, tail)) = keys.split_first() {
            let tos = &graph[key];
            if lan.iter().all(|lan| tos.contains(lan)) {
                lan.insert(key);
                if max.len() < lan.len() {
                    max.clear();
                    for lan in &*lan {
                        max.insert(lan);
                    }
                }
                rec(max, graph, lan, tail);
                lan.remove(key);
            }
            rec(max, graph, lan, tail);
        }
    }
    let mut max = std::collections::HashSet::new();
    rec(
        &mut max,
        &graph,
        &mut Default::default(),
        &graph.keys().map(|key| *key).into_iter().collect::<Vec<_>>(),
    );
    let mut max = max.into_iter().collect::<Vec<_>>();
    max.sort_unstable();
    max.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            "co,de,ka,ta",
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
    println!("{}", sum(std::io::stdin().lock()));
}

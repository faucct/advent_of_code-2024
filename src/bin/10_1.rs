fn sum(reader: impl std::io::BufRead) -> usize {
    let lines = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    fn rec(lines: &Vec<Vec<u8>>, to_visit: &mut Vec<Vec<bool>>, i: usize, j: usize) -> usize {
        let mut scores = 0;
        for next in [
            (i.wrapping_sub(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
            (i.wrapping_add(1), j),
        ] {
            if let Some(line) = lines.get(next.0) {
                if let Some(&cell) = line.get(next.1) {
                    if cell + 1 == lines[i][j]
                        && std::mem::replace(&mut to_visit[next.0][next.1], false)
                    {
                        if cell == b'0' {
                            scores += 1;
                        }
                        scores += rec(lines, to_visit, next.0, next.1);
                    }
                }
            }
        }
        scores
    }
    let mut scores = 0;
    let mut to_visit = vec![vec![true; lines[0].len()]; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell == b'9' {
                for line in &mut to_visit {
                    line.fill(true);
                }
                scores += rec(&lines, &mut to_visit, i, j);
            }
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            3,
            sum("10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"
                .as_bytes())
        );
        assert_eq!(
            36,
            sum("89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

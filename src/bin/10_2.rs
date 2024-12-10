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
    let mut counts = vec![vec![0; lines[0].len()]; lines.len()];
    let mut queue = std::collections::VecDeque::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell == b'9' {
                queue.push_back((i, j));
                counts[i][j] = 1;
            }
        }
    }
    for hard in (b'0'..=b'9').rev() {
        for _ in 0..queue.len() {
            let prev = queue.pop_front().unwrap();
            for next in [
                (prev.0.wrapping_sub(1), prev.1),
                (prev.0, prev.1.wrapping_sub(1)),
                (prev.0, prev.1.wrapping_add(1)),
                (prev.0.wrapping_add(1), prev.1),
            ] {
                if let Some(line) = lines.get(next.0) {
                    if let Some(&cell) = line.get(next.1) {
                        if cell + 1 == hard {
                            if counts[next.0][next.1] == 0 {
                                queue.push_back(next);
                            }
                            counts[next.0][next.1] += counts[prev.0][prev.1];
                        }
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for (counts, line) in counts.into_iter().zip(lines) {
        for (count, cell) in counts.into_iter().zip(line) {
            if cell == b'0' {
                sum += count;
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
            3,
            sum(".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."
                .as_bytes())
        );
        assert_eq!(
            81,
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

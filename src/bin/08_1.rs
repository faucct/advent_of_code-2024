fn sum(reader: impl std::io::BufRead) -> usize {
    let lines = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let mut frequencies_positions = std::collections::HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, &cell) in line.iter().enumerate() {
            if cell != b'.' {
                frequencies_positions
                    .entry(cell)
                    .or_insert(Vec::new())
                    .push((i, j));
            }
        }
    }
    let mut antinodes = std::collections::HashSet::new();
    for positions in frequencies_positions.values() {
        for (i, &a) in positions.iter().enumerate() {
            for &b in positions[..i].iter() {
                if let (Some(i), Some(j)) = ((b.0 * 2).checked_sub(a.0), (b.1 * 2).checked_sub(a.1))
                {
                    if i < lines.len() && j < lines[0].len() {
                        antinodes.insert((i, j));
                    }
                }
                if let (Some(i), Some(j)) = ((a.0 * 2).checked_sub(b.0), (a.1 * 2).checked_sub(b.1))
                {
                    if i < lines.len() && j < lines[0].len() {
                        antinodes.insert((i, j));
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            14,
            sum("............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

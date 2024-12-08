fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        b = std::mem::replace(&mut a, b) % b;
    }
    a
}

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
                let period = gcd(
                    (a.0 as isize - b.0 as isize).unsigned_abs(),
                    (a.1 as isize - b.1 as isize).unsigned_abs(),
                );
                let period = (
                    (a.0 as isize - b.0 as isize) / period as isize,
                    (a.1 as isize - b.1 as isize) / period as isize,
                );
                {
                    let mut b = (a.0 as isize, a.1 as isize);
                    loop {
                        antinodes.insert(b);
                        b = (b.0 + period.0, b.1 + period.1);
                        if !(0..lines.len() as isize).contains(&b.0)
                            || !(0..lines[0].len() as isize).contains(&b.1)
                        {
                            break;
                        }
                    }
                    let mut a = (a.0 as isize, a.1 as isize);
                    loop {
                        antinodes.insert(a);
                        a = (a.0 - period.0, a.1 - period.1);
                        if !(0..lines.len() as isize).contains(&a.0)
                            || !(0..lines[0].len() as isize).contains(&a.1)
                        {
                            break;
                        }
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
            34,
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

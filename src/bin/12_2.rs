fn sum(reader: impl std::io::BufRead) -> usize {
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    fn perimeter(lines: &mut Vec<Vec<u8>>, region: u8, i: usize, j: usize) -> usize {
        let prev = (i, j);
        let mut sum = 0;
        for ((i, j), horizontal) in [
            ((i.wrapping_sub(1), j), true),
            ((i, j.wrapping_sub(1)), false),
            ((i, j.wrapping_add(1)), false),
            ((i.wrapping_add(1), j), true),
        ] {
            if if let Some(cell) = lines.get_mut(i).and_then(|line| line.get_mut(j)) {
                if *cell == region {
                    *cell = 1;
                    sum += perimeter(lines, region, i, j);
                    false
                } else {
                    *cell != 1
                }
            } else {
                true
            } {
                if if horizontal {
                    let inner = lines
                        .get(prev.0)
                        .and_then(|line| line.get(prev.1.wrapping_sub(1)));
                    let outer = lines.get(i).and_then(|line| line.get(j.wrapping_sub(1)));
                    (inner != Some(&1) && inner != Some(&region))
                        || outer == Some(&1)
                        || outer == Some(&region)
                } else {
                    let inner = lines
                        .get(prev.0.wrapping_sub(1))
                        .and_then(|line| line.get(prev.1));
                    let outer = lines.get(i.wrapping_sub(1)).and_then(|line| line.get(j));
                    (inner != Some(&1) && inner != Some(&region))
                        || outer == Some(&1)
                        || outer == Some(&region)
                } {
                    sum += 1;
                }
            }
        }
        sum
    }
    fn area(lines: &mut Vec<Vec<u8>>, region: u8, i: usize, j: usize) -> usize {
        let mut sum = 1;
        for (i, j) in [
            (i.wrapping_sub(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
            (i.wrapping_add(1), j),
        ] {
            if let Some(cell) = lines.get_mut(i).and_then(|line| line.get_mut(j)) {
                if *cell == 1 {
                    *cell = 0;
                    sum += area(lines, region, i, j);
                }
            }
        }
        sum
    }
    let mut score = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] != 0 {
                let region = lines[i][j];
                lines[i][j] = 1;
                let perimeter = perimeter(&mut lines, region, i, j);
                lines[i][j] = 0;
                let area = area(&mut lines, region, i, j);
                score += area * perimeter;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            80,
            sum("AAAA
BBCD
BBCC
EEEC"
                .as_bytes())
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            436,
            sum("OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
                .as_bytes())
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            1206,
            sum("RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

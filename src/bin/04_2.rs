fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut count = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            if lines[row].as_bytes()[col] == b'A' {
                if (0..4).any(|shift| {
                    [
                        (row.wrapping_sub(1), col.wrapping_sub(1)),
                        (row.wrapping_sub(1), col.wrapping_add(1)),
                        (row.wrapping_add(1), col.wrapping_add(1)),
                        (row.wrapping_add(1), col.wrapping_sub(1)),
                    ]
                    .into_iter()
                    .enumerate()
                    .all(|(seq, (row, col))| {
                        Some(&b"MMSSMMSS"[seq + shift])
                            == lines.get(row).and_then(|line| line.as_bytes().get(col))
                    })
                }) {
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
    fn example() {
        assert_eq!(
            9,
            sum("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

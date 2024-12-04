fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut count = 0;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            for row_direction in [0, 1, usize::MAX] {
                for col_direction in [0, 1, usize::MAX] {
                    let mut row: usize = row;
                    let mut col = col;
                    if "XMAS".as_bytes().into_iter().all(|&b| {
                        if let Some(line) = lines.get(row) {
                            if let Some(&cell) = line.as_bytes().get(col) {
                                if cell == b {
                                    row = row.wrapping_add(row_direction);
                                    col = col.wrapping_add(col_direction);
                                    return true;
                                }
                            }
                        }
                        false
                    }) {
                        count += 1;
                    }
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
            18,
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

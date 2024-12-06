fn sum(reader: impl std::io::BufRead) -> usize {
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let state = lines
        .iter_mut()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter_mut().enumerate().find_map(|(j, cell)| {
                if let Some(direction) = [b'^', b'>', b'v', b'<']
                    .iter()
                    .position(|&direction| direction == *cell)
                {
                    *cell = b'.';
                    Some((i, j, direction))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let directions = [[usize::MAX, 0], [0, 1], [1, 0], [0, usize::MAX]];
    (0..lines.len())
        .map(|i| {
            (0..lines[0].len())
                .filter(|&j| {
                    let added = (i, j);
                    if state.0 == added.0 && state.1 == added.1 {
                        return false;
                    }
                    let mut state = state;
                    let mut count = 0;
                    loop {
                        count += 1;
                        let direction = directions[state.2];
                        let i = state.0.wrapping_add(direction[0]);
                        let j = state.1.wrapping_add(direction[1]);
                        if let Some(line) = lines.get(i) {
                            if let Some(cell) = line.get(j) {
                                if *cell == b'#' || (i, j) == added {
                                    state.2 = (state.2 + 1) % 4;
                                } else {
                                    state.0 = i;
                                    state.1 = j;
                                }
                                if count == lines.len() * lines[0].len() {
                                    return true;
                                }
                                continue;
                            }
                        }
                        return false;
                    }
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            6,
            sum("....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

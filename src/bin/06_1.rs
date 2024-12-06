fn sum(reader: impl std::io::BufRead) -> i32 {
    let mut lines = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .collect::<Vec<_>>();
    let mut state = lines
        .iter_mut()
        .enumerate()
        .find_map(|(i, line)| {
            line.iter_mut().enumerate().find_map(|(j, cell)| {
                if let Some(direction) = [b'^', b'>', b'v', b'<']
                    .iter()
                    .position(|&direction| direction == *cell)
                {
                    *cell = b'_';
                    Some((i, j, direction))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let directions = [[usize::MAX, 0], [0, 1], [1, 0], [0, usize::MAX]];
    let mut count = 1;
    loop {
        let direction = directions[state.2];
        let i = state.0.wrapping_add(direction[0]);
        let j = state.1.wrapping_add(direction[1]);
        if let Some(line) = lines.get_mut(i) {
            if let Some(cell) = line.get_mut(j) {
                match *cell {
                    b'#' => {
                        state.2 = (state.2 + 1) % 4;
                        continue;
                    }
                    b'.' => {
                        count += 1;
                        *cell = b'_';
                    }
                    _ => {}
                }
                state.0 = i;
                state.1 = j;
            }
        } else {
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            41,
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

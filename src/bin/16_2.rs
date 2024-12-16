use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    let mut map = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.unwrap().into_bytes();
            line.into_iter()
                .enumerate()
                .map(|(j, cell)| {
                    if cell == b'S' {
                        start = (i, j);
                    }
                    if cell == b'E' {
                        end = (i, j);
                    }
                    (cell, [usize::MAX; 4])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut queue = std::collections::BinaryHeap::new();
    const DIRECTIONS: [(usize, usize); 4] = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    queue.push((0i32, start, 1));
    let mut best = usize::MAX;
    while let Some(queued) = queue.pop() {
        if best < -queued.0 as usize {
            break;
        }
        if map[queued.1 .0][queued.1 .1].0 == b'E' {
            best = -queued.0 as usize;
        }
        let cell = &mut map[queued.1 .0][queued.1 .1];
        if cell.1[queued.2] != usize::MAX {
            continue;
        }
        cell.1[queued.2] = -queued.0 as usize;
        for queued in [
            (queued.0 - 1000, queued.1, (queued.2 + 3) % 4),
            (queued.0 - 1000, queued.1, (queued.2 + 1) % 4),
            (
                queued.0 - 1,
                (
                    queued.1 .0.wrapping_add(DIRECTIONS[queued.2].0),
                    queued.1 .1.wrapping_add(DIRECTIONS[queued.2].1),
                ),
                queued.2,
            ),
        ] {
            let cell = &map[queued.1 .0][queued.1 .1];
            if cell.1[queued.2] == usize::MAX && cell.0 != b'#' {
                queue.push(queued);
            }
        }
    }
    fn rec(map: &mut Vec<Vec<(u8, [usize; 4])>>, queued: (i32, (usize, usize), usize)) {
        let cell = &mut map[queued.1 .0][queued.1 .1];
        if cell.0 == b'#' || ((-queued.0) as usize) != cell.1[queued.2] {
            return;
        }
        cell.1[queued.2] = 0;
        for queued in [
            (queued.0 + 1000, queued.1, (queued.2 + 3) % 4),
            (queued.0 + 1000, queued.1, (queued.2 + 1) % 4),
            (
                queued.0 + 1,
                (
                    queued.1 .0.wrapping_sub(DIRECTIONS[queued.2].0),
                    queued.1 .1.wrapping_sub(DIRECTIONS[queued.2].1),
                ),
                queued.2,
            ),
        ] {
            rec(map, queued);
        }
    }
    for direction in 0..4 {
        rec(&mut map, (-(best as i32), end, direction));
    }
    map.into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|cell| cell.1.into_iter().any(|direction| direction == 0))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            45,
            sum("###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
                .as_bytes())
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            64,
            sum("#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut start = (usize::MAX, usize::MAX);
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
                    (cell, [false; 4])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut queue = std::collections::BinaryHeap::new();
    let directions = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    queue.push((0i32, start, 1));
    while let Some(queued) = queue.pop() {
        if map[queued.1 .0][queued.1 .1].0 == b'E' {
            return -queued.0 as usize;
        }
        if std::mem::replace(&mut map[queued.1 .0][queued.1 .1].1[queued.2], true) {
            continue;
        }
        for queued in [
            (queued.0 - 1000, queued.1, (queued.2 + 3) % 4),
            (queued.0 - 1000, queued.1, (queued.2 + 1) % 4),
            (
                queued.0 - 1,
                (
                    queued.1 .0.wrapping_add(directions[queued.2].0),
                    queued.1 .1.wrapping_add(directions[queued.2].1),
                ),
                queued.2,
            ),
        ] {
            let cell = &map[queued.1 .0][queued.1 .1];
            if !cell.1[queued.2] && cell.0 != b'#' {
                queue.push(queued);
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            7036,
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
            11048,
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

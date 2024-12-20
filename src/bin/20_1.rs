use std::usize;

fn sum(save: usize, reader: impl std::io::BufRead) -> std::collections::HashMap<usize, usize> {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    let map = reader
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let line = line.unwrap().into_bytes();
            if line.is_empty() {
                return None;
            }
            for (j, &cell) in line.iter().enumerate() {
                match cell {
                    b'S' => start = (i, j),
                    b'E' => end = (i, j),
                    _ => {}
                }
            }
            Some(line)
        })
        .collect::<Vec<_>>();
    let mut start_times = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    let mut len = queue.len();
    let mut time = 1;
    let mut max_time = usize::MAX;
    while let Some((i, j)) = queue.pop_front() {
        for (i, j) in [
            (i.wrapping_sub(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
            (i.wrapping_add(1), j),
        ] {
            if let Some(&cell) = map.get(i).and_then(|row| row.get(j)) {
                if cell != b'#' && start_times[i][j] == usize::MAX {
                    if cell == b'E' {
                        max_time = time;
                    }
                    start_times[i][j] = time;
                    queue.push_back((i, j));
                }
            }
        }
        len -= 1;
        if len == 0 {
            if max_time != usize::MAX {
                break;
            }
            len = queue.len();
            time += 1;
        }
    }

    let mut end_times = vec![vec![usize::MAX; map[0].len()]; map.len()];
    end_times[end.0][end.1] = 0;
    queue.clear();
    queue.push_back(end);
    let mut len = queue.len();
    let mut time = 1;
    while let Some((i, j)) = queue.pop_front() {
        for (i, j) in [
            (i.wrapping_sub(1), j),
            (i, j.wrapping_sub(1)),
            (i, j.wrapping_add(1)),
            (i.wrapping_add(1), j),
        ] {
            if let Some(row) = map.get(i) {
                if let Some(&cell) = row.get(j) {
                    if cell != b'#' && end_times[i][j] == usize::MAX {
                        end_times[i][j] = time;
                        queue.push_back((i, j));
                    }
                }
            }
        }
        len -= 1;
        if len == 0 {
            len = queue.len();
            time += 1;
        }
    }
    let mut count = std::collections::HashMap::new();
    for (i, times) in end_times.iter().enumerate() {
        for (j, &time) in times.iter().enumerate() {
            if max_time < start_times[i][j] {
                continue;
            }
            let start_time = time;
            const CHEAT: usize = 2;
            for j_diff in 0..=CHEAT {
                for end in [(i, j.wrapping_sub(j_diff)), (i, j.wrapping_add(j_diff))] {
                    if let Some(&time) = end_times.get(end.0).and_then(|row| row.get(end.1)) {
                        if time != usize::MAX && start_time >= time + j_diff + save {
                            *count.entry(start_time - time - j_diff).or_default() += 1;
                        }
                    }
                }
            }
            for i_diff in 0..=CHEAT {
                for end in [(i.wrapping_sub(i_diff), j), (i.wrapping_add(i_diff), j)] {
                    if let Some(&time) = end_times.get(end.0).and_then(|row| row.get(end.1)) {
                        if time != usize::MAX && start_time >= time + i_diff + save {
                            *count.entry(start_time - time - i_diff).or_default() += 1;
                        }
                    }
                }
            }
            for i_diff in 1..=CHEAT {
                for j_diff in 1..=CHEAT - i_diff {
                    for end in [
                        (i.wrapping_sub(i_diff), j.wrapping_sub(j_diff)),
                        (i.wrapping_sub(i_diff), j.wrapping_add(j_diff)),
                        (i.wrapping_add(i_diff), j.wrapping_sub(j_diff)),
                        (i.wrapping_add(i_diff), j.wrapping_add(j_diff)),
                    ] {
                        if let Some(&time) = end_times.get(end.0).and_then(|row| row.get(end.1)) {
                            if time != usize::MAX && start_time >= time + i_diff + j_diff + save {
                                *count
                                    .entry(start_time - time - i_diff - j_diff)
                                    .or_default() += 1;
                            }
                        }
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
    fn example1() {
        assert_eq!(
            std::collections::HashMap::from([
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1),
            ]),
            sum(
                1,
                "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!(
        "{:?}",
        sum(100, std::io::stdin().lock())
            .into_iter()
            .map(|(save, count)| if save < 100 { 0 } else { count })
            .sum::<usize>()
    );
}

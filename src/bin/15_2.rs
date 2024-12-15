use std::collections::{hash_map, HashMap};

fn sum(reader: impl std::io::BufRead) -> usize {
    let mut map = Vec::new();
    let mut lines = reader.lines();
    let mut robot = (usize::MAX, usize::MAX);
    while let Some(line) = lines.next() {
        let line = line.unwrap().into_bytes();
        if line.is_empty() {
            break;
        }
        for (i, &cell) in line.iter().enumerate() {
            if cell == b'@' {
                robot = (map.len(), i * 2);
            }
        }
        map.push(
            line.into_iter()
                .flat_map(|cell| match cell {
                    b'#' => b"##",
                    b'O' => b"[]",
                    b'.' => b"..",
                    b'@' => b"@.",
                    _ => panic!("{:?}", cell as char),
                })
                .copied()
                .collect::<Vec<_>>(),
        );
    }
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let mut visited = std::collections::HashMap::new();
        for mov in line.into_bytes() {
            let direction = match mov {
                b'<' => (0, usize::MAX),
                b'^' => (usize::MAX, 0),
                b'>' => (0, 1),
                b'v' => (1, 0),
                _ => panic!(),
            };
            fn test(
                map: &mut Vec<Vec<u8>>,
                direction: (usize, usize),
                prev: u8,
                visited: &mut HashMap<(usize, usize), u8>,
                position: (usize, usize),
            ) -> bool {
                if let Some(&cell) = map.get(position.0).and_then(|row| row.get(position.1)) {
                    match visited.entry(position) {
                        hash_map::Entry::Occupied(mut occupied) => {
                            if *occupied.get() == b'.' {
                                *occupied.get_mut() = prev;
                            }
                            true
                        }
                        hash_map::Entry::Vacant(vacant) => {
                            vacant.insert(prev);
                            match cell {
                                b'#' => false,
                                b'.' => true,
                                b'@' => test(
                                    map,
                                    direction,
                                    cell,
                                    visited,
                                    (
                                        position.0.wrapping_add(direction.0),
                                        position.1.wrapping_add(direction.1),
                                    ),
                                ),
                                b'[' => {
                                    test(
                                        map,
                                        direction,
                                        cell,
                                        visited,
                                        (
                                            position.0.wrapping_add(direction.0),
                                            position.1.wrapping_add(direction.1),
                                        ),
                                    ) && (direction.0 == 0
                                        || test(
                                            map,
                                            direction,
                                            b'.',
                                            visited,
                                            (position.0, position.1.wrapping_add(1)),
                                        ))
                                }
                                b']' => {
                                    test(
                                        map,
                                        direction,
                                        cell,
                                        visited,
                                        (
                                            position.0.wrapping_add(direction.0),
                                            position.1.wrapping_add(direction.1),
                                        ),
                                    ) && (direction.0 == 0
                                        || test(
                                            map,
                                            direction,
                                            b'.',
                                            visited,
                                            (position.0, position.1.wrapping_sub(1)),
                                        ))
                                }
                                _ => panic!(),
                            }
                        }
                    }
                } else {
                    false
                }
            }
            visited.clear();
            if test(&mut map, direction, b'.', &mut visited, robot) {
                robot = (
                    robot.0.wrapping_add(direction.0),
                    robot.1.wrapping_add(direction.1),
                );
                for (position, val) in visited.drain() {
                    map[position.0][position.1] = val;
                }
            }
        }
    }
    let mut sum = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == b'[' {
                sum += 100 * i + j;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(
            105 + 207 + 306,
            sum("#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
                .as_bytes())
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            9021,
            sum("##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

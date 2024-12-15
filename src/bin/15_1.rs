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
                robot = (map.len(), i);
            }
        }
        map.push(line);
    }
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        'mov: for mov in line.into_bytes() {
            let direction = match mov {
                b'<' => (0, usize::MAX),
                b'^' => (usize::MAX, 0),
                b'>' => (0, 1),
                b'v' => (1, 0),
                _ => panic!(),
            };
            let mut len = 0;
            {
                let mut position = robot;
                loop {
                    position = (
                        position.0.wrapping_add(direction.0),
                        position.1.wrapping_add(direction.1),
                    );
                    if let Some(&cell) = map.get(position.0).and_then(|row| row.get(position.1)) {
                        len += 1;
                        match cell {
                            b'#' => {
                                continue 'mov;
                            }
                            b'.' => {
                                break;
                            }
                            _ => {}
                        }
                    } else {
                        len = 0;
                        break;
                    }
                }
            }
            let mut position = robot;
            robot = (
                robot.0.wrapping_add(direction.0),
                robot.1.wrapping_add(direction.1),
            );
            let mut prev = b'.';
            for _ in 0..=len {
                std::mem::swap(&mut prev, &mut map[position.0][position.1]);
                position = (
                    position.0.wrapping_add(direction.0),
                    position.1.wrapping_add(direction.1),
                );
            }
            if prev != b'.' {
                panic!("{}", prev as char);
            }
        }
    }
    let mut sum = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == b'O' {
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
    fn example1() {
        assert_eq!(
            2028,
            sum("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
                .as_bytes())
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            10092,
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

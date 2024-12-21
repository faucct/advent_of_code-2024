use std::usize;

fn sum(reader: impl std::io::BufRead) -> usize {
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap().into_bytes();
            if line.is_empty() {
                0
            } else {
                let mut visited = std::collections::HashSet::new();
                let mut queue = std::collections::VecDeque::new();
                queue.push_back(([b'A', b'A'], b'A', 0));
                let mut len: usize = 1;
                let mut presses = 1;
                while let Some(front) = queue.pop_front() {
                    'press: for &(mut press) in b"<v>^A" {
                        let mut front = front;
                        'robots: {
                            for state in &mut front.0 {
                                if press == b'A' {
                                    press = *state;
                                } else {
                                    *state = match *state {
                                        b'A' => match press {
                                            b'<' => b'^',
                                            b'v' => b'>',
                                            _ => continue 'press,
                                        },
                                        b'^' => match press {
                                            b'>' => b'A',
                                            b'v' => b'v',
                                            _ => continue 'press,
                                        },
                                        b'>' => match press {
                                            b'^' => b'A',
                                            b'<' => b'v',
                                            _ => continue 'press,
                                        },
                                        b'v' => match press {
                                            b'^' => b'^',
                                            b'>' => b'>',
                                            b'<' => b'<',
                                            _ => continue 'press,
                                        },
                                        b'<' => match press {
                                            b'>' => b'v',
                                            _ => continue 'press,
                                        },
                                        _ => panic!(),
                                    };
                                    break 'robots;
                                }
                            }
                            if press == b'A' {
                                if line[front.2] == front.1 {
                                    front.2 += 1;
                                    if front.2 as usize == line.len() {
                                        let mut number = 0;
                                        for b in line {
                                            if b.is_ascii_digit() {
                                                number = number * 10 + (b - b'0') as usize;
                                            }
                                        }
                                        return number * presses;
                                    }
                                }
                            } else {
                                front.1 = match front.1 {
                                    b'A' => match press {
                                        b'<' => b'0',
                                        b'^' => b'3',
                                        _ => continue 'press,
                                    },
                                    b'0' => match press {
                                        b'>' => b'A',
                                        b'^' => b'2',
                                        _ => continue 'press,
                                    },
                                    b'1' => match press {
                                        b'>' => b'2',
                                        b'^' => b'4',
                                        _ => continue 'press,
                                    },
                                    b'2' => match press {
                                        b'<' => b'1',
                                        b'v' => b'0',
                                        b'>' => b'3',
                                        b'^' => b'5',
                                        _ => continue 'press,
                                    },
                                    b'3' => match press {
                                        b'<' => b'2',
                                        b'^' => b'6',
                                        b'v' => b'A',
                                        _ => continue 'press,
                                    },
                                    b'4' => match press {
                                        b'v' => b'1',
                                        b'>' => b'5',
                                        b'^' => b'7',
                                        _ => continue 'press,
                                    },
                                    b'5' => match press {
                                        b'v' => b'2',
                                        b'<' => b'4',
                                        b'>' => b'6',
                                        b'^' => b'8',
                                        _ => continue 'press,
                                    },
                                    b'6' => match press {
                                        b'<' => b'5',
                                        b'v' => b'3',
                                        b'^' => b'9',
                                        _ => continue 'press,
                                    },
                                    b'7' => match press {
                                        b'v' => b'4',
                                        b'>' => b'8',
                                        _ => continue 'press,
                                    },
                                    b'8' => match press {
                                        b'v' => b'5',
                                        b'<' => b'7',
                                        b'>' => b'9',
                                        _ => continue 'press,
                                    },
                                    b'9' => match press {
                                        b'<' => b'8',
                                        b'v' => b'6',
                                        _ => continue 'press,
                                    },
                                    _ => panic!(),
                                };
                            }
                        }
                        if visited.insert(front) {
                            queue.push_back(front);
                        }
                    }
                    len -= 1;
                    if len == 0 {
                        len = queue.len();
                        presses += 1;
                    }
                }
                panic!()
            }
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            126384,
            sum("029A
980A
179A
456A
379A"
                .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

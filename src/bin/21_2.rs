use core::panic;
use std::usize;

fn sum(depth: usize, reader: impl std::io::BufRead) -> usize {
    #[derive(Default)]
    struct Directions<T> {
        a: T,
        up: T,
        down: T,
        left: T,
        right: T,
    }
    let mut prev = Directions::<Directions<usize>>::default();
    for _ in 0..depth {
        let mut next = Directions {
            a: Directions {
                a: 0,
                up: usize::MAX,
                down: usize::MAX,
                left: usize::MAX,
                right: usize::MAX,
            },
            up: Directions {
                a: usize::MAX,
                up: 0,
                down: usize::MAX,
                left: usize::MAX,
                right: usize::MAX,
            },
            down: Directions {
                a: usize::MAX,
                up: usize::MAX,
                down: 0,
                left: usize::MAX,
                right: usize::MAX,
            },
            left: Directions {
                a: usize::MAX,
                up: usize::MAX,
                down: usize::MAX,
                left: 0,
                right: usize::MAX,
            },
            right: Directions {
                a: usize::MAX,
                up: usize::MAX,
                down: usize::MAX,
                left: usize::MAX,
                right: 0,
            },
        };
        for _ in 0..3 {
            next = Directions {
                a: Directions {
                    a: 0,
                    up: prev.a.left + 1 + prev.left.a,
                    down: (prev.a.left + 1 + prev.left.down + 1 + prev.down.a)
                        .min(prev.a.down + 1 + prev.down.left + 1 + prev.left.a),
                    right: prev.a.down + 1 + prev.down.a,
                    left: (prev.a.left + 1 + prev.left.down + 1 + prev.down.left + 1 + prev.left.a)
                        .min(prev.a.down + 1 + prev.down.left + 2 + prev.left.a),
                },
                up: Directions {
                    a: prev.a.right + 1 + prev.right.a,
                    up: 0,
                    down: prev.a.down + 1 + prev.down.a,
                    left: (prev.a.down + 1 + prev.down.left + 1 + prev.left.a),
                    right: (prev.a.down + 1 + prev.down.right + 1 + prev.right.a)
                        .min(prev.a.right + 1 + prev.right.down + 1 + prev.down.a),
                },
                down: Directions {
                    a: (prev.a.up + 1 + prev.up.right + 1 + prev.right.a)
                        .min(prev.a.right + 1 + prev.right.up + 1 + prev.up.a),
                    up: prev.a.up + 1 + prev.up.a,
                    down: 0,
                    left: prev.a.left + 1 + prev.left.a,
                    right: prev.a.right + 1 + prev.right.a,
                },
                left: Directions {
                    a: (prev.a.right
                        + 1
                        + (1 + prev.right.up + 1 + prev.up.a)
                            .min(prev.right.up + 1 + prev.up.right + 1 + prev.right.a)),
                    up: prev.a.right + 1 + prev.right.up + 1 + prev.up.a,
                    down: prev.a.right + 1 + prev.right.a,
                    left: 0,
                    right: prev.a.right + 2 + prev.right.a,
                },
                right: Directions {
                    a: prev.a.up + 1 + prev.up.a,
                    up: (prev.a.up + 1 + prev.up.left + 1 + prev.left.a)
                        .min(prev.a.left + 1 + prev.left.up + 1 + prev.up.a),
                    down: prev.a.left + 1 + prev.left.a,
                    left: prev.a.left + 2 + prev.left.a,
                    right: 0,
                },
            };
        }
        prev = next;
    }
    let matrix = prev;
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap().into_bytes();
            if line.is_empty() {
                return 0;
            }
            let mut prev = b'A';
            let presses = line
                .iter()
                .map(|&next| {
                    1 + match std::mem::replace(&mut prev, next) {
                        b'A' => match next {
                            b'A' => 0,
                            b'0' => matrix.a.left + 1 + matrix.left.a,
                            b'1' => (matrix.a.up + 1 + matrix.up.left + 2 + matrix.left.a).min(
                                matrix.a.left
                                    + 1
                                    + matrix.left.up
                                    + 1
                                    + matrix.up.left
                                    + 1
                                    + matrix.left.a,
                            ),
                            b'2' => (matrix.a.up + 1 + matrix.up.left + 1 + matrix.left.a)
                                .min(matrix.a.left + 1 + matrix.left.up + 1 + matrix.up.a),
                            b'3' => matrix.a.up + 1 + matrix.up.a,
                            b'4' => (matrix.a.left
                                + 1
                                + matrix.left.up
                                + 1
                                + (1 + matrix.up.left + 1 + matrix.left.a)
                                    .min(matrix.up.left + 1 + matrix.left.up + 1 + matrix.up.a))
                            .min(matrix.a.up + 2 + matrix.up.left + 2 + matrix.left.a),
                            b'5' => (matrix.a.up
                                + 1
                                + (1 + matrix.up.left + 1 + matrix.left.a)
                                    .min(matrix.up.left + 1 + matrix.left.up + 1 + matrix.up.a))
                            .min(matrix.a.left + 1 + matrix.left.up + 2 + matrix.up.a),
                            b'6' => matrix.a.up + 2 + matrix.up.a,
                            b'7' => matrix.a.up + 3 + matrix.up.left + 2 + matrix.left.a,
                            b'8' => (matrix.a.up + 3 + matrix.up.left + 1 + matrix.left.a)
                                .min(matrix.a.left + 1 + matrix.left.up + 3 + matrix.up.a),
                            b'9' => matrix.a.up + 3 + matrix.up.a,
                            _ => panic!(),
                        },
                        b'0' => match next {
                            b'A' => matrix.a.right + 1 + matrix.right.a,
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => matrix.a.up + 1 + matrix.up.a,
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => matrix.a.up + 3 + matrix.up.a,
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'1' => match next {
                            b'A' => (matrix.a.right + 2 + matrix.right.down + 1 + matrix.down.a)
                                .min(
                                    matrix.a.right
                                        + 1
                                        + matrix.right.down
                                        + 1
                                        + matrix.down.right
                                        + 1
                                        + matrix.right.a,
                                ),
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => matrix.a.up + 2 + matrix.up.a,
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'2' => match next {
                            b'A' => (matrix.a.down + 1 + matrix.down.right + 1 + matrix.right.a)
                                .min(matrix.a.right + 1 + matrix.right.down + 1 + matrix.down.a),
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => todo!(),
                            b'9' => (matrix.a.right + 1 + matrix.right.up + 2 + matrix.up.a).min(
                                matrix.a.up
                                    + 1
                                    + (1 + matrix.up.right + 1 + matrix.right.a).min(
                                        matrix.up.right + 1 + matrix.right.up + 1 + matrix.up.a,
                                    ),
                            ),
                            _ => panic!(),
                        },
                        b'3' => match next {
                            b'A' => matrix.a.down + 1 + matrix.down.a,
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => (matrix.a.up + 1 + matrix.up.left + 2 + matrix.left.a)
                                .min(matrix.a.left + 2 + matrix.left.up + 1 + matrix.up.a),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => {
                                matrix.a.left
                                    + 1
                                    + (1 + matrix.left.up + 2 + matrix.up.a).min(
                                        matrix.left.up
                                            + 1
                                            + (1 + matrix.up.left + 1 + matrix.left.a).min(
                                                matrix.up.left
                                                    + 1
                                                    + matrix.left.up
                                                    + 1
                                                    + matrix.up.a,
                                            ),
                                    )
                            }
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'4' => match next {
                            b'A' => todo!(),
                            b'0' => todo!(),
                            b'1' => matrix.a.down + 1 + matrix.down.a,
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => matrix.a.right + 1 + matrix.right.a,
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'5' => match next {
                            b'A' => todo!(),
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => matrix.a.right + 1 + matrix.right.a,
                            b'7' => todo!(),
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'6' => match next {
                            b'A' => matrix.a.down + 2 + matrix.down.a,
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'7' => match next {
                            b'A' => todo!(),
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => (matrix.a.right + 2 + matrix.right.down + 2 + matrix.down.a)
                                .min(matrix.a.down + 2 + matrix.down.right + 2 + matrix.right.a),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => matrix.a.right + 1 + matrix.right.a,
                            b'9' => matrix.a.right + 2 + matrix.right.a,
                            _ => panic!(),
                        },
                        b'8' => match next {
                            b'A' => todo!(),
                            b'0' => matrix.a.down + 3 + matrix.down.a,
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => (matrix.a.right + 1 + matrix.right.down + 2 + matrix.down.a)
                                .min(matrix.a.down + 2 + matrix.down.right + 1 + matrix.right.a),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => todo!(),
                            b'8' => todo!(),
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        b'9' => match next {
                            b'A' => matrix.a.down + 3 + matrix.down.a,
                            b'0' => todo!(),
                            b'1' => todo!(),
                            b'2' => todo!(),
                            b'3' => todo!(),
                            b'4' => todo!(),
                            b'5' => todo!(),
                            b'6' => todo!(),
                            b'7' => matrix.a.left + 2 + matrix.left.a,
                            b'8' => matrix.a.left + 1 + matrix.left.a,
                            b'9' => todo!(),
                            _ => panic!(),
                        },
                        _ => panic!(),
                    }
                })
                .sum::<usize>();
            let mut number = 0;
            for b in line {
                if b.is_ascii_digit() {
                    number = number * 10 + (b - b'0') as usize;
                }
            }
            presses * number
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
            sum(
                2,
                "029A
980A
179A
456A
379A"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{:?}", sum(25, std::io::stdin().lock()));
}

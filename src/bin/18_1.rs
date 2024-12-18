use std::usize;

fn sum(limit: usize, reader: impl std::io::BufRead) -> usize {
    let mut width = 0;
    let mut height = 0;
    let bytes = reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                return None;
            }
            let (x, y) = line.split_once(",").unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            width = width.max(x + 1);
            height = height.max(y + 1);
            Some((y, x))
        })
        .collect::<Vec<_>>();
    let mut map = vec![vec![true; width]; height];
    for &byte in &bytes[..limit] {
        map[byte.0][byte.1] = false;
    }
    let mut vec_deque = std::collections::VecDeque::new();
    vec_deque.push_back((0usize, 0usize));
    let mut steps = 1;
    let mut len = 1;
    while let Some((y, x)) = vec_deque.pop_front() {
        for (y, x) in [
            (y.wrapping_sub(1), x),
            (y, x.wrapping_sub(1)),
            (y, x.wrapping_add(1)),
            (y.wrapping_add(1), x),
        ] {
            if y == map.len() - 1 && x == map[0].len() - 1 {
                return steps;
            }
            if let Some(cell) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
                if std::mem::replace(cell, false) {
                    vec_deque.push_back((y, x));
                }
            }
        }
        len -= 1;
        if len == 0 {
            len = vec_deque.len();
            steps += 1;
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
            22,
            sum(
                12,
                "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
                .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(1024, std::io::stdin().lock()));
}

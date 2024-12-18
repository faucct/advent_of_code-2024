use std::usize;

fn sum(reader: impl std::io::BufRead) -> (usize, usize) {
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
            Some((x, y))
        })
        .collect::<Vec<_>>();
    let mut vec_deque = std::collections::VecDeque::new();
    let mut map = vec![vec![true; width]; height];
    let mut limits = 0..bytes.len();
    'limits: while !limits.is_empty() {
        let limit = (limits.start + limits.end) / 2;
        for row in &mut map {
            row.fill(true);
        }
        for &byte in &bytes[..limit] {
            map[byte.1][byte.0] = false;
        }
        vec_deque.clear();
        vec_deque.push_back((0usize, 0usize));
        while let Some((y, x)) = vec_deque.pop_front() {
            for (y, x) in [
                (y.wrapping_sub(1), x),
                (y, x.wrapping_sub(1)),
                (y, x.wrapping_add(1)),
                (y.wrapping_add(1), x),
            ] {
                if y == map.len() - 1 && x == map[0].len() - 1 {
                    limits.start = limit + 1;
                    continue 'limits;
                }
                if let Some(cell) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
                    if std::mem::replace(cell, false) {
                        vec_deque.push_back((y, x));
                    }
                }
            }
        }
        limits.end = limit;
    }
    bytes[limits.end - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            (6, 1),
            sum("5,4
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
            .as_bytes())
        );
    }
}

fn main() {
    println!("{:?}", sum(std::io::stdin().lock()));
}

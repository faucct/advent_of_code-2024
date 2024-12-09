fn sum(reader: impl std::io::BufRead) -> usize {
    let bytes = std::io::read_to_string(reader).unwrap().into_bytes();
    let mut blocks = bytes
        .iter()
        .enumerate()
        .map(|(id, &len)| {
            let len = len as usize - b'0' as usize;
            if id % 2 == 0 {
                (vec![(id / 2, len)], len..len)
            } else {
                (vec![], 0..len)
            }
        })
        .collect::<Vec<_>>();
    'from_id: for from_id in (0..blocks.len()).rev() {
        let from = &blocks[from_id];
        for &id in &from.0 {
            if id.0 * 2 == from_id {
                for free in 0..blocks.len() {
                    if from_id < free {
                        break;
                    }
                    if from.1.end <= blocks[free].1.len() {
                        let len = from.1.end;
                        let from = &mut blocks[from_id];
                        from.1.start = 0;
                        from.0.pop();
                        blocks[free].0.push(id);
                        blocks[free].1.start += len;
                        continue 'from_id;
                    }
                }
                break;
            }
        }
    }
    let mut checksum = 0;
    let mut position = 0;
    for block in &blocks {
        for block in &block.0 {
            for _ in 0..block.1 {
                checksum += position * block.0;
                position += 1;
            }
        }
        position += block.1.len();
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(2858, sum("2333133121414131402".as_bytes()));
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

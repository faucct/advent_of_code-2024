fn sum(reader: impl std::io::BufRead) -> usize {
    let bytes = std::io::read_to_string(reader).unwrap().into_bytes();
    let mut positions = 0;
    let mut head = bytes.iter().enumerate().flat_map(|(id, &len)| {
        let id = if id % 2 == 0 { id / 2 } else { usize::MAX };
        let len = len as usize - b'0' as usize;
        let position = positions;
        positions += len;
        (position..positions).map(move |position| (position, id))
    });
    let mut positions = bytes
        .iter()
        .map(|&len| len as usize - b'0' as usize)
        .sum::<usize>();
    let tail = bytes.iter().enumerate().rev().flat_map(|(id, &len)| {
        let id = if id % 2 == 0 { id / 2 } else { usize::MAX };
        let len = len as usize - b'0' as usize;
        let position = positions;
        positions -= len;
        (if id == usize::MAX {
            position
        } else {
            positions
        }..position)
            .rev()
            .map(move |position| (position, id))
    });
    let mut checksum = 0;
    for tail in tail {
        if tail.1 != usize::MAX {
            while let Some(head) = head.next() {
                if head.0 < tail.0 {
                    if head.1 == usize::MAX {
                        checksum += head.0 * tail.1;
                        break;
                    } else {
                        checksum += head.0 * head.1;
                    }
                } else {
                    if tail.0 == head.0 {
                        checksum += head.0 * head.1;
                    }
                    break;
                }
            }
        }
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(1928, sum("2333133121414131402".as_bytes()));
    }
}

fn main() {
    println!("{}", sum(std::io::stdin().lock()));
}

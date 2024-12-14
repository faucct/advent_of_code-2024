fn sum(height: usize, width: usize, reader: impl std::io::BufRead) -> usize {
    let regex = regex::Regex::new("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)$").unwrap();
    let mut quadrants = [0; 4];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let line = regex.captures(&line).unwrap();
        let p = (
            line.get(1).unwrap().as_str().parse::<i128>().unwrap(),
            line.get(2).unwrap().as_str().parse::<i128>().unwrap(),
        );
        let v = (
            line.get(3).unwrap().as_str().parse::<i128>().unwrap(),
            line.get(4).unwrap().as_str().parse::<i128>().unwrap(),
        );
        let y = (p.0 + v.0 * 100).rem_euclid(width as i128) as usize;
        let x = (p.1 + v.1 * 100).rem_euclid(height as i128) as usize;
        println!("{x} {y}");
        quadrants[if y < width / 2 {
            0
        } else if y > width / 2 {
            2
        } else {
            continue;
        } + if x < height / 2 {
            0
        } else if x > height / 2 {
            1
        } else {
            continue;
        }] += 1;
    }
    quadrants.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            12,
            sum(
                7,
                11,
                "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
                    .as_bytes()
            )
        );
    }
}

fn main() {
    println!("{}", sum(103, 101, std::io::stdin().lock()));
}

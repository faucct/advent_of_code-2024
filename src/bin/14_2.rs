use core::str;

fn sum(height: usize, width: usize, reader: impl std::io::BufRead) {
    let regex = regex::Regex::new("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)$").unwrap();
    let mut map = vec![vec![b'.'; width]; height];
    let mut robots = reader.lines().map(|line| {
        let line = line.unwrap();
        let line = regex.captures(&line).unwrap();
        let p = (
            line.get(1).unwrap().as_str().parse::<i128>().unwrap(),
            line.get(2).unwrap().as_str().parse::<i128>().unwrap(),
        );
        let v = (
            line.get(3).unwrap().as_str().parse::<i128>().unwrap(),
            line.get(4).unwrap().as_str().parse::<i128>().unwrap(),
        );
        (p, v)
    }).collect::<Vec<_>>();
    for seconds in 0..10000 {
        let mut xs = [0; 103];
        let mut ys = [0; 103];
        for robot in &mut robots {
            xs[robot.0.0 as usize] += 1;
            ys[robot.0.1 as usize] += 1;
        }
        if xs.into_iter().max().unwrap() > 20 || ys.into_iter().max().unwrap() > 20 {
            println!("{}", seconds);
            for robot in &mut robots {
                map[robot.0.1 as usize][robot.0.0 as usize] = b'X';
            }
            for row in &mut map {
                println!("{}", str::from_utf8(&*row).unwrap());
                row.fill(b'.');
            }
            println!("");
        }
        for robot in &mut robots {
            robot.0.0 = (robot.0.0 + robot.1.0).rem_euclid(width as i128);
            robot.0.1 = (robot.0.1 + robot.1.1).rem_euclid(height as i128);
            xs[robot.0.0 as usize] += 1;
            ys[robot.0.1 as usize] += 1;
        }
    }
}

fn main() {
    sum(103, 101, std::io::stdin().lock());
}

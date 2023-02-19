use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Write},
    process,
};

const POWER: u32 = 234;
// const DX: [i32; 4] = [0, 1, 0, -1];
// const DY: [i32; 4] = [1, 0, -1, 0];

fn query(x: i32, y: i32, p: u32) -> bool {
    println!("{} {} {}", x, y, p);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let res = input.trim().parse::<i32>().unwrap();
    if res == -1 || res == 2 {
        process::exit(0);
    }
    if res != 1 {
        return query(x, y, p);
    }
    true
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn dist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

fn input() -> (u32, u32, u32, u32, Vec<Point>, Vec<Point>) {
    let (n, w, k, _c) = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|i| i.parse::<u32>().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };
    let wsrc: Vec<Point> = (0..w)
        .map(|_| {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());
            let tmp = Point::new(iter.next().unwrap(), iter.next().unwrap());
            tmp
        })
        .collect();
    let house: Vec<Point> = (0..k)
        .map(|_| {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());
            let tmp: Point = Point::new(iter.next().unwrap(), iter.next().unwrap());
            tmp
        })
        .collect();
    (n, w, k, _c, wsrc, house)
}

fn main() {
    let (n, w, k, _c, wsrc, house) = input();
    let in_range = |x: i32, y: i32| -> bool { x >= 0 && x < n as i32 && y >= 0 && y < n as i32 };

    let mut is_broken = vec![vec![false; n as usize]; n as usize];

    let mut pq = BinaryHeap::<Reverse<(i32, u32, u32)>>::new();
    for i in 0..k {
        let mut nearest = 0u32;
        for j in 1..w {
            let dist = house[i as usize].dist(&wsrc[j as usize]);
            if house[i as usize].dist(&wsrc[nearest as usize]) > dist {
                nearest = j;
            }
        }
        pq.push(Reverse((
            house[i as usize].dist(&wsrc[nearest as usize]),
            i,
            nearest,
        )));
    }

    while let Some(Reverse((_, i, nearest_idx))) = pq.pop() {
        let nearest = if nearest_idx < w {
            wsrc[nearest_idx as usize]
        } else {
            house[(nearest_idx - w) as usize]
        };

        let mut x = house[i as usize].x;
        let mut y = house[i as usize].y;
        if is_broken[x as usize][y as usize] {
            continue;
        }
        let mut skip = false;

        while x != nearest.x && !skip {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, POWER);
            }
            for d in 0..2 {
                let ny = y + 1 - (2 * d);
                if in_range(x, ny) && is_broken[x as usize][ny as usize] {
                    skip = true;
                    break;
                }
            }
            x += if x < nearest.x { 1 } else { -1 };
        }
        while y != nearest.y && !skip {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, POWER);
            }
            y += if y < nearest.y { 1 } else { -1 };
            for d in 0..2 {
                let nx = x + 1 - (2 * d);
                if in_range(nx, y) && is_broken[nx as usize][y as usize] {
                    skip = true;
                    break;
                }
            }
        }
        if !is_broken[x as usize][y as usize] && !skip {
            is_broken[x as usize][y as usize] = query(x, y, POWER);
        }

        for j in 0..k {
            if i == j {
                continue;
            }
            let h = house[j as usize];
            if is_broken[h.x as usize][h.y as usize] {
                continue;
            }
            let dist = house[i as usize].dist(&house[j as usize]);
            pq.push(Reverse((dist, j, w + i)));
        }
    }
    assert!(false);
}

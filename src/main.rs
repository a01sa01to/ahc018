use std::{
    io::{self, Write},
    process,
};

const MAX_POWER: u32 = 5000;

fn query(x: i32, y: i32, p: u32) -> bool {
    println!("{} {} {}", x, y, p);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let res = input.trim().parse::<i32>().unwrap();
    if res == -1 || res == 2 {
        process::exit(0);
    }
    res == 1
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
    let mut is_broken = vec![vec![false; n as usize]; n as usize];

    let mut p = (0..k).collect::<Vec<u32>>();
    p.sort_by(|a, b| {
        let neara = (0..w as usize)
            .map(|i| house[*a as usize].dist(&wsrc[i]))
            .min()
            .unwrap();
        let nearb = (0..w as usize)
            .map(|i| house[*b as usize].dist(&wsrc[i]))
            .min()
            .unwrap();
        neara.cmp(&nearb)
    });

    // まず縦いって横行く
    for _i in 0..k {
        let i = p[_i as usize] as usize;
        eprintln!("{}: {} {}", i, house[i].x, house[i].y);
        let mut nearest = wsrc[0];
        for j in 1..(w + _i) as usize {
            if j >= w as usize {
                let dist = house[i].dist(&house[p[j - w as usize] as usize]);
                if house[i].dist(&nearest) > dist {
                    nearest = house[p[j - w as usize] as usize];
                }
            } else {
                let dist = house[i].dist(&wsrc[j]);
                if house[i].dist(&nearest) > dist {
                    nearest = wsrc[j];
                }
            }
        }

        let mut x = house[i].x;
        let mut y = house[i].y;
        while x != nearest.x {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
            }
            x += if x < nearest.x { 1 } else { -1 };
        }
        while y != nearest.y {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
            }
            y += if y < nearest.y { 1 } else { -1 };
        }
        if !is_broken[x as usize][y as usize] {
            is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
        }
    }
    assert_eq!(true, false);
}

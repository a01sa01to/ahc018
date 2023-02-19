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
    let mut dist = vec![1e9 as i32; k as usize];
    let mut wsrc4house = vec![!0; k as usize];
    for i in 0..k as usize {
        for j in 0..w as usize {
            let tmpdist = (house[i].0 - wsrc[j].0).abs() + (house[i].1 - wsrc[j].1).abs();
            if dist[i] > tmpdist {
                dist[i] = tmpdist;
                wsrc4house[i] = j as u32;
            }
        }
    }
    let mut is_broken = vec![vec![false; n as usize]; n as usize];
    // まず縦いって横行く
    for i in 0..k as usize {
        let mut x = house[i].0;
        let mut y = house[i].1;
        let j = wsrc4house[i];
        eprintln!("doing house{}: ({}, {}), wsrc:{}", i, x, y, j);
        while x != wsrc[j as usize].0 {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
            }
            if x < wsrc[j as usize].0 {
                x += 1;
            } else {
                x -= 1;
            }
        }
        eprintln!("x done");
        while y != wsrc[j as usize].1 {
            if !is_broken[x as usize][y as usize] {
                is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
            }
            if y < wsrc[j as usize].1 {
                y += 1;
            } else {
                y -= 1;
            }
        }
        eprintln!("y done");
        if !is_broken[x as usize][y as usize] {
            is_broken[x as usize][y as usize] = query(x, y, MAX_POWER);
        }
    }
    assert_eq!(true, false);
}

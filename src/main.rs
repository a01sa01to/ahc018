extern crate rand;

use rand::Rng;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::{Debug, Formatter},
    io::{self, Write},
    process,
};

// ---------- Enum, Struct ---------- //
// 方向管理
#[derive(Clone, Copy, PartialEq)]
enum Dir {
    None = -1,
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}
// 岩盤の状態管理
#[derive(Clone, Copy, PartialEq)]
enum RockState {
    None,
    Broken,
    NotBroken,
    Flowing,
}
// 座標管理
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

// ---------- Implementation ---------- //
impl Point {
    // コンストラクタ
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    // マンハッタン距離
    #[allow(dead_code)]
    fn mdist(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    // ユークリッド距離
    fn edist(&self, other: &Point) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as i64
    }
}
// println用
impl Debug for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::None => write!(f, "None"),
            Dir::Up => write!(f, "Up"),
            Dir::Left => write!(f, "Left"),
            Dir::Down => write!(f, "Down"),
            Dir::Right => write!(f, "Right"),
        }
    }
}

// ---------- Constants ---------- //
const DX: [i32; 4] = [-1, 0, 1, 0];
const DY: [i32; 4] = [0, -1, 0, 1];
const DX_8: [i32; 8] = [0, 1, 1, 1, 0, -1, -1, -1];
const DY_8: [i32; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const N: u32 = 200;

// ヤー！パワー！！
//
// 　　　　ノ从从从ヽ
// 　　　 /ミミ 彡彡∧
// 　　　｜／￣￣￣＼｜
// 　　　 Y　==　==　Y
// 　　　(|　ヽ〉ノ　|)
// 　　　 ( 　 |　　 )
// 　　　 ｜ ﾉヽノヽ｜
// 　　　／＼((￣)/／
// 　　／ ＼ ヽ二ノ＼
// 　／⌒＼ ＼　　 | ヽ
// `/　　　ヽ ＼＿ノ　|
// ｜　　　 |　　　_　|
// ｜　　　ノ　　 へヽ|
// ｜　　　)ノ　 / 三|
// ｜　　／￣￣￣　_ノ
// 人　　　　　　／
// 　＼＿＿＿＿／
//
const POWER: u32 = 128;

const DFS_WIDTH: i32 = 12; // DFSで探索する間隔幅
const BREAK_AC_INIT: i32 = 3; // 1探索で壊す上限回数
const NEAR_AC: i32 = 15; // 目標地点の近づき度
const FLOWING_AC: i32 = 12; // 流れている岩盤の近づき度
const BROKEN_AC: i32 = 8; // 近くに壊されている岩盤があった時の許容度

// ---------- Functions ---------- //
fn in_range(x: i32, y: i32) -> bool {
    x >= 0 && x < N as i32 && y >= 0 && y < N as i32
}

fn query(x: i32, y: i32, p: u32, bedrock: &mut Vec<Vec<(RockState, i32)>>) -> RockState {
    // クエリを投げなくていいとき
    if bedrock[x as usize][y as usize].0 == RockState::Broken {
        return RockState::Broken;
    }
    if bedrock[x as usize][y as usize].0 == RockState::Flowing {
        return RockState::Flowing;
    }
    // クエリ投げ
    println!("{} {} {}", x, y, p);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let res = input.trim().parse::<i32>().unwrap();
    // 回数管理
    bedrock[x as usize][y as usize].1 += 1;
    // 終わるべき
    if res == -1 || res == 2 {
        process::exit(0);
    }
    // 状態
    if res == 1 {
        bedrock[x as usize][y as usize].0 = RockState::Broken
    } else {
        bedrock[x as usize][y as usize].0 = RockState::NotBroken
    }
    // return
    bedrock[x as usize][y as usize].0
}

fn query_until_broken(
    x: i32,
    y: i32,
    p: u32,
    bedrock: &mut Vec<Vec<(RockState, i32)>>,
    nxt_state: RockState,
) {
    // 再帰する
    let res = query(x, y, p, bedrock);
    if res != RockState::Broken && res != RockState::Flowing {
        query_until_broken(x, y, p, bedrock, nxt_state);
    } else {
        bedrock[x as usize][y as usize].0 = nxt_state;
    }
}

fn connect_greedy(
    src: Point,
    target: Point,
    bedrock: &mut Vec<Vec<(RockState, i32)>>,
    nxt_state: RockState,
) {
    // 一直線に src から target まで掘る
    let mut x = src.x;
    let mut y = src.y;
    while x != target.x || y != target.y {
        if (x - target.x).abs() > (y - target.y).abs() {
            if x < target.x {
                query_until_broken(x, y, POWER, bedrock, nxt_state);
                x += 1;
            } else {
                query_until_broken(x, y, POWER, bedrock, nxt_state);
                x -= 1;
            }
        } else {
            if y < target.y {
                query_until_broken(x, y, POWER, bedrock, nxt_state);
                y += 1;
            } else {
                query_until_broken(x, y, POWER, bedrock, nxt_state);
                y -= 1;
            }
        }
    }
    // target が壊れてない場合もあるので
    query_until_broken(target.x, target.y, POWER, bedrock, nxt_state);
}

fn connect_bfs(src: Point, target: Point, bedrock: &mut Vec<Vec<(RockState, i32)>>) {
    // ダイクストラでいい感じにつなげる
    let mut que = BinaryHeap::<Reverse<(i32, Point)>>::new();
    let mut dist = vec![vec![(1e9 as i32, Point::new(-1, -1)); N as usize]; N as usize];
    que.push(Reverse((0, src)));
    dist[src.x as usize][src.y as usize].0 = 0;
    while let Some(Reverse((dst, now))) = que.pop() {
        if now.x == target.x && now.y == target.y {
            break;
        }
        if dst > dist[now.x as usize][now.y as usize].0 {
            continue;
        }
        for i in 0..8 {
            let nx = now.x + DFS_WIDTH * DX_8[i];
            let ny = now.y + DFS_WIDTH * DY_8[i];
            if !in_range(nx, ny) {
                continue;
            }
            // 斜めは倍率を高めに設定
            let mag = if DX_8[i].abs() + DY_8[i].abs() > 1 {
                3
            } else {
                1
            };
            let ndst = dst + (bedrock[nx as usize][ny as usize].1) * mag;
            // 壊れてないなら飛ばす
            if bedrock[nx as usize][ny as usize].0 != RockState::Broken
                && bedrock[nx as usize][ny as usize].0 != RockState::Flowing
            {
                continue;
            }
            if ndst < dist[nx as usize][ny as usize].0 {
                dist[nx as usize][ny as usize].0 = ndst;
                dist[nx as usize][ny as usize].1 = now;
                que.push(Reverse((ndst, Point::new(nx, ny))));
            }
        }
    }
    // つなげる
    let mut now = target;
    while now.x != src.x || now.y != src.y {
        let prev = dist[now.x as usize][now.y as usize].1;
        if prev.x == -1 {
            break;
        }
        connect_greedy(prev, now, bedrock, RockState::Flowing);
        now = prev;
    }
}

fn dfs(
    now: Point,
    prev_dir: Dir,
    src: Point,
    target: Point,
    break_ac: i32,
    wsrc: &Vec<Point>,
    house: &Vec<Point>,
    bedrock: &mut Vec<Vec<(RockState, i32)>>,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    let mut rng = rand::thread_rng();

    visited[now.x as usize][now.y as usize] = true;
    // すでに流れている場合はそこからつなげられる
    if bedrock[now.x as usize][now.y as usize].0 == RockState::Flowing {
        connect_bfs(src, now, bedrock);
        println!(
            "# Done: from:({},{}) to:({},{})",
            src.x, src.y, now.x, now.y
        );
        return true;
    }
    // 近くまで行ったらつなげる
    if (now.x - target.x).abs() <= NEAR_AC && (now.y - target.y).abs() <= NEAR_AC {
        connect_greedy(now, target, bedrock, RockState::Flowing);
        connect_bfs(src, now, bedrock);
        return true;
    }
    // 近くに水路があればそこからつなげる
    for di in 0..FLOWING_AC {
        for dx in -FLOWING_AC..FLOWING_AC {
            for dy in -FLOWING_AC..FLOWING_AC {
                if dx.abs() + dy.abs() > di {
                    continue;
                }
                let nx = now.x + dx;
                let ny = now.y + dy;
                if in_range(nx, ny) && bedrock[nx as usize][ny as usize].0 == RockState::Flowing {
                    connect_greedy(now, Point::new(nx, ny), bedrock, RockState::Flowing);
                    connect_bfs(src, now, bedrock);
                    return true;
                }
            }
        }
    }
    // 目標地点から離れすぎる場合はやめる
    let rnddst = ((src.edist(&target) as f64).sqrt() - (now.edist(&target) as f64).sqrt()) / 4.0;
    println!("# Source: ({}, {}), Target: ({}, {}), Now: ({}, {}), Dist now: {}, Dist target: {}, rnddst: {}, Exp: {}", src.x, src.y, target.x, target.y, now.x, now.y, now.edist(&target), src.edist(&target), rnddst, rnddst.exp());
    if rnddst.exp() < rng.gen::<f64>() {
        return false;
    }
    // 方向の優先順位決め
    let priority_dir: [Dir; 4];
    if (now.x - target.x).abs() > (now.y - target.y).abs() {
        if now.x > target.x {
            if now.y > target.y {
                priority_dir = [Dir::Up, Dir::Left, Dir::Right, Dir::Down];
            } else {
                priority_dir = [Dir::Up, Dir::Right, Dir::Left, Dir::Down];
            }
        } else {
            if now.y > target.y {
                priority_dir = [Dir::Down, Dir::Left, Dir::Right, Dir::Up];
            } else {
                priority_dir = [Dir::Down, Dir::Right, Dir::Left, Dir::Up];
            }
        }
    } else {
        if now.y > target.y {
            if now.x > target.x {
                priority_dir = [Dir::Left, Dir::Up, Dir::Down, Dir::Right];
            } else {
                priority_dir = [Dir::Left, Dir::Down, Dir::Up, Dir::Right];
            }
        } else {
            if now.x > target.x {
                priority_dir = [Dir::Right, Dir::Up, Dir::Down, Dir::Left];
            } else {
                priority_dir = [Dir::Right, Dir::Down, Dir::Up, Dir::Left];
            }
        }
    }
    println!(
        "# Source: ({}, {}), Now: ({}, {}), Target: ({}, {}), Priority: [{:?}, {:?}, {:?}, {:?}]",
        src.x,
        src.y,
        now.x,
        now.y,
        target.x,
        target.y,
        priority_dir[0],
        priority_dir[1],
        priority_dir[2],
        priority_dir[3]
    );

    // 今いる場所から一番近い水源を探す
    let mut new_target = wsrc[0];
    for i in 1..wsrc.len() {
        if now.edist(&wsrc[i]) < now.edist(&new_target) {
            new_target = wsrc[i];
        }
    }
    for i in 0..house.len() {
        let h = house[i];
        if bedrock[h.x as usize][h.y as usize].0 == RockState::Flowing {
            if now.edist(&h) < now.edist(&new_target) {
                new_target = h;
            }
        }
    }
    println!(
        "# Prev Target: ({}, {}), New Target: ({}, {})",
        target.x, target.y, new_target.x, new_target.y
    );

    // DFS
    for dir in 0..4 {
        let nxt_dir = priority_dir[dir];
        // 戻る方向にはいかない
        if nxt_dir as i32 % 2 == prev_dir as i32 % 2 && nxt_dir != prev_dir {
            continue;
        }
        let nx = now.x + DFS_WIDTH * DX[nxt_dir as usize];
        let ny = now.y + DFS_WIDTH * DY[nxt_dir as usize];
        if in_range(nx, ny) {
            if visited[nx as usize][ny as usize] {
                continue;
            }
            let mut is_broken_tmp = bedrock[nx as usize][ny as usize].0 == RockState::Broken
                || bedrock[nx as usize][ny as usize].0 == RockState::Flowing;
            // 1回の上限を決めて掘る
            for _ in 0..break_ac {
                let res = query(nx, ny, POWER, bedrock);
                if res == RockState::Broken {
                    is_broken_tmp = true;
                    break;
                }
            }
            // もし壊れたら先に進む
            if is_broken_tmp {
                // すでに近くに壊れた岩盤があるなら、そこにつなげる
                for di in 0..BROKEN_AC {
                    for dx in -BROKEN_AC..BROKEN_AC {
                        for dy in -BROKEN_AC..BROKEN_AC {
                            if dx.abs() + dy.abs() == 0 {
                                continue;
                            }
                            if dx.abs() + dy.abs() > di {
                                continue;
                            }
                            let nnx = nx + dx;
                            let nny = ny + dy;
                            if in_range(nnx, nny) {
                                if bedrock[nnx as usize][nny as usize].0 == RockState::Broken {
                                    let new_now = Point::new(nx, ny);
                                    let new_src = Point::new(nnx, nny);
                                    // すでに壊れている場所のみをたどって行けるならつなげる
                                    if dfs(
                                        new_src,
                                        Dir::None,
                                        new_src,
                                        target,
                                        0,
                                        wsrc,
                                        house,
                                        bedrock,
                                        visited,
                                    ) {
                                        println!(
                                            "# Connect: ({}, {}) -> ({}, {})",
                                            new_now.x, new_now.y, new_src.x, new_src.y
                                        );
                                        connect_greedy(
                                            new_now,
                                            new_src,
                                            bedrock,
                                            RockState::Flowing,
                                        );
                                        connect_bfs(src, new_now, bedrock);
                                        return true;
                                    } else {
                                        println!(
                                            "# Cannot connect: ({}, {}) -> ({}, {})",
                                            new_now.x, new_now.y, new_src.x, new_src.y
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                let connected = dfs(
                    Point::new(nx, ny),
                    nxt_dir,
                    src,
                    new_target,
                    break_ac,
                    wsrc,
                    house,
                    bedrock,
                    visited,
                );
                // つながったらtrue
                if connected {
                    return true;
                }
            }
        }
    }
    false
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
    // Constants Assertion
    assert!(
        DFS_WIDTH > BROKEN_AC,
        "常に繋げかねないので、DFS_WIDTH > BROKEN_AC"
    );
    assert!(
        FLOWING_AC > BROKEN_AC,
        "処理が変になるので、FLOWING_AC > BROKEN_AC"
    );

    let (n, w, k, _c, wsrc, house) = input();

    // (RockState, i32): (岩盤状態, 何回掘ったか)
    let mut bedrock = vec![vec![(RockState::None, 0); n as usize]; n as usize];

    // 近い順にソート
    let mut pq = BinaryHeap::<Reverse<(i64, u32, u32)>>::new();
    for i in 0..k {
        let mut nearest = 0u32;
        for j in 1..w {
            let dist = house[i as usize].edist(&wsrc[j as usize]);
            if house[i as usize].edist(&wsrc[nearest as usize]) > dist {
                nearest = j;
            }
        }
        pq.push(Reverse((
            house[i as usize].edist(&wsrc[nearest as usize]),
            i,
            nearest,
        )));
    }

    while let Some(Reverse((_, i, nearest_idx))) = pq.pop() {
        let h = house[i as usize];
        // すでに流れている
        if bedrock[h.x as usize][h.y as usize].0 == RockState::Flowing {
            continue;
        }
        // 初期目標地点は一番近いところ
        let nearest = if nearest_idx < w {
            wsrc[nearest_idx as usize]
        } else {
            house[(nearest_idx - w) as usize]
        };
        // すでに壊れている場合はつなぐ
        if bedrock[h.x as usize][h.y as usize].0 == RockState::Broken {
            connect_bfs(h, nearest, &mut bedrock);
            continue;
        }
        // DFS
        let mut visited = vec![vec![false; n as usize]; n as usize];
        while !dfs(
            h,
            Dir::None,
            h,
            nearest,
            BREAK_AC_INIT,
            &wsrc,
            &house,
            &mut bedrock,
            &mut visited,
        ) {
            for i in 0..n {
                for j in 0..n {
                    visited[i as usize][j as usize] = false;
                }
            }
        }

        // 新しい距離を追加する
        for j in 0..k {
            if i == j {
                continue;
            }
            let dist = house[i as usize].edist(&house[j as usize]);
            pq.push(Reverse((dist, j, w + i)));
        }
    }
    assert!(false, "全部つながってないよ！");
}

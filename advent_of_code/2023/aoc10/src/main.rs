use std::io::{stdin, Read};
use std::collections::VecDeque;
use je::point::Point;

type Grid = Vec<Vec<char>>;

fn parse(buf: &str) -> Grid {
    let mut out = vec![];
    for l in buf.split('\n') {
        out.push(l.chars().collect());
    }
    out
}

fn find(g: &Grid, v: char) -> Point {
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            if g[y][x] == v {
                return Point::new(x as i32, y as i32);
            }
        }
    }
    unreachable!();
}

fn get_tile(g: &Grid, p: &Point) -> char {
    let x = p.x as usize;
    let y = p.y as usize;
    g[y][x]
}

fn in_bounds(g: &Grid, p: &Point) -> bool {
    let x = p.x as usize;
    let y = p.y as usize;
    y < g.len() && x < g[0].len()
}

fn get_relative(g: &Grid, p: &Point, off: &Point) -> Option<Point> {
    let tgt = *p + *off;
    if !in_bounds(&g, &tgt) {
        return None;
    }
    Some(tgt)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn from_dir(d: Dir) -> Point {
    match d {
        Dir::Up => return Point::new(0, -1),
        Dir::Down => return Point::new(0, 1),
        Dir::Left => return Point::new(-1, 0),
        Dir::Right => return Point::new(1, 0),
    }
}

fn get_openings(c: char) -> Vec<Dir> {
    let mut out = vec![];
    match c {
        'S' => {
            out.push(Dir::Up);
            out.push(Dir::Down);
            out.push(Dir::Left);
            out.push(Dir::Right);
        },
        '|' => {
            out.push(Dir::Up);
            out.push(Dir::Down);
        },
        '-' => {
            out.push(Dir::Left);
            out.push(Dir::Right);
        },
        'L' => {
            out.push(Dir::Up);
            out.push(Dir::Right);
        },
        'J' => {
            out.push(Dir::Up);
            out.push(Dir::Left);
        },
        '7' => {
            out.push(Dir::Left);
            out.push(Dir::Down);
        },
        'F' => {
            out.push(Dir::Right);
            out.push(Dir::Down);
        },
        _ => {}
    }
    out
}

fn can_connect(g: &Grid, p: &Point, d: Dir) -> bool {
    let src_tile = get_tile(&g, p);
    let src_dirs = get_openings(src_tile);
    if !src_dirs.contains(&d) {
        return false;
    }

    let dst = *p + from_dir(d);
    if !in_bounds(&g, &dst) {
        return false;
    }

    let dst_tile = get_tile(&g, &dst);
    let dst_dirs = get_openings(dst_tile);
    let match_dir : Dir;
    match d {
        Dir::Up => match_dir = Dir::Down,
        Dir::Down => match_dir = Dir::Up,
        Dir::Left => match_dir = Dir::Right,
        Dir::Right => match_dir = Dir::Left,
    }
    dst_dirs.contains(&match_dir)
}

fn get_connected(g: &Grid, p: &Point) -> Vec<Point> {
    let mut out = vec![];
    let opens = get_openings(get_tile(g, p));
    for o in &opens {
        if can_connect(&g, p, *o) {
            let dst = *p + from_dir(*o);
            // println!("{:?} {:?} {:?}", p, o, dst);
            out.push(dst);
        }
    }
    out
}

fn part1(buf: &str) -> u32 {
    let g = parse(buf);
    let mut q : VecDeque<(Point, u32)> = VecDeque::new();
    let begin = find(&g, 'S');
    let mut out = 0;
    q.push_back((begin, 0));

    let mut seen : Vec<Point> = vec![];
    while q.len() > 0 {
        let (p, dist) = q.pop_front().unwrap();
        if seen.contains(&p) {
            continue;
        }
        let next_dist = dist + 1;
        // println!("see:{}@{:?} dst:{}", get_tile(&g, &p), &p, next_dist);
        seen.push(p);

        for np in get_connected(&g, &p) {
            q.push_back((np, next_dist));
            out = std::cmp::max(out, next_dist - 1);
        }
    }
    out
}

fn regions(g: &Grid) -> Vec<Vec<Point>> {
    let mut out = vec![];
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            let cur = Point::new(x, y);
            if seen.contains(cur) {
                continue;
            }
            seen.insert(cur);

            let con = get_connected(&g, &cur);

        }
    }
}

fn enclose(g: &Grid) -> u32 {
    let mut seen = vec![];
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            let cur = Point::new(x, y);
            if seen.contains(cur) {
                continue;
            }
            seen.insert(cur);

            let con = get_connected(&g, &cur);

        }
    }
    todo!();
}

fn part2(buf: &str) -> u32 {
    0 
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....".to_string();
        assert_eq!(part1(&input), 4);

        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...".to_string();
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn connect() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....".to_string();
        let g = parse(&input);
        let c1 = get_connected(&g, &Point::new(1, 1));
        assert_eq!(c1.len(), 2);
        for c in &c1 {
            let cn = get_connected(&g, c);
        }
    }

    #[test]
    fn example2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".to_string();
        assert_eq!(enclose(&input), 4);
    }
}

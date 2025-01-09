use std::io::{stdin, Read};
use std::collections::{ HashSet, VecDeque };
use util::Point;

type Grid = Vec<Vec<char>>;
type Points = Vec<Point>;

fn grid_new(len: usize) -> Grid {
    let mut out = vec![];
    for _y in 0..len {
        let mut row = vec![];
        for _x in 0..len {
            row.push('.');
        }
        out.push(row);
    }
    out
}

fn grid_corrupt(g: &mut Grid, pts: &Points, lim: usize) {
    for (idx,pt) in pts.iter().enumerate() {
        if idx >= lim {
            break;
        }
        g[pt.y as usize][pt.x as usize] = 'X';
    }
}

fn grid_get(g: &Grid, pt: Point) -> Option<char> {
    if pt.x < 0 || pt.y < 0
        || pt.y as usize >= g.len()
        || pt.x as usize >= g[0].len() {
            return None;
    }
    Some(g[pt.y as usize][pt.x as usize])
}

fn grid_nbrs(g: &Grid, pt: Point) -> Vec<(Point, char)> {
    let mut out = vec![];
    for dir in vec![Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1)] {
        let nbr = pt + dir;
        if let Some(val) = grid_get(&g, nbr) {
            out.push((nbr, val));
        }
    }
    out
}

fn grid_search(g: &Grid, start: Point, goal: Point) -> Option<u32> {
    let mut frontier : VecDeque<(Point, u32)> = VecDeque::new();
    let mut seen : HashSet<Point> = HashSet::new();
    frontier.push_back((start, 0));
    while frontier.len() > 0 {
        let (pt, steps) = frontier.pop_front().unwrap();
        if !seen.insert(pt) {
            continue;
        }

        let nbrs = grid_nbrs(g, pt);
        for (npt, nval) in nbrs {
            if npt == goal {
                return Some(steps + 1);
            }
            match nval {
                '.' => {
                    frontier.push_back((npt, steps + 1));
                }
                _ => {},
            }
        }
    }
    None
}

fn from_str(s: &str) -> Points {
    let mut out = vec![];
    for l in s.split("\n") {
        let toks : Vec<&str> = l.split(',').collect();
        if toks.len() != 2 {
            continue;
        }
        //  2,0
        let x = toks[0].parse::<i32>().unwrap();
        let y = toks[1].parse::<i32>().unwrap();
        out.push(Point::new(x, y));
    }
    out
}

fn part1(buf: &str, len: usize) -> u32 {
    let points = from_str(buf);
    let mut grid = grid_new(len);
    grid_corrupt(&mut grid, &points, 1024);
    let start = Point::new(0, 0);
    let pos = len as i32 - 1;
    let goal = Point::new(pos, pos);
    grid_search(&grid, start, goal).unwrap()
}

fn part2(buf: &str, len: usize) -> Point {
    let points = from_str(buf);
    let start = Point::new(0, 0);
    let pos = len as i32 - 1;
    let goal = Point::new(pos, pos);
    let grid = grid_new(len);
    let mut li : usize = 0;
    let mut ri : usize = points.len() - 1;
    let mut ngrid;
    let mut last = Point::new(-1, -1);
    while li < ri {
        println!("{} {}", li, ri);
        let mid = (li + ri) / 2;
        ngrid = grid.clone();
        grid_corrupt(&mut ngrid, &points, mid);
        match grid_search(&ngrid, start, goal) {
            Some(_) => {
                li = mid + 1;
            },
            None  => {
                last = points[mid];
                ri = mid - 1;
            },
        }
    }
    last
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf, 71));
    println!("part2: {:?}", part2(&buf, 71));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1()
    {
        let s = "
5,4
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
5,1";
        assert_eq![part1(s, 7), 22];
    }

    #[test]
    fn example2()
    {
        let s = "
5,4
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
5,1,
6,0";
        assert_eq![part2(s, 7), Point::new(6, 0)];
    }
}

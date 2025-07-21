use std::io::{stdin, Read};
use util::{Grid, Point};
use std::collections::HashSet;

// using a C-style, struct + functions taking that struct approach
// just for grins
// although you do sort of feel the "freebie self" parameter missing
// as an ergonomic thing at callsites...

type Grid = Vec<Vec<u32>>;
fn from_buf(s: &str) -> Grid {
    let mut out = vec![];
    for l in s.split('\n') {
        let mut row = vec![];
        for c in l.chars() {
            row.push(char::to_digit(c, 10).unwrap_or(u32::MAX));
        }
        if row.len() == 0 {
            continue;
        }
        out.push(row);
    }
    out
}

fn get(g: &Grid, p: Point) -> Option<u32> {
    if p.x < 0 || p.y < 0 
            || p.x as usize >= g[0].len()
            || p.y as usize >= g.len() {
        return None;
    }
    Some(g[p.y as usize][p.x as usize])
}

fn get_nbr(g: &Grid, p: Point, target: u32) -> Vec<Point> {
    let dirs : [Point; 4] = [
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1)];
    let mut out = vec![];
    for d in dirs {
        let np = p + d;
        if let Some(val) = get(g, np) {
            if val == target {
                out.push(np);
            }
        }
    }
    out
}

fn explore_v1(g: &Grid, start: Point, goal: u32) -> u32 {
    let mut frontier = vec![start];
    let mut target = get(g, start).unwrap();
    assert!(target == 0);

    while target < goal {
        target += 1;
        // set here because we only care about how many unique
        // goal-nodes we hit in this bfs
        let mut nbrs : HashSet<Point> = HashSet::new();
        for cur in &frontier {
            for nn in get_nbr(g, *cur, target) {
                nbrs.insert(nn);
            }
        }
        frontier.clear();

        if target == goal {
            return nbrs.len() as u32;
        }
        for nbr in &nbrs {
            frontier.push(*nbr);
        }
    }
    unreachable!();
}

fn explore_v2(g: &Grid, start: Point, goal: u32) -> u32 {
    let mut frontier = vec![start];
    let mut target = get(g, start).unwrap();
    assert!(target == 0);

    while target < goal {
        target += 1;
        // vector of neighbors here
        // we actually want to count the total number of paths,
        // not just the number of unique end nodes we hit
        let mut nbrs = vec![];
        for cur in &frontier {
            for nn in get_nbr(g, *cur, target) {
                nbrs.push(nn);
            }
        }
        frontier.clear();

        if target == goal {
            return nbrs.len() as u32;
        }
        for nbr in &nbrs {
            frontier.push(*nbr);
        }
    }
    unreachable!();
}

fn get_starts(g: &Grid) -> Vec<Point> {
    let mut out : Vec<Point> = vec![];
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            let p = Point::new(x as i32, y as i32);
            if get(&g, p).unwrap() == 0 {
                out.push(p);
            }
        }
    }
    out
}

fn part1(buf: &str) -> u32 {
    let g = from_buf(buf);
    let starts = get_starts(&g);

    let mut out = 0;
    for h in starts {
        out += explore_v1(&g, h, 9);
    }
    out
}

fn part2(buf: &str) -> u32 {
    let g = from_buf(buf);
    let starts = get_starts(&g);

    let mut out = 0;
    for h in starts {
        out += explore_v2(&g, h, 9);
    }
    out
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
    fn simple() {
        let s = "...0...
...1...
...2...
6543...
7......
8......
9......";
        assert_eq![part1(s), 1];
    }
    #[test]
    fn fork() {
        let s = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        assert_eq![part1(s), 2];
    }

    #[test]
    fn example1() {
        let s = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        assert_eq![part1(s), 4];
    }

    #[test]
    fn mid() {
        let s = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        assert_eq![part1(s), 3];
    }

    #[test]
    fn example2() {
        let s = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq![part2(s), 81];
    }
}

use std::cmp;
use std::io::{stdin, Read};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x: x, y: y }
    }

    fn from_str(s: &str) -> Option<Self> {
        let toks: Vec<&str> = s.split(',').collect();
        if toks.len() != 2 {
            return None;
        }
        Some(Self {
            x: toks[0].parse::<usize>().unwrap(),
            y: toks[1].parse::<usize>().unwrap(),
        })
    }
}

#[derive(Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
    // TODO:  these cannot possibly be right?  what if you need multiple
    // iterators over the same struct at the same time?
    cur: Point,
    has_next: bool,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line {
            p1: p1,
            p2: p2,
            cur: p1,
            has_next: true,
        }
    }
}

impl Iterator for Line {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next {
            return None;
        }

        // awful hack around my lack of expertise on the borrow checker
        let tmp = self.cur.clone();

        // move from the last position towards p2
        let &mut pos;
        let &dest;
        if self.p1.x == self.p2.x {
            pos = &mut self.cur.y;
            dest = &self.p2.y;
        } else {
            pos = &mut self.cur.x;
            dest = &self.p2.x;
        }

        let out;
        if *pos == *dest {
            out = None;
        } else {
            out = Some(tmp);
            if *pos < *dest {
                *pos += 1;
            } else {
                *pos -= 1;
            }
        }
        out
    }
}

fn parse(buf: &str) -> Vec<Line> {
    let mut out = vec![];
    for l in buf.split('\n') {
        let toks: Vec<&str> = l.split(" -> ").collect();
        if toks.len() < 2 {
            continue;
        }
        let p1 = Point::from_str(toks[0]).unwrap();
        let p2 = Point::from_str(toks[1]).unwrap();
        out.push(Line::new(p1, p2));
    }
    out
}

fn part1(lines: Vec<Line>) -> usize {
    let mut max = Point::new(0, 0);
    for line in &lines {
        max.x = cmp::max(max.x, line.p1.x);
        max.x = cmp::max(max.x, line.p2.x);
        max.y = cmp::max(max.y, line.p1.y);
        max.y = cmp::max(max.y, line.p2.y);
    }

    let mut grid = vec![];
    for y in 0..max.y + 1 {
        grid.push(vec![0; max.x as usize + 1]);
    }
    let mut out = 0;
    for line in lines {
        // only horizontal/vertical lines
        if line.p1.x == line.p2.x || line.p1.y == line.p2.y {
            for point in line {
                grid[point.y as usize][point.x as usize] += 1;
            }
        }
    }

    for y in 0..grid.len() {
        println!("{:?}", grid[y]);
        for x in 0..grid[y].len() {
            if grid[y][x] > 1 {
                out += 1;
            }
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = parse(&buf);

    println!("part1: {}", part1(lines.clone()));
    // println!("part2: {}", part2(game.clone()));
}

mod test {
    use super::*;

    #[test]
    fn test_line_iterate() {
        let lim = 5;
        let p1 = Point::new(0, 0);
        let p2 = Point::new(lim, 0);
        let mut line = Line::new(p1, p2);

        let mut points = vec![];
        while let Some(p) = line.next() {
            points.push(p);
        }

        let mut expect = vec![];
        for i in 0..lim+1 {
            expect.push(Point::new(i, 0));
        }
        assert_eq!(points, expect);
    }

    #[test]
    fn test_line_iterate_y_reverse() {
        let lim = 4;
        let p1 = Point::new(0, lim);
        let p2 = Point::new(0, 0);
        let mut line = Line::new(p1, p2);

        let mut points = vec![];
        while let Some(p) = line.next() {
            points.push(p);
        }

        let mut expect = vec![];
        for i in 0..lim+1 {
            expect.push(Point::new(0, lim - i));
        }
        assert_eq!(points, expect);
    }
}

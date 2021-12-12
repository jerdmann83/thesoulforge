use std::cmp;
use std::io::{stdin, Read};
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }

    fn from_str(s: &str) -> Option<Self> {
        let toks: Vec<&str> = s.split(',').collect();
        if toks.len() != 2 {
            return None;
        }
        Some(Self {
            x: toks[0].parse::<i32>().unwrap(),
            y: toks[1].parse::<i32>().unwrap(),
        })
    }

    fn move_towards(&mut self, dest: &Point) {
        if *self == *dest {
            return;
        }

        let delta;
        if self.x == dest.x {
            let mv = if self.y < dest.y { 1 } else { -1 };
            delta = Point::new(0, mv);
        } else if self.y == dest.y {
            let mv = if self.x < dest.x { 1 } else { -1 };
            delta = Point::new(mv, 0);
        } else {
            let x = if self.x < dest.x { 1 } else { -1 };
            let y = if self.y < dest.y { 1 } else { -1 };
            delta = Point::new(x, y);
        }
        *self += delta;
    }
}

#[derive(Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
    // I'm clearly missing something with dealing with iterator state.
    // This totally falls apart if you need multiple iterators over the
    // same struct at the same time, right?
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
        if self.cur == self.p2 {
            if self.has_next {
                self.has_next = false;
                return Some(self.cur);
            } else {
                return None;
            }
        }

        let tmp = self.cur.clone();
        self.cur.move_towards(&self.p2);
        Some(tmp)
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

#[derive(PartialEq)]
enum Part {
    Part1,
    Part2,
}

fn solve(lines: Vec<Line>, part: Part) -> usize {
    let mut grid = vec![];
    for y in 0..1000 {
        grid.push(vec![0; 1000]);
    }
    let mut out = 0;
    for line in lines {
        let add: bool;
        if line.p1.x == line.p2.x || line.p1.y == line.p2.y {
            add = true;
        } else {
            add = part == Part::Part2;
        }
        if add {
            for point in line {
                grid[point.y as usize][point.x as usize] += 1;
            }
        }
    }

    for y in 0..grid.len() {
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

    println!("part1: {}", solve(lines.clone(), Part::Part1));
    println!("part2: {}", solve(lines.clone(), Part::Part2));
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
        for i in 0..lim + 1 {
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
        for i in 0..lim + 1 {
            expect.push(Point::new(0, lim - i));
        }
        assert_eq!(points, expect);
    }

    #[test]
    fn test_line_iterate_diagonal() {
        let lim = 3;
        let p1 = Point::new(0, 0);
        let p2 = Point::new(lim, lim);
        let mut line = Line::new(p1, p2);

        let mut points = vec![];
        while let Some(p) = line.next() {
            points.push(p);
        }

        let mut expect = vec![];
        for i in 0..lim + 1 {
            expect.push(Point::new(i, i));
        }
        assert_eq!(points, expect);
    }
}

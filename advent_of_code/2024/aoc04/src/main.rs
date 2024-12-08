use std::io::{stdin, Read};
use std::ops;

struct Grid {
    grid: Vec<Vec<char>>,
}
impl Grid {
    fn from_buf(s: &str) -> Grid {
        let mut out = vec![];
        for l in s.split('\n') {
            let mut row = vec![];
            for c in l.chars() {
                row.push(c);
            }
            if row.len() == 0 {
                continue;
            }
            out.push(row);
        }
        Self{grid: out}
    }

    fn contains(&self, pt: Point) -> bool {
        return pt.x >= 0 && pt.y >= 0 
            && pt.y < self.grid.len().try_into().unwrap()
            && pt.x < self.grid[0].len().try_into().unwrap();
    }

    fn get_char(&self, dst: Point) -> Option<char> {
        if !self.contains(dst) {
            return None;
        }
        Some(self.grid[dst.y as usize][dst.x as usize])
    }

    fn get_str(&self, pat: &str, start: Point, dir: Point) -> bool {
        let dst = start + dir;
        if pat.len() == 0 {
            return true;
        } 
        if let Some(chr) = self.get_char(start) {
            if chr != pat.chars().nth(0).unwrap() {
                return false;
            }
            return self.get_str(&pat[1..], 
                                dst,
                                dir);
        }
        false
    }

    fn get_x(&self, pat: &str, start: Point) -> bool {
        // super hack:  special case of 3 means we can cheat and just
        // peek in one tile in each direction
        if pat.len() != 3 {
            unreachable!();
        }
        // continuing the horrible 3-len assumption
        let mid_len = 1;
        let lhs_expect = pat.chars().nth(0).unwrap();
        let rhs_expect = pat.chars().nth(2).unwrap();
        // bounds check
        if start.x < mid_len || start.y < mid_len
            || start.x + mid_len > self.grid[0].len() as i32
            || start.y + mid_len > self.grid.len() as i32 {
                return false;
        }
        // anchor our search based on these two diagonal positions
        // X.X
        // ...
        // ...
        let diags = vec![
            Point::new(-1, 1),
            Point::new(1, 1),
        ];
        let mut hits = 0;
        for d in diags {
            let fd = d.flip();
            let lhs = self.get_char(start + d);
            let rhs = self.get_char(start + fd);
            if lhs.is_none() || rhs.is_none() {
                // no need to check the other diagonals
                // we're dealing with diagonals. there is no way for
                // one pair of them to be in range of a given
                // square-cell and the opposite pair not
                return false;
            }
            let lval = lhs.unwrap();
            let rval = rhs.unwrap();
            if lval == lhs_expect && rval == rhs_expect
            || lval == rhs_expect && rval == lhs_expect {
                hits += 1;
            }
        }
        hits == 2
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }

    fn flip(&self) -> Self {
        Point::new(-self.x, -self.y)
    }
}
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn get_dirs() -> Vec<Point> {
    let mut out = vec![];
    for y in -1..2 {
        for x in -1..2 {
            if x == 0 && y == 0 {
                continue;
            }
            out.push(Point::new(x, y));
        }
    }
    out
}

fn part1(buf: &str) -> u32 {
    let g = Grid::from_buf(buf);
    let gr = &g.grid;
    let dirs = get_dirs();
    let mut out = 0;
    for y in 0..gr.len() {
        for x in 0..gr[y].len() {
            let loc = Point::new(x as i32, y as i32);
            for d in &dirs {
                if g.get_str("XMAS", loc, *d) {
                    out += 1;
                }

            }
        }
    }
    out
}

fn part2(buf: &str) -> u32 {
    let g = Grid::from_buf(buf);
    let gr = &g.grid;
    let mut out = 0;
    for y in 0..gr.len() {
        for x in 0..gr[y].len() {
            let loc = Point::new(x as i32, y as i32);
            if g.get_x("MAS", loc) {
                out += 1;
            }
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    // TODO: test passes, real input value too high
    println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn example2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(input), 9);
    }
}

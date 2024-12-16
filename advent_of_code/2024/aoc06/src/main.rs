use std::io::{stdin, Read};
use std::ops;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
    fn from_dir(d: Dir) -> Self {
        match d {
            Dir::Up => Self::new(0, 1),
            Dir::Down => Self::new(0, -1),
            Dir::Left => Self::new(-1, 0),
            Dir::Right => Self::new(1, 0),
        }
    }
    fn rotate(&mut self, d: Dir) {
        let prev = self.clone();
        match d {
            Dir::Left => {
                *self = Self::new(-prev.y, prev.x);
            }
            Dir::Right => {
                *self = Self::new(prev.y, -prev.x);
            },
            _ => unreachable!(),
        }
    }
}
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    pos: Point,
    dir: Point,
}

#[derive(Clone, Debug)]
struct Map {
    grid: Vec<Vec<char>>,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut grid = vec![];
        for l in s.split('\n') {
            grid.push(l.chars().collect());
        }
        grid.reverse();
        // we need the first row added to be the top of the grid,
        // or the highest y position.  the last added must be 0
        Self{ grid }
    }

    fn contains(&self, p: Point) -> bool {
        p.x > -1 && p.y > -1
                && p.x < self.grid[0].len() as i32
                && p.y < self.grid.len() as i32
    }

    fn peek(&self, p: Point, d: Point) -> Option<Point> {
        let np = p + d;

        if !self.contains(np) {
            return None;
        }
        Some(np)
    }

    fn get(&self, p: Point) -> Option<char> {
        if !self.contains(p) {
            return None;
        }
        let c : char = self.grid[p.y as usize][p.x as usize];
        return Some(c);
    }

    fn find(&self, c: char) -> Option<Point> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == c {
                    return Some(Point::new(x as i32, y as i32));
                }
            }
        }
        None
    }
}

fn part1(buf: &str) -> u32 {
    let m = Map::from_str(buf);
    let pos = m.find('^').unwrap();
    let mut g = Guard{pos, dir: Point::from_dir(Dir::Up)};
    let mut seen : HashSet<Point> = HashSet::new();
    loop {
        seen.insert(g.pos);
        let np = m.peek(g.pos, g.dir);
        // escaped the map?
        if np.is_none() {
            break;
        }
        // otherwise, what tile are we looking at?
        let np = np.unwrap();
        let cur = m.get(np).unwrap();
        match cur {
            '#' => {
                let last = g.dir;
                g.dir.rotate(Dir::Right);

            },
            // TODO: starting guard space is also empty
            // normalize in map create?
            '.' | '^' => {
                // TODO: += operator / trait thing
                g.pos = g.pos + g.dir;
            },
            _ => todo!(),
        }
    }
    seen.len() as u32
}

fn part2(buf: &str) -> u32 {
    todo!();
}


fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    // println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq![part1(input), 41];
    }

    #[test]
    fn example2() {
    }
}

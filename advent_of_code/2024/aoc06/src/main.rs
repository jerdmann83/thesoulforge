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
            // TODO: the upside-down nature of my grid parse means both
            // the up/down directions as well as the left-right rotate are
            // reversed from what might be more intuitive 
            // what's the idiomatic way to do this in 2d games programming?
            Dir::Up => Self::new(0, -1),
            Dir::Down => Self::new(0, 1),
            Dir::Left => Self::new(-1, 0),
            Dir::Right => Self::new(1, 0),
        }
    }
    fn rotate(&mut self, d: Dir) {
        let prev = self.clone();
        match d {
            Dir::Left => {
                *self = Self::new(prev.y, -prev.x);
            }
            Dir::Right => {
                *self = Self::new(-prev.y, prev.x);
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

type Grid = Vec<Vec<char>>;

#[derive(Clone, Debug)]
struct Map {
    pub grid: Grid,
}

const EMPTY : char = '.';
const GUARD : char = '^';
const OBSTACLE : char = '#';
const NEW_OBSTACLE : char = 'O';

fn parse(s: &str) -> (Map, Guard) {
    let mut grid : Grid = vec![];
    for l in s.split('\n') {
        grid.push(l.chars().collect());
    }
    // we need the first row added to be the top of the grid,
    // or the highest y position.  the last added must be 0
    // grid.reverse();

    // now go back and find the guard
    let mut guards = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == GUARD {
                let pos = Point::new(x as i32, y as i32);
                guards.push(Guard{pos, dir: Point::from_dir(Dir::Up)});
                // re-normalize it as empty space
                grid[y][x] = EMPTY;
            }
        }
    }

    let map = Map{ grid };
    assert!(guards.len() == 1);
    ( map, guards[0] )
}

impl Map {
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

    fn print(grid: &Grid, g: &Guard) {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let c;
                if g.pos == Point::new(x as i32, y as i32) {
                    c = '^';
                } else {
                    c = grid[y][x];
                }
                print!("{}", c);
            }
            print!("{}", "\n");
        }
    }

    // not a typo.  this interface moves out of self
    // the reason is we don't want the obstacles that were
    // added from previous runs to leak into future ones
    fn is_loop(mut self, mut g: Guard, obs: Point) -> bool {
        assert!(self.contains(obs));
        if obs == g.pos {
            return false;
        }

        // add the new obstacle to the grid
        self.grid[obs.y as usize][obs.x as usize] = NEW_OBSTACLE;

        let mut seen : HashSet<(Point, Point)> = HashSet::new();
        loop {
            assert!(g.pos != obs);
            // if we see the same tuple of guard pos/dir
            // it means we're in a loop
            if seen.contains(&(g.pos, g.dir)) {
                return true;
            }
            seen.insert((g.pos, g.dir));
            let np = self.peek(g.pos, g.dir);
            // escaped the map?
            if np.is_none() {
                return false;
            }
            // otherwise, what tile are we looking at?
            let np = np.unwrap();
            let cur = self.get(np).unwrap();
            match cur {
                OBSTACLE | NEW_OBSTACLE => {
                    g.dir.rotate(Dir::Right);
                },
                EMPTY => {
                    g.pos = g.pos + g.dir;
                },
                _ => unreachable!(),
            }
        }
    }
}

fn part1(buf: &str) -> u32 {
    let (m, mut g) = parse(buf);
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
            OBSTACLE | NEW_OBSTACLE => {
                g.dir.rotate(Dir::Right);
            },
            EMPTY => {
                g.pos = g.pos + g.dir;
            },
            _ => todo!(),
        }
    }
    seen.len() as u32
}

fn part2(buf: &str) -> u32 {
    let (m, g) = parse(buf);
    let mut out = 0;
    for y in 0..m.grid.len() {
        for x in 0..m.grid[y].len() {
            let obs = Point::new(x as i32, y as i32);
            let mn = m.clone();
            // there has to be a better way to design this that doesn't require
            // a clone on every invocation
            // maybe a second obstacle grid-layer that lives on top of the
            // underlying map itself?
            if mn.is_loop(g.clone(), obs) {
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
    println!("part2: {}", part2(&buf));
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
        assert_eq![part2(input), 6];
    }
}

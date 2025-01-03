use std::io::{stdin, Read};
use std::collections::{ HashSet, VecDeque };
use util::{Dir, Point};
use std::cmp;

type Grid = Vec<Vec<char>>;
struct Map {
    grid: Grid,
    start: Point,
    goal: Point,
}

const START : char = 'S';
const GOAL  : char = 'E';

impl Map {
    fn from_str(s: &str) -> Self {
        let mut grid: Grid = vec![];
        let mut start = Point::new(-1, -1);
        let mut goal = Point::new(-1, -1);
        for l in s.split('\n') {
            grid.push(l.chars().collect());
        }


        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                match grid[y][x] {
                    START => start = Point::new(x as i32, y as i32),
                    GOAL  => goal = Point::new(x as i32, y as i32),
                    _ => {},
                }
            }
        }
        assert![start.x > -1];
        assert![goal.x > -1];
        Self{ grid, start, goal }
    }

    fn get_ways(&self, src: Point) -> Vec<(Point, Dir, char)> {
        let mut out = vec![];
        for dir in Point::all_dirs() {
            let dst = src + Point::from_dir(dir);
            if let Some(c) = self.get(dst) {
                match c {
                    '.' | GOAL => out.push((dst, dir, c)),
                    _ => {},
                }
            }
        }
        out
    }

    fn get(&self, p: Point) -> Option<char> {
        if p.x < 0
                || p.y < 0
                || p.x as usize > self.grid[0].len()
                || p.y as usize > self.grid.len() {
            return None;
        }
        Some(self.grid[p.y as usize][p.x as usize])
    }
}

fn part1(buf: &str) -> u32 {
    let m = Map::from_str(buf);
    let mut frontier : VecDeque<(Point, Dir, u32, u32)> = VecDeque::new();
    for (pt, _dir, _c) in m.get_ways(m.start) {
        // character starts facing east, so right
        frontier.push_back((pt, Dir::Right, 0, 0));
    }
    let mut seen : HashSet<Point> = HashSet::new();

    let mut best = u32::MAX;
    while frontier.len() > 0 {
        let (pos, dir, steps, turns) = frontier.pop_front().unwrap();
        if pos == m.goal {
            let score = (turns * 1000) + steps + 1;
            println!("goal! {} [s:{} t:{}]", score, steps, turns);
            best = cmp::min(best, score);
            continue;
        }

        if !seen.insert(pos) {
            continue;
        }
        let nways = m.get_ways(pos);
        for (np, ndir, _c) in nways {
            let nturn = if dir == ndir { 0 } else { 1 };
            frontier.push_back((np, ndir, steps + 1, turns + nturn));
        }
    }
    best
}

fn part2(_buf: &str) -> u32 {
    todo!();
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
        let s = "#####
#E.S#
#####";
        assert_eq![part1(s), 1002];
    }

    #[test]
    fn example1() {
        let s = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
            assert_eq![part1(s), 7036];
    }

    #[test]
    fn example2() {
    }
}

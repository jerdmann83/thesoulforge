use std::io::{stdin, Read};
use std::collections::{ HashMap, HashSet, VecDeque };
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

type Path = Vec<Point>;
type Paths = Vec<Path>;

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

    fn search(&self) -> (u32, Paths) {
        // let mut rpath : Nodes = HashMap::new();
        let mut rcost : HashMap<Point, u32> = HashMap::new();
        // rpath.insert(self.start, vec![Point::new(-1, -1)]);
        rcost.insert(self.start, 0);

        let mut all_paths = vec![];
        let mut frontier : VecDeque<(Point, Dir, u32, Vec<Point>)> = VecDeque::new();
        // character starts facing east, so right
        frontier.push_back((self.start, Dir::Right, 0, vec![]));

        while frontier.len() > 0 {
            let (pos, dir, cost, path) = frontier.pop_front().unwrap();
            if pos == self.goal {
                println!("goal! s:{}", cost);
                all_paths.push((path, cost));
                continue;
            }

            let nways = self.get_ways(pos);
            for (np, ndir, _c) in nways {
                let nturn : u32;
                if dir == ndir { 
                    nturn = 0;
                } else { 
                    let dp = Point::from_dir(dir);
                    let ndp = Point::from_dir(ndir);
                    if (dp.x == 0 && ndp.x == 0)
                        || (dp.y == 0 && ndp.y == 0) {
                        nturn = 2;
                    } else {
                        nturn = 1;
                    }
                }

                let mut known_cost = u32::MAX;
                if let Some(v) = rcost.get(&np) {
                    known_cost = *v;
                }
                let new_cost = cost + 1 + (nturn * 1000);

                if new_cost < known_cost {
                    let mut npath = path.clone();
                    npath.push(np);
                    frontier.push_back((np, ndir, new_cost, npath));
                    // rpath.entry(np).or_insert(vec![]).push(pos);
                    rcost.insert(np, new_cost);
                }
            }
        }

        let mut best = u32::MAX;
        for (_, cost) in &all_paths {
            best = cmp::min(best, *cost);
        }
        let mut paths = vec![];
        for (path, cost) in &all_paths {
            if *cost == best {
                paths.push(path.clone());
            }
        }
        (best, paths)
    }
}

fn part1(buf: &str) -> u32 {
    let m = Map::from_str(buf);
    m.search().0
}

fn part2(buf: &str) -> u32 {
    let m = Map::from_str(buf);
    let paths = m.search().1;
    let mut all_points = HashSet::new();
    for path in paths {
        for pt in path {
            all_points.insert(pt);
        }
    }
    all_points.len() as u32
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
#S.E#
#####";
        // zero turns needed
        assert_eq![part1(s), 2];

        let s = "#####
#E.S#
#####";
        // two turns as start facing is east
        assert_eq![part1(s), 2002];
    }

    #[test]
    fn split() {
        let s = "#######
#....S#
#.###.#
#.....#
#.#####
#.#####
#E#####
#######";
        part1(s);
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
        let s = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
            assert_eq![part2(s), 64];
    }
}

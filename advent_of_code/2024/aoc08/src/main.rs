use std::io::{stdin, Read};
use util::Point;
use std::collections::HashMap;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Antenna = HashMap<char, Vec<Point>>;
#[derive(Clone, Debug)]
struct Map {
    grid: Grid,
    antenna: Antenna,
}


fn parse(s: &str) -> Map {
    let mut grid = vec![];
    let mut antenna : Antenna = Antenna::new();
    for (yi,l) in s.split("\n").enumerate() {
        let mut row = vec![];
        for (xi,c) in l.chars().enumerate() {
            if c != EMPTY {
                let p = Point::new(xi as i32, yi as i32);
                antenna.entry(c).or_insert(vec![]).push(p);
            }
            row.push(c);
        }
        grid.push(row);
    }
    Map{ grid, antenna }
}

const EMPTY : char = '.';

impl Map {
    fn get_antinodes(&self) -> HashSet<Point> {
        let mut out = HashSet::new();
        for (c, points) in &self.antenna {
            for p1 in points {
                for p2 in points {
                    if *p1 == *p2 {
                        continue;
                    }
                    let slope = *p2 - *p1;
                    let dst = *p2 + slope;
                    if self.contain(dst) {
                        println!("{} at {:?}", c, dst);

                        out.insert(dst);
                    }
                }
            }
        }
        out
    }

    fn contain(&self, p: Point) -> bool {
        p.x > -1 && p.y > -1
            && p.x < self.grid.len() as i32
            && p.y < self.grid[0].len() as i32
    }
}

fn part1(buf: &str) -> u32 {
    let m = parse(buf);
    println!("{:?}", m);
    let nodes = m.get_antinodes();
    nodes.len() as u32
}

fn part2(buf: &str) -> u32 {
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
    fn example1() {
        let s = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq![part1(s), 14];
    }
}

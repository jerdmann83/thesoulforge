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
}

type Grid = Vec<Vec<u32>>;

fn get_adjacent(grid: &Grid, p: Point) -> Vec<u32> {
    let offsets = vec![
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(1, 0),
        Point::new(-1, 0),
    ];

    let mut out = vec![];
    for o in &offsets {
        let mut cur = p.clone();
        cur += *o;
        let max_x = grid[0].len() as i32;
        let max_y = grid.len() as i32;
        if cur.x < 0 || cur.y < 0 || cur.x >= max_x || cur.y >= max_y {
            continue;
        }
        out.push(grid[cur.y as usize][cur.x as usize]);
    }
    out
}

fn parse(buf: &str) -> Grid {
    let mut grid = vec![];
    let mut row = 0;
    for l in buf.split('\n') {
        let mut next_row = vec![];
        for c in l.chars() {
            let val = c.to_digit(10).unwrap();
            next_row.push(val);
        }
        if next_row.len() > 0 {
            grid.push(next_row);
        }
        row += 1;
    }
    grid
}

fn part1(grid: &Grid) -> u32 {
    let mut lows = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cur = grid[y][x];
            let adj = get_adjacent(grid, Point::new(x as i32, y as i32));
            let mut is_lowest = true;
            for val in adj {
                if cur >= val {
                    is_lowest = false;
                    break;
                }
            }
            if is_lowest {
                lows.push(cur);
            }
        }
    }
    let mut out = 0;
    for low in lows {
        out += low + 1;
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let lines = parse(&buf);

    println!("part1: {}", part1(&lines));
    // println!("part2: {}", solve(lines.clone(), Part::Part2));
}

mod test {
    use super::*;

    #[test]
    fn test() {}
}

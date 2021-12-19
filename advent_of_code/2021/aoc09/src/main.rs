use std::collections::HashSet;
use std::io::{stdin, Read};
use std::ops::AddAssign;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

fn get_adjacent(grid: &Grid, p: Point) -> Vec<Point> {
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
        out.push(Point::new(cur.x, cur.y));
    }
    out
}

fn parse(buf: &str) -> Grid {
    let mut grid = vec![];
    for l in buf.split('\n') {
        let mut next_row = vec![];
        for c in l.chars() {
            let val = c.to_digit(10).unwrap();
            next_row.push(val);
        }
        if next_row.len() > 0 {
            grid.push(next_row);
        }
    }
    grid
}

fn get_point(grid: &Grid, point: &Point) -> u32 {
    grid[point.y as usize][point.x as usize]
}

fn get_low_points(grid: &Grid) -> Vec<Point> {
    let mut lows = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cur = grid[y][x];
            let adj = get_adjacent(grid, Point::new(x as i32, y as i32));
            let mut is_lowest = true;
            for p in adj {
                if cur >= get_point(grid, &p) {
                    is_lowest = false;
                    break;
                }
            }
            if is_lowest {
                lows.push(Point::new(x as i32, y as i32));
            }
        }
    }
    lows
}

fn get_basin(grid: &Grid, low: &Point) -> Vec<Point> {
    let mut remain = get_adjacent(grid, *low);
    let mut points = vec![];
    let mut seen = HashSet::new();
    while let Some(next) = remain.pop() {
        if !seen.insert(next) {
            continue;
        }
        if get_point(grid, &next) == 9 {
            continue;
        }
        points.push(next);
        for p in get_adjacent(grid, next) {
            remain.push(p);
        }
    }
    points
}

fn part1(grid: &Grid) -> u32 {
    let mut out = 0;
    let lows = get_low_points(grid);
    for p in lows {
        out += get_point(grid, &p) + 1;
    }
    out
}

fn part2(grid: &Grid) -> u32 {
    let lows = get_low_points(grid);

    let mut sizes = vec![];
    for low in lows {
        let points = get_basin(grid, &low);
        sizes.push(points.len());
    }
    sizes.sort();
    let mut out = 1;
    for i in 0..3 {
        out *= sizes[sizes.len() - 1 - i];
    }
    out as u32
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let grid = parse(&buf);

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

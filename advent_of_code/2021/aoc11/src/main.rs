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

    fn from_usize(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    val: u32,
    flashed: bool,
}
type Grid = Vec<Vec<Tile>>;

impl Tile {
    fn bump(&mut self) -> bool {
        self.val += 1;
        let mut out = false;
        if self.val > 9 && !self.flashed {
            self.flashed = true;
            out = true;
        }
        out
    }

    fn reset(&mut self) {
        self.val = 0;
        self.flashed = false;
    }
}

fn get_adjacent(grid: &Grid, p: Point) -> Vec<Point> {
    let mut offsets = vec![];
    for y in -1..2 {
        for x in -1..2 {
            if y == 0 && x == 0 {
                continue;
            }
            offsets.push(Point::new(y, x));
        }
    }

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
            next_row.push(Tile {
                val: val,
                flashed: false,
            });
        }
        if next_row.len() > 0 {
            grid.push(next_row);
        }
    }
    grid
}

fn get_tile<'a>(grid: &'a mut Grid, point: &Point) -> &'a mut Tile {
    &mut grid[point.y as usize][point.x as usize]
}

fn run_turn(mut grid: &mut Grid) -> usize {
    let mut flashes = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            get_adjacent(&grid, Point::from_usize(x, y));
            let tile = &mut grid[y][x];
            if tile.bump() {
                flashes.push(Point::from_usize(x, y));
            }
        }
    }

    let mut q = flashes.clone();
    while q.len() > 0 {
        let cur = q.pop().unwrap();
        for point in get_adjacent(&grid, cur) {
            let tile = &mut get_tile(&mut grid, &point);
            if tile.bump() {
                q.push(point);
                flashes.push(point);
            }
        }
    }

    for point in &flashes {
        let tile = &mut grid[point.y as usize][point.x as usize];
        tile.reset();
    }

    flashes.len()
}

fn part1(mut grid: Grid, turns: u32) -> usize {
    let mut out = 0;
    for _turn in 0..turns {
        out += run_turn(&mut grid);
    }
    out
}

fn part2(mut grid: Grid) -> usize {
    let mut expect = 0;
    for y in 0..grid.len() {
        for _ in 0..grid[y].len() {
            expect += 1;
        }
    }

    for turn in 1..9999 {
        let cur = run_turn(&mut grid);
        if cur == expect {
            return turn;
        }
    }
    todo!();
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let grid = parse(&buf);

    println!("part1: {}", part1(grid.clone(), 100));
    println!("part2: {}", part2(grid.clone()));
}

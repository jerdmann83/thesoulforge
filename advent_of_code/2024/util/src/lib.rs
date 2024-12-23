use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
    pub fn from_dir(d: Dir) -> Self {
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
    pub fn rotate(&mut self, d: Dir) {
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

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub g: Vec<Vec<char>>,
}

impl Grid {
    pub fn from_str(s: &str) -> Self {
        let mut g = vec![];
        for l in s.split('\n') {
            let row : Vec<char> = l.chars().collect();
            if row.is_empty() {
                continue;
            }
            g.push(row);
        }
        Self { g }
    }

    pub fn get(&self, p: Point) -> Option<char> {
        if p.x < 0 || p.y < 0 
            || p.x as usize >= self.g[0].len()
                || p.y as usize >= self.g.len() {
                    return None;
                }
        Some(self.g[p.y as usize][p.x as usize])
    }

    pub fn get_nbr(&self, p: Point) -> Vec<Point> {
        let dirs : [Point; 4] = [
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(0, 1)];
        let mut out = vec![];
        for d in dirs {
            let np = p + d;
            if let Some(_) = self.get(np) {
                out.push(np);
            }
        }
        out
    }

    pub fn iter(&self) -> GridIt {
        GridIt::from_grid(self)
    }
}

pub struct GridIt<'a> {
    pos: Point,
    grid: &'a Grid,
}

impl<'a> GridIt<'a> {
    pub fn from_grid(grid: &'a Grid) -> Self {
        let pos = Point::new(0, 0);
        Self { pos, grid }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridItem {
    pub pos: Point,
    pub val: char,
}

impl<'a> Iterator for GridIt<'a> {
    type Item = GridItem;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let val = self.grid.get(pos)?;

        self.pos.x += 1;
        if self.grid.get(self.pos).is_none() {
            self.pos.x = 0;
            self.pos.y += 1;
        }
        Some(GridItem{ pos, val })
    }
}

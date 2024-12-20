use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
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

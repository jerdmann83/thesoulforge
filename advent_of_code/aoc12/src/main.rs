use std::io::{stdin, Read};
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // this new fn is hilariously worse than a raw struct
    // why did I do it this way.....
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }

    fn nonzero(&self) -> bool {
        self.x != 0 || self.y != 0
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug)]
struct Ship {
    // unit circle degrees thing where 0 is right, 180 is left
    face: i32,
    pos: Point,

    // waypoint at some point relative to the ship
    wp: Point,
}

impl Ship {
    fn new() -> Self {
        Ship {
            face: 0,
            pos: Point::new(0, 0),
            wp: Point::new(10, 1),
        }
    }

    // positive is left, positive right
    // possibly been reading too much godot / vector stuff lately
    fn rotate(&mut self, deg: i32) {
        self.face += deg;
        if self.face > 359 {
            self.face -= 360;
        } else if self.face < 0 {
            self.face += 360;
        }
    }

    fn mv(&mut self, dir: Point) {
        self.pos += dir;
    }

    fn mv_to_wp(&mut self, times: i32) {
        self.pos.x += self.wp.x * times;
        self.pos.y += self.wp.y * times;
    }

    fn wp_mv(&mut self, dir: Point) {
        self.wp += dir;
    }

    // hack force single direction because lazy
    fn wp_rotate_left(&mut self, deg: i32) {
        let mut rem = deg;
        while rem > 0 {
            let x = self.wp.x;
            let y = self.wp.y;
            self.wp.x = -y;
            self.wp.y = x;
            rem -= 90;
        }
    }

    fn forward(&mut self, len: i32) {
        match self.face {
            0 => self.pos.x += len,
            90 => self.pos.y += len,
            180 => self.pos.x -= len,
            270 => self.pos.y -= len,
            _ => todo!(),
        }
    }

    fn manhattan(&self) -> u32 {
        (self.pos.x.abs() + self.pos.y.abs()) as u32
    }
}

fn part1(buf: &str) -> u32 {
    let mut s = Ship::new();
    for l in buf.split_whitespace() {
        if l.len() < 2 {
            continue;
        }
        let action = &l[0..1];
        let num = l[1..].parse::<i32>().unwrap();

        match action {
            // R90
            "N" => s.mv(Point::new(0, num)),
            "S" => s.mv(Point::new(0, -num)),
            "E" => s.mv(Point::new(num, 0)),
            "W" => s.mv(Point::new(-num, 0)),
            "L" => s.rotate(num),
            "R" => s.rotate(-num),
            "F" => s.forward(num),
            _ => todo!(),
        }
    }

    s.manhattan()
}

fn part2(buf: &str) -> u32 {
    let mut s = Ship::new();
    for l in buf.split_whitespace() {
        if l.len() < 2 {
            continue;
        }
        let action = &l[0..1];
        let num = l[1..].parse::<i32>().unwrap();

        match action {
            // R90
            "N" => s.wp_mv(Point::new(0, num)),
            "S" => s.wp_mv(Point::new(0, -num)),
            "E" => s.wp_mv(Point::new(num, 0)),
            "W" => s.wp_mv(Point::new(-num, 0)),
            "L" => s.wp_rotate_left(num),
            "R" => s.wp_rotate_left(360 - num),
            "F" => s.mv_to_wp(num),
            _ => todo!(),
        }
    }

    s.manhattan()
}
fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    println!("part1: {}", part1(&buf));
    println!("part2: {}", part2(&buf));
}

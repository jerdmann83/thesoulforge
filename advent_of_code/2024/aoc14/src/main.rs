use std::io::{stdin, Read};
use regex::Regex;
use util::Point;

fn part1(buf: &str, width: i32, height: i32, seconds: u32) -> u32 {
    let mut robots = parse(buf);
    for _ in 0..seconds {
        for r in &mut robots {
            r.pos.x = (r.pos.x + r.vel.x + width) % width;
            r.pos.y = (r.pos.y + r.vel.y + height) % height;
        }
    }

    let xmid : i32 = width / 2;
    let ymid : i32 = height / 2;
    let mut quads : Vec<Vec<Robot>> = vec![];
    for _ in 0..4 {
        quads.push(vec![]);
    }
    for r in &robots {
        if r.pos.x < xmid && r.pos.y < ymid {
            quads[0].push(*r);
        }
        else if r.pos.x > xmid && r.pos.y < ymid {
            quads[1].push(*r);
        }
        else if r.pos.x > xmid && r.pos.y > ymid {
            quads[2].push(*r);
        }
        else if r.pos.x < xmid && r.pos.y > ymid {
            quads[3].push(*r);
        }
        else {
            assert!(r.pos.x == xmid || r.pos.y == ymid);
        }
    }

    let mut out = 1;
    for q in &quads {
        out *= q.len();
    }
    out as u32
}

fn part2(buf: &str) -> u32 {
    todo!();
}

#[derive(Clone, Copy, Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse(s: &str) -> Vec<Robot> {
    // p=9,5 v=-3,-3
    let rex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut out = vec![];
    for (_, [px, py, vx, vy]) in rex.captures_iter(s).map(|c| c.extract()) {
        let r = Robot { 
            pos: Point::new(
                px.parse::<i32>().unwrap(),
                py.parse::<i32>().unwrap()),
            vel: Point::new(
                vx.parse::<i32>().unwrap(),
                vy.parse::<i32>().unwrap())
        };
        out.push(r);
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf, 101, 103, 100));
    // println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let s = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq![part1(s, 7, 11, 100), 12];
    }

    #[test]
    fn example2() {
    }
}

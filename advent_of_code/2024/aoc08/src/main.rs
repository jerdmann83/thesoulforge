use std::io::{stdin, Read};
use util::Point;

type Grid = Vec<Vec<char>>;

#[derive(Clone, Debug)]
struct Map {
    grid: Grid,
}

const EMPTY : char = '.';
const GUARD : char = '^';
const OBSTACLE : char = '#';
const NEW_OBSTACLE : char = 'O';

fn part1(buf: &str) -> u32 {
    todo!();
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
        let p = Point::new(1, 1);
        let s = "
............
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

    #[test]
    fn example2() {
    }
}

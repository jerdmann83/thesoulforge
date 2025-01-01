use std::io::{stdin, Read};
use util::Point;

type Grid = Vec<Vec<char>>;
type Moves = Vec<char>;
struct Map {
    grid: Grid,
}

impl Map {
    pub fn get(&self, p: Point) -> Option<char> {
        if p.x < 0 || p.y < 0 
            || p.x as usize >= self.grid[0].len()
                || p.y as usize >= self.grid.len() {
                    return None;
                }
        Some(self.grid[p.y as usize][p.x as usize])
    }

    pub fn swap(&mut self, p1: Point, p2: Point) {
        let c1 = self.get(p1).unwrap();
        let c2 = self.get(p2).unwrap();
        self.grid[p1.y as usize][p1.x as usize] = c2;
        self.grid[p2.y as usize][p2.x as usize] = c1;
    }
}

fn parse(s: &str) -> ( Map, Moves ) {
    let chunks : Vec<&str> = s.split("\n\n").collect();
    assert_eq![chunks.len(), 2];
    let mut grid : Grid = vec![];
    for l in chunks[0].split('\n') {
        grid.push(l.chars().collect());
    }

    let moves : Moves = chunks[1].chars().collect();
    ( Map { grid }, moves )
}

fn score(g: &Grid) -> u32 {
    let mut out = 0;
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            match g[y][x] {
                'O' => out += (100 * y) + x,
                _ => {},
            }
        }
    }
    out as u32
}

fn part1(buf: &str) -> u32 {
    let (mut m, moves) = parse(buf);
    let mut robot = Point::new(-1, -1);
    for y in 0..m.grid.len() {
        for x in 0..m.grid[y].len() {
            if m.grid[y][x] == '@' {
                robot = Point::new(x as i32, y as i32);
                break;
            }
        }
    }
    assert![robot.x > -1];

    for mv in &moves {
        let dir : Point;
        match mv {
            '<' => dir = Point::new(-1, 0),
            '>' => dir = Point::new(1, 0),
            '^' => dir = Point::new(0, -1),
            'v' => dir = Point::new(0, 1),
            _ => continue,
        }

        let mut boxes = vec![];
        let mut pt = robot + dir;
        let mut last = '#';
        let next = m.get(pt);
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        match next {
            '#' => continue,
            '.' => {
                m.swap(robot, pt);
                robot = pt;
            }
            'O' => {
                while let Some(tile) = m.get(pt) {
                    last = tile;
                    match tile {
                        'O' => boxes.push(pt),
                        _ => break,
                    }
                    pt = pt + dir;
                }
                match last {
                    '#' => continue,
                    '.' => {
                        for pt in boxes.into_iter().rev() {
                            let mov = pt + dir;
                            m.swap(pt, mov);
                        }

                        let mov = robot + dir;
                        m.swap(robot, mov);
                        robot = mov;
                    },
                    _ => todo!(),
                }
            }
            _ => {},
        }
    }

    score(&m.grid)
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
        let s = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq![part1(s), 2028];
    }

    #[test]
    fn example2() {
    }

    #[test]
    fn test_score() {
        let s = "#######
#...O..#
#......#
########

>>>";
        let (m, _moves) = parse(s);
        assert_eq![score(&m.grid), 104];
    }
}

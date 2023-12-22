use std::io::{stdin, Read};
use je::point::Point;

fn part1(buf: &str) -> usize {
    let u = Universe::from_str(buf);
    u.distance_all(1)
}

fn part2(buf: &str, dilate_size: i32) -> usize {
    let u = Universe::from_str(buf);
    u.distance_all(dilate_size)
}

#[derive(Debug)]
struct Universe {
    dilate_rows: Vec<usize>,
    dilate_cols: Vec<usize>,
    galaxies: Vec<Point>,
}

impl Universe {
    fn from_str(s: &str) -> Self {
        let mut dilate_rows = vec![];
        let mut dilate_cols = vec![];
        let mut galaxies = vec![];
        let mut galaxies_by_col : Vec<usize> = vec![];
        let rows : Vec<&str> = s.split('\n').collect();
        for y in 0..rows.len() {
            let row : Vec<char> = rows[y].chars().collect();
            if row.len() == 0 {
                continue;
            }
            if galaxies_by_col.len() == 0 {
                galaxies_by_col = vec![0; row.len()];
            }
            let mut num = 0;
            for x in 0..row.len() {
                if row[x] == '#' {
                    galaxies.push(Point::new(x as i32, y as i32));
                    galaxies_by_col[x] += 1;
                    num += 1;
                }
            }
            if num == 0 {
                dilate_rows.push(y);
            }
        }

        for x in 0..galaxies_by_col.len() {
            if galaxies_by_col[x] == 0 {
                dilate_cols.push(x);
            }
        }

        Self{ dilate_rows, dilate_cols, galaxies }
    }

    fn distance(&self, p1: Point, p2: Point, dilate_size: i32) -> usize {
        // let dist = p1 - p2;
        // let mut xdist = dist.x.abs();
        let mut xdist = 0;
        let xbegin = std::cmp::min(p1.x, p2.x);
        let xend   = std::cmp::max(p1.x, p2.x);
        for x in xbegin..xend {
            let mut step = 1;
            if self.dilate_cols.contains(&(x as usize)) {
                step = dilate_size;
            }
            xdist += step;
        }

        // let mut ydist = dist.y.abs();
        let mut ydist = 0;
        let ybegin = std::cmp::min(p1.y, p2.y);
        let yend   = std::cmp::max(p1.y, p2.y);
        for y in ybegin..yend {
            let mut step = 1;
            if self.dilate_rows.contains(&(y as usize)) {
                step = dilate_size;
            }
            ydist += step;
        }

        // xdist += 1;
        // ydist += 1;
        xdist as usize + ydist as usize
    }

    fn galaxy_pairs(&self) -> Vec<Vec<Point>> {
        let mut out : Vec<Vec<Point>> = vec![];
        for g1 in &self.galaxies {
            for g2 in &self.galaxies {
                if g1 == g2 {
                    continue;
                }

                let mut points = vec![*g1, *g2];
                points.sort();
                if out.contains(&points) {
                    continue;
                }
                out.push(points);
            }
        }
        out
    }

    fn distance_all(&self, dilate_size: i32) -> usize {
        let mut out = 0;
        let mut num = 0;
        for gp in self.galaxy_pairs() {
            out += self.distance(gp[0], gp[1], dilate_size);
            num += 1;
        }
        out
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    println!("part2: {}", part2(&buf, 1000000));
}

#[cfg(test)]
mod test {
    use super::*;
    // # . . # -> 3
    // #....................# -> 3

    #[test]
    fn just_two() {
        let input = ".#..#..
.......
.......".to_string();
        let u = Universe::from_str(&input);
        assert_eq!(u.distance_all(1), 3);
        assert_eq!(u.distance_all(10), 21);
    }

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 374);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT, 10), 1030);
    }
}

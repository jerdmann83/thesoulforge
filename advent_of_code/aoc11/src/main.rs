use std::fmt;
use std::io::{stdin, Read};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl Spot {
    fn from_char(c: &char) -> Option<Self> {
        match &c {
            '.' => Some(Spot::Floor),
            'L' => Some(Spot::Empty),
            '#' => Some(Spot::Occupied),
            _ => None,
        }
    }
}

type GridT = Vec<Vec<Spot>>;

fn parse(buf: &str) -> GridT {
    let mut g: GridT = vec![];
    for l in buf.split_whitespace() {
        let mut row = vec![];
        if l.len() == 0 {
            continue;
        }
        for c in l.chars() {
            row.push(Spot::from_char(&c).unwrap());
        }
        g.push(row);
    }
    g
}

fn neighbors(g: &GridT, x: usize, y: usize, goal: Spot) -> u32 {
    let mut coords: Vec<(i32, i32)> = vec![];
    for cx in -1..2 {
        for cy in -1..2 {
            // every neighbor, not the cell itself
            if !(cx == 0 && cy == 0) {
                coords.push((cx, cy));
            }
        }
    }

    let mut num = 0;
    let mut out: Vec<(usize, usize)> = vec![];
    for c in coords {
        let xn = x as i32 + c.0;
        let yn = y as i32 + c.1;
        // some sort of better/more idiomatic clamp thing here?
        if xn < 0 || yn < 0 {
            continue;
        } else if yn as usize >= g.len() || xn as usize >= g[0].len() {
            continue;
        }
        out.push((xn as usize, yn as usize));

        let cur = &g[yn as usize][xn as usize];
        if *cur == goal {
            num += 1;
        }
    }
    num
}

fn count(g: &GridT, goal: Spot) -> u32 {
    let mut num = 0;
    for y in 0..g.len() {
        for x in 0..g[0].len() {
            if g[y][x] == goal {
                num += 1;
            }
        }
    }
    num
}

fn run_until_stable(g: &GridT) -> GridT {
    // simple double-buffered grid flip thing
    // initially g1 is a copy of the original grid and g2 is an all-empty one
    //
    // copy cells from one to the other to honor the simultaneous part of the
    // grid update behavior
    let mut g1 = g.to_vec();
    let mut g2 = g.to_vec();
    for y in 0..g2.len() {
        for x in 0..g2[0].len() {
            let mut cur = &mut g2[y][x];
            *cur = Spot::Empty;
        }
    }

    let mut src = &mut g1;
    let mut dst = &mut g2;

    let mut num = 0;
    loop {
        num += 1;
        if *src == *dst {
            println!("complete after {} iterations", num);
            break;
        }

        // this is wildly inefficient as it swaps the values around.  my intent
        // is to just reseat the references each cycle but this terrible
        // brute-force approach works for now
        std::mem::swap(src, dst);

        run_one(&src, &mut dst, &num);
    }
    src.to_vec()
}

fn run_one(src: &GridT, dst: &mut GridT, num: &u32) {
    for y in 0..src.len() {
        for x in 0..src[0].len() {
            let mut cur = src[y][x].clone();
            // If a seat is empty (L) and there are no occupied seats
            // adjacent to it, the seat becomes occupied.
            //
            // If a seat is occupied (#) and four or more seats adjacent to
            // it are also occupied, the seat becomes empty.
            match cur {
                Spot::Empty => {
                    if neighbors(&src, x, y, Spot::Occupied) == 0 {
                        cur = Spot::Occupied;
                        // assert!(*num == 1 || dst[y][x] == Spot::Empty);
                    }
                }
                Spot::Occupied => {
                    if neighbors(&src, x, y, Spot::Occupied) > 3 {
                        cur = Spot::Empty;
                        // assert!(*num == 1 || dst[y][x] == Spot::Occupied);
                    }
                }
                Spot::Floor => {
                    assert!(dst[y][x] == Spot::Floor);
                }
            }
            println!("{},{} src={:?} dst={:?}", x, y, src[y][x], cur);

            dst[y][x] = cur;
        }
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let g = parse(&buf);

    let gn = run_until_stable(&g);
    let num = count(&gn, Spot::Occupied);
    println!("part1: {}", num);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nbr() {
        let mut buf = String::new();
        buf.push_str("LLL\n");
        buf.push_str("...\n");
        buf.push_str("###\n");
        buf.push_str("...\n");
        let g: GridT = parse(&buf);
        assert_eq!(neighbors(&g, 0, 0, Spot::Floor), 2);
        assert_eq!(neighbors(&g, 2, 2, Spot::Occupied), 1);
        assert_eq!(neighbors(&g, 1, 1, Spot::Empty), 3);
        assert_eq!(neighbors(&g, 1, 1, Spot::Occupied), 3);
        assert_eq!(neighbors(&g, 1, 2, Spot::Floor), 6);

        assert_eq!(count(&g, Spot::Floor), 6);
        assert_eq!(count(&g, Spot::Occupied), 3);
        assert_eq!(count(&g, Spot::Empty), 3);
    }

    #[test]
    fn test_run_one() {
        let mut buf = String::new();
        buf.push_str("LLL\n");
        buf.push_str("LLL\n");
        buf.push_str("LLL\n");
        let mut g: GridT = parse(&buf);
        let mut gn: GridT = g.to_vec();
        run_one(&g, &mut gn);
        // ###
        // ###
        // ###
        assert_eq!(count(&gn, Spot::Floor), 0);
        assert_eq!(count(&gn, Spot::Empty), 0);
        assert_eq!(count(&gn, Spot::Occupied), 9);

        std::mem::swap(&mut g, &mut gn);
        run_one(&g, &mut gn);
        // #L#
        // LLL
        // #L#
        assert_eq!(count(&gn, Spot::Floor), 0);
        assert_eq!(count(&gn, Spot::Empty), 5);
        assert_eq!(count(&gn, Spot::Occupied), 4);
    }
}

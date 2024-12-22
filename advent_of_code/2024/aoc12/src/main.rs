use std::io::{stdin, Read};
use util::{Grid, Point};
use std::collections::{ HashMap, HashSet };

type Region = HashSet<Point>;
type Regions = HashMap<char, Region>;

fn get_regions(g: &Grid) -> Regions {
    let mut out : Regions = HashMap::new();
    for it in g.iter() {
        let r = out.entry(it.val).or_insert(Region::new());
        r.insert(it.pos);
    }
    out
}

fn part1(buf: &str) -> u32 {
    let g = Grid::from_str(buf);
    let r = get_regions(&g);
    println!("{:?}", r);
    0
}

fn part2(buf: &str) -> u32 {
    todo!();
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    // println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let s = "AAAA
BBCD
BBCC
EEEC";
        assert_eq![part1(s), 140];
    }

    #[test]
    fn example2() {
    }
}

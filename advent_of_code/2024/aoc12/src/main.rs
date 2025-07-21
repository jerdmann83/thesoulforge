use std::io::{stdin, Read};
use util::{Grid, Point};
use std::collections::HashSet;

#[derive(Debug)]
struct Region {
    val: char,
    points: HashSet<Point>,
}
type Regions = Vec<Region>;

fn get_regions(g: &Grid) -> Regions {
    let mut out : Regions = vec![];
    let mut seen : HashSet<Point> = HashSet::new();
    for it in g.iter() {
        // make sure we visit each tile exactly once
        if seen.contains(&it.pos) {
            continue;
        }
        // flood-fill each new tile we land on
        let mut frontier = vec![it.pos];
        seen.insert(it.pos);
        let val = it.val;
        let mut region = Region{ val, points: HashSet::new() };
        while frontier.len() > 0 {
            let cur = frontier.pop().unwrap();
            region.points.insert(cur);

            let nbrs = g.get_nbr(cur);
            for nbr in nbrs {
                // only flood-filling the current region
                if g.get(nbr).unwrap() != val {
                    continue;
                }
                // new tiles only
                if !seen.insert(nbr) {
                    continue;
                }
                frontier.push(nbr);
            }
        }
        assert!(region.points.len() > 0);
        out.push(region);
    }
    out
}

fn perimeter(g: &Grid, r: &Region) -> u32 {
    let mut out = 0;
    let dirs : [Point; 4] = [
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1)];
    for p in &r.points {
        for d in dirs {
            let np = *p + d;
            let nv = g.get(np);
            let val = nv.unwrap_or(' ');
            // no fence needed for any region tile edges touching matching tiles
            if val == r.val {
                continue
            }
            // all other edges need a fence
            // whether at the map edge
            // or touching a non-matching tile
            out += 1;
        }
    }
    out
}

fn part1(buf: &str) -> u32 {
    let g = Grid::from_str(buf);
    let r = get_regions(&g);
    let mut out = 0;
    for reg in r {
        let per = perimeter(&g, &reg);
        let area = reg.points.len() as u32;
        out += area * per;
    }
    out
}

fn part2(buf: &str) -> u32 {
    let g = Grid::from_str(buf);
    let r = get_regions(&g);
    let mut out = 0;
    for reg in r {
        let per = perimeter(&g, &reg);
        let area = reg.points.len() as u32;
        out += area * per;
    }
    out
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
    fn simple() {
        let s = "AA";
        assert_eq![part1(s), 12];
    }

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
        let s = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq![part1(s), 1930];
    }

    #[test]
    fn test_part2() {
        let s = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq![part2(s), 368];
    }


}

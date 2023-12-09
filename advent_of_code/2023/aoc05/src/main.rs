use std::io::{stdin, Read};
use std::collections::HashMap;

type Val = u32;
type Map = HashMap<Val, Val>;

#[derive(Debug)]
struct Range {
    dbegin: Val,
    sbegin: Val,
    len: Val,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Val>,
    ranges: Vec<Map>,
}

impl Almanac {
    fn get_numbers(s: &str) -> Vec<Val> {
        let mut out : Vec<Val> = vec![];
        for t in s.split_whitespace() {
            out.push(t.parse::<Val>().unwrap());
        }
        out
    }

    fn from_str(s: &str) -> Self {
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
        let mut seeds : Vec<Val> = vec![];
        let mut ranges = vec![];
        for sect in s.split("\n\n") {
            println!("almanac: sect");
            let top_toks : Vec<&str> = sect.split(':').collect();
            let label = top_toks[0];
            if label == "seeds" {
                seeds = Self::get_numbers(top_toks[1]);
                continue;
            }

            let vals = Self::get_numbers(top_toks[1]);
            let mut i = 0;
            assert!(vals.len() % 3 == 0);
            let mut map = Map::new();
            while i < vals.len() {
                let dbegin = vals[i];
                let sbegin = vals[i+1];
                let len = vals[i+2];

                for range_i in 0..len {
                    let src = sbegin + range_i;
                    let dst = dbegin + range_i;
                    map.insert(src, dst);
                }
                println!("almanac: map");
                i += 3;
            }
            ranges.push(map);
        }
        assert_eq!(ranges.len(), 7);
        println!("almanac: done");
        Almanac{seeds, ranges}
    }

    fn get_loc(&self, seed: Val) -> Val {
        println!("start seed: {:?}", seed);
        let mut next = seed;
        for r in &self.ranges {
            println!("enter: {}", next);
            let rc = r.get(&next);
            if let Some(val) = rc {
                next = *val;
                continue;
            }
        }
        next
    }

    fn get_locs(&self) -> Vec<Val> {
        let mut out : Vec<Val> = vec![];
        for s in &self.seeds {
            let loc = self.get_loc(*s);
            println!("done. seed {}: loc {}", s, loc);
            out.push(loc);
            println!("{:?}", out);
        }
        out
    }
}

fn part1(buf: &str) -> u32 {
    let a = Almanac::from_str(buf);
    let locs = a.get_locs();
    let mut out = 99999999;
    println!("{:?}", locs);
    for l in locs {
        out = std::cmp::min(l, out);
    }
    out
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

    const INPUT : &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn example2() {
        todo!();
    }
}

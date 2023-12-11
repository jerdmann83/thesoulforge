use std::io::{stdin, Read};

type Val = usize;

#[derive(Debug)]
struct Range {
    dbegin: Val,
    sbegin: Val,
    len: Val,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Val>,
    ranges: Vec<Vec<Range>>,
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
        let mut seeds : Vec<Val> = vec![];
        let mut ranges = vec![];
        for sect in s.split("\n\n") {
            let top_toks : Vec<&str> = sect.split(':').collect();
            if top_toks.len() != 2 {
                continue;
            }

            let label = top_toks[0];
            if label == "seeds" {
                seeds = Self::get_numbers(top_toks[1]);
                continue;
            }

            let vals = Self::get_numbers(top_toks[1]);
            let mut i = 0;
            assert!(vals.len() % 3 == 0);
            let mut level_ranges = vec![];
            while i < vals.len() {
                let dbegin = vals[i];
                let sbegin = vals[i+1];
                let len = vals[i+2];

                level_ranges.push(Range{dbegin, sbegin, len});
                i += 3;
            }
            ranges.push(level_ranges);
        }
        assert_eq!(ranges.len(), 7);
        Almanac{seeds, ranges}
    }

    fn get_loc(&self, seed: Val) -> Val {
        let mut next = seed;
        for r in &self.ranges {
            for sr in r {
                let send = sr.sbegin + sr.len;
                if next >= sr.sbegin && next < send {
                    next = sr.dbegin + (next - sr.sbegin);
                    break;
                }
            }
        }
        next
    }

    fn get_locs(&self) -> Vec<Val> {
        let mut out : Vec<Val> = vec![];
        for s in &self.seeds {
            let loc = self.get_loc(*s);
            out.push(loc);
        }
        out
    }

    fn get_least_loc_part2(&self) -> usize {
        let mut idx : usize = 0;
        let mut out : usize = usize::MAX;
        while idx < self.seeds.len() {
            let sbegin = &self.seeds[idx];
            let len = &self.seeds[idx+1];
            for s in *sbegin..*sbegin+*len {
                let loc = self.get_loc(s);
                out = std::cmp::min(out, loc);
            }
            idx += 2;
        }
        out
    }
}

fn part1(buf: &str) -> usize {
    let a = Almanac::from_str(buf);
    let locs = a.get_locs();
    let mut out = usize::MAX;
    for l in locs {
        out = std::cmp::min(l, out);
    }
    out
}

fn part2(buf: &str) -> usize {
    let a = Almanac::from_str(buf);
    a.get_least_loc_part2()
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
        assert_eq!(part2(INPUT), 46);
    }
}

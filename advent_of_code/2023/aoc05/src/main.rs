use std::io::{stdin, Read};
use std::collections::HashMap;

type Val = u32;
#[derive(Debug)]
struct Range {
    dbegin: Val,
    sbegin: Val,
    len: Val,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Type {
    Soil,
    Fert,
    Water,
    Light,
    Temp,
    Humid,
    Loc,
}

fn type_from_str(s: &str) -> Type {
    match s {
        "seed-to-soil" => return Type::Soil,
        "soil-to-fertilizer" => return Type::Fert,
        "fertilizer-to-water" => return Type::Water,
        "water-to-light" => return Type::Light,
        "light-to-temperature" => return Type::Temp,
        "temperature-to-humidity" => return Type::Humid,
        "humidity-to-location" => return Type::Loc,
        _ => {
            println!("{}", s);
            todo!("{}", s);
        }
    }
}

#[derive(Debug)]
struct Almanac {
    // seed -> soil -> fert -> water 
    // -> light -> temp -> humid -> loc
    //
    // drange -> srange -> len
    seeds: Vec<Val>,
    // map: HashMap<Type, Range>,
    ranges: Vec<Range>,
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
        // let mut map : HashMap<Type, Range> = HashMap::new();
        let mut ranges = vec![];
        for sect in s.split("\n\n") {
            let top_toks : Vec<&str> = sect.split(':').collect();
            let label = top_toks[0];
            if label == "seeds" {
                seeds = Self::get_numbers(top_toks[1]);
                continue;
            }

            // let map_str = label.split_whitespace().collect::<Vec<&str>>()[0];
            // let etype = type_from_str(map_str);
            let vals = Self::get_numbers(top_toks[1]);
            ranges.push(Range{
                dbegin: vals[0],
                sbegin: vals[1],
                len: vals[2],
            });
        }
        Almanac{seeds, ranges}
    }

    fn get_next(v: Val, r: &Range) -> Val {
        for offset in 0..r.len {
            if r.sbegin + offset == v {
                let out = r.dbegin + offset;
                println!("hit: {} -> {}", v, out);
            }
        }
        v
    }

    fn get_loc(&self, seed: Val) -> Val {
        println!("start seed: {:?}", seed);
        let mut out : usize;
        let mut next = seed;
        let mut last;
        for r in &self.ranges {
            last = next;
            next = Self::get_next(next, &r);
            println!("last:{} next:{}", last, next);
        }
        next
    }

    fn get_locs(&self) -> Vec<Val> {
        let mut out : Vec<Val> = vec![];
        for s in &self.seeds {
            let loc = self.get_loc(*s);
            println!("done. seed {}: loc {}", s, loc);
            out.push(self.get_loc(*s));
        }
        out
    }
}

fn part1(buf: &str) -> u32 {
    let a = Almanac::from_str(buf);
    let locs = a.get_locs();
    let mut out = 99999999;
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

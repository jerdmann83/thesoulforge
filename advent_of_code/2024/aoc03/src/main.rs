use std::io::{stdin, Read};
use regex::Regex;

fn part1(buf: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();
    let mut out = 0;

    for f in re.captures_iter(buf) {
        let lhs = f["lhs"].parse::<u32>().unwrap();
        let rhs = f["rhs"].parse::<u32>().unwrap();
        out += lhs * rhs;
    }
    out
}

enum Type {
    Mul(u32, u32),
    Do,
    Dont,
}
struct Instruction {
    pos: usize,
    itype: Type,
}

fn part2(buf: &str) -> u32 {
    // TODO: I'm honestly surprised that the regex crate's API is bifurcated in this way
    // the captures_iter returns Capture structs, which apparently don't store
    // any location/position information?
    // even python's regex module does this natively...
    // let re_mul = Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();
    // let re_do = Regex::new(r"do\(\)").unwrap();
    // let re_dont = Regex::new(r"don't\(\)").unwrap();
    // let instructions : Vec<Instruction> = vec![];
    //
    // let mut locs = re_mul.capture_locations();
    // re_mul.captures_read(&mut locs, buf);
    // println!("{:?}", locs.len());
    // println!("{:?}", locs);
        // let lhs = f["lhs"].parse::<u32>().unwrap();
        // let rhs = f["rhs"].parse::<u32>().unwrap();
        
    let mut out = 0;
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    // println!("part1: {}", part1(&buf));
    println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
    }

    #[test]
    fn example2() {
    }
}

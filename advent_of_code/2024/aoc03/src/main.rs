use std::io::{stdin, Read};
use regex::Regex;

fn part1(buf: &str) -> u32 {
    let re = Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();
    let mut out = 0;

    for c in re.captures_iter(buf) {
        let lhs = c["lhs"].parse::<u32>().unwrap();
        let rhs = c["rhs"].parse::<u32>().unwrap();
        out += lhs * rhs;
    }
    out
}

#[derive(Debug)]
enum Type {
    Mul(u32, u32),
    Do,
    Dont,
}
#[derive(Debug)]
struct Instruction {
    pos: usize,
    itype: Type,
}
type Instructions = Vec<Instruction>;

impl PartialOrd for Instruction {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Instruction {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&rhs.pos)
    }
}

impl PartialEq for Instruction {
    fn eq(&self, rhs: &Self) -> bool {
        self.pos == rhs.pos
    }
}

impl Eq for Instruction {}

fn parse(out: &mut Instructions, buf: &str, re: Regex, cb: dyn &Fn(&str) -> Type) {
    for m in re.find_iter(buf) {
        let s = re.captures(m.as_str()).unwrap();
        let t = cb(s);
        out.push(t);
        // println!("{:?}", s);
        // println!("{:?}", m);
        // out.push(Instruction{pos: m
    }
}

fn build_do(_: &str) -> Type {
    Type::Do
}
fn build_dont(_: &str) -> Type {
    Type::Dont
}
fn build_mul(s: &str) -> Type {
    Type::Mul(1, 1)
}

fn part2(buf: &str) -> u32 {
    let mut instructions : Vec<Instruction> = vec![];
    // parse(&mut instructions, 
    //       buf, 
    //       Regex::new(r"do\(\)").unwrap(),
    //       &build_do);
    // parse(&mut instructions, 
    //       buf, 
    //       Regex::new(r"don't\(\)").unwrap(),
    //       Type::Dont);
    parse(&mut instructions, 
          buf, 
          Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap(),
          Type::Mul(1, 1));
    // instructions.sort_by(|lhs, rhs| lhs.pos < rhs.pos ); 
    instructions.sort();
    for ins in instructions {
        println!("{:?}", ins);

    }
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

use std::io::{stdin, Read};

#[derive(Clone, Debug)]
struct Eqn {
    target: i32,
    vals: Vec<i32>,
}
type Eqns = Vec<Eqn>;

fn parse(s: &str) -> Eqns {
    let mut out : Eqns = vec![];
    for l in s.split("\n") {
        let toks : Vec<&str> = l.split(":").collect();
        if toks.len() != 2 {
            continue;
        }
        let target = toks[0].parse::<i32>().unwrap();
        let mut vals = vec![];
        for vtok in toks[1].split(' ') {
            if let Ok(val) = vtok.parse::<i32>() {
                vals.push(val);
            }
        }
        out.push(Eqn{ target, vals });
    }
    out
}

enum Op {
    Add,
    Mul,
}
fn apply(op: Op, lhs: i32, rhs: i32) -> i32 {
    match op {
        Op::Add => return lhs + rhs,
        Op::Mul => return lhs * rhs,
    }
}

fn part1(eqns: Eqns) -> u32 {
}

fn part2(eqns: Eqns) -> u32 {
    todo!();
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let eqns = parse(&buf);
    println!("part1: {}", part1(eqns.clone()));
    println!("part2: {}", part2(eqns.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let s = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let e = parse(s);
        println!("{:?}", e);
    }

    #[test]
    fn example2() {
    }
}

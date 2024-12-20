use std::io::{stdin, Read};

#[derive(Clone, Debug)]
struct Eqn {
    target: i64,
    vals: Vec<i64>,
}
type Eqns = Vec<Eqn>;

fn parse(s: &str) -> Eqns {
    let mut out : Eqns = vec![];
    for l in s.split("\n") {
        let toks : Vec<&str> = l.split(":").collect();
        if toks.len() != 2 {
            continue;
        }
        let target = toks[0].parse::<i64>().unwrap();
        let mut vals = vec![];
        for vtok in toks[1].split(' ') {
            if let Ok(val) = vtok.parse::<i64>() {
                vals.push(val);
            }
        }
        out.push(Eqn{ target, vals });
    }
    out
}

// Just break part1 now that I'm doing part2.
// I really don't need a v2 opcode set or compatibility or whatever
// for a toy/practice problem. :)
#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
    Cat,
}
fn apply(op: Op, lhs: i64, rhs: i64) -> i64 {
    match op {
        Op::Add => return lhs + rhs,
        Op::Mul => return lhs * rhs,
        Op::Cat => {
            let s = format!("{}{}", lhs, rhs);
            s.parse::<i64>().unwrap()
        }
    }
}
fn get_op(o: i64) -> Option<Op> {
    match o {
        0 => return Some(Op::Add),
        1 => return Some(Op::Mul),
        2 => return Some(Op::Cat),
        _ => return None,
    }
}

fn run_eqn(eqn: &Eqn, opcodes: &Vec<i64>) -> i64 {
    let mut ops = vec![];
    for code in opcodes {
        let op = get_op(*code);
        ops.push(op.unwrap());
    }
    let mut cur = eqn.vals[0];
    assert![eqn.vals.len() == ops.len() + 1];
    for idx in 0..ops.len() {
        cur = apply(ops[idx], cur, eqn.vals[idx + 1]);
    }
    cur
}

#[derive(Clone, Debug)]
struct OpCounter {
    ops: Vec<i64>
}

impl OpCounter {
    fn new(num_operands: usize) -> Self {
        Self{ops: vec![0; num_operands - 1]}
    }
    fn bump(&mut self) -> bool {
        let mut i = 0;
        while i < self.ops.len() {
            self.ops[i] += 1;
            if get_op(self.ops[i]).is_some() {
                return true;
            }
            self.ops[i] = 0;
            i += 1;
        }
        false
    }
}

fn solve(eqn: &Eqn) -> bool {
    // assign default opcodes for all operands
    // we'll iterate through all possibilities until we hit
    // the target or exhaust all combinations
    let mut oc = OpCounter::new(eqn.vals.len());
    loop {
        if run_eqn(&eqn, &oc.ops) == eqn.target {
            return true;
        }
        if !oc.bump() {
            return false;
        }
    }
}

fn part2(eqns: Eqns) -> i64 {
    let mut out = 0;
    for eqn in &eqns {
        if solve(eqn) {
            out += eqn.target;
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let eqns = parse(&buf);
    println!("part2: {}", part2(eqns.clone()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn eqns() {
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
        assert_eq![part2(e), 11387];
    }
}

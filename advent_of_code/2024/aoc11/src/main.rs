use std::io::{stdin, Read};
use std::mem;

type Stones = Vec<u64>;
fn parse(s: &str) -> Stones {
    let mut out = vec![];
    for t in s.split_whitespace() {
        let val = t.parse::<u64>();
        if let Ok(v) = val {
            out.push(v);
        }
    }
    out
}

fn num_digits(mut v: u64) -> usize {
    let mut out = 0;
    while v > 0 {
        v /= 10;
        out += 1;
    }
    out
}

fn run_one(s: &Stones, out: &mut Stones) {
    out.clear();
    for st in s {
        if *st == 0 {
            out.push(1 as u64);
            continue;
        } 
        let digits = num_digits(*st);
        if digits % 2 == 0 {
            let pivot = digits / 2;
            let scale : u64 = 10;
            let scale = (scale as u32).pow(pivot as u32);
            let lhs = *st / scale as u64;
            let rhs = *st % scale as u64;
            out.push(lhs);
            out.push(rhs);
            continue;
        }
        out.push(*st * 2024);
    }
}

fn run(buf: &str, lim: usize) -> usize {
    let mut s1 = parse(buf);
    let mut s2 : Stones = vec![];
    for _i in 0..lim {
        run_one(&s1, &mut s2);
        mem::swap(&mut s1, &mut s2);
    }
    s1.len()
}

fn part1(buf: &str) -> usize {
    run(buf, 25)
}

fn part2(buf: &str) -> usize {
    run(buf, 75)
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
    fn example1() {
        let s = "125 17";
        assert_eq![part1(s), 55312];
    }

    #[test]
    fn example2() {
    }
}

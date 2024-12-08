use std::io::{stdin, Read};

type Report = Vec<i32>;
type ReportsT = Vec<Report>;
fn parse(buf: &str) -> ReportsT {
    let mut out : ReportsT = vec![];
    for l in buf.split("\n") {
        let mut row = vec![];
        for tok in l.split(" ") {
            if let Ok(val) = tok.parse::<i32>() {
                row.push(val);
            }
        }
        if row.len() > 0 {
            out.push(row);
        }
    }
    out
}

fn is_safe(r: &Report) -> bool {
    let mut idx = 0;
    let mut dir = 0;
    while idx < r.len() - 1 {
        let diff = r[idx + 1] - r[idx];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        println!("{} {} [{} {}]", r[idx], r[idx+1], dir, diff);
        if dir > 0 && diff < 0 || dir < 0 && diff > 0 {
            return false;
        }
        idx += 1;
        if diff < 0 {
            dir = -1;
        } else {
            dir = 1;
        }
    }
    true
}

fn part1(r: ReportsT) -> u32 {
    let mut tot = 0;
    for l in r {
        if is_safe(&l) {
            tot += 1
        }
    }
    tot
}

fn dampen_floor(r: &Report) -> Vec<Report> {
    let mut out = vec![];
    let mut idx : usize = 0;
    while idx < r.len() {
        let mut next = vec![];
        for (vidx, val) in r.iter().enumerate() {
            if vidx == idx {
                continue;
            }
            next.push(*val);
        }
        out.push(next);
        idx += 1;
    }
    out
}

fn part2(r: ReportsT) -> u32 {
    let mut tot = 0;
    for l in r {
        let dampened_rows = dampen_floor(&l);
        let mut safe = false;
        for d in dampened_rows {
            if is_safe(&d) {
                safe = true;
                break;
            }
        }
        if safe {
            tot += 1;
        }
    }
    tot
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let reports = parse(&buf);
    println!("part1: {}", part1(reports.clone()));
    println!("part2: {}", part2(reports.clone()));
}

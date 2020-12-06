use std::io::{stdin, Read};

// extern crate Regex;
use regex::Regex;

struct Row {
    min: u32,
    max: u32,
    req: char,
    pw: String,
}

fn parse(buf: &str) -> Vec<Row> {
    // 5-7 l: llmlqmblllh
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut rows: Vec<Row> = vec![];
    for cap in re.captures_iter(&buf) {
        rows.push(Row {
            min: cap[1].parse::<u32>().unwrap(),
            max: cap[2].parse::<u32>().unwrap(),
            req: cap[3].parse::<char>().unwrap(),
            pw: cap[4].to_string(),
        });
    }
    rows
}

fn solve_part1(rows: &Vec<Row>) -> u32 {
    let mut num_compliant: u32 = 0;
    for r in rows {
        let mut req_count: u32 = 0;
        for c in r.pw.chars() {
            if c == r.req {
                req_count += 1;
            }
        }
        if req_count >= r.min && req_count <= r.max {
            num_compliant += 1
        }
    }
    num_compliant
}

fn solve_part2(rows: &Vec<Row>) -> u32 {
    let mut num_compliant: u32 = 0;
    for r in rows {
        let mut matches = 0;
        let indexes = vec![r.min as usize, r.max as usize];
        let chars: Vec<char> = r.pw.chars().collect();
        for idx in indexes {
            if chars[idx - 1] == r.req {
                matches += 1
            }
        }
        if matches == 1 {
            num_compliant += 1
        }
    }
    num_compliant
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let rows = parse(&buf);
    println!("part1: {} compliant", solve_part1(&rows));
    println!("part2: {} compliant", solve_part2(&rows));
}

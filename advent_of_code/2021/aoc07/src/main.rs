use std::io::{stdin, Read};

fn get_max(crabs: &Vec<u32>) -> u32 {
    let mut max = 0;
    for crab in crabs {
        if *crab > max {
            max = *crab;
        }
    }
    max
}

fn part1(crabs: &Vec<u32>) -> i64 {
    let max_pos = get_max(&crabs);

    let mut cheapest: i64 = 9999999;
    for pos in 0..max_pos {
        let mut candidate = 0;
        for crab in crabs {
            candidate += (*crab as i64 - pos as i64).abs();
        }
        if candidate < cheapest {
            cheapest = candidate;
        }
    }
    cheapest
}

fn part2(crabs: &Vec<u32>) -> i64 {
    let max_pos = get_max(&crabs);

    let mut cheapest: i64 = 99999999999999;
    for pos in 0..max_pos {
        let mut candidate: i64 = 0;
        for crab in crabs {
            let dist = (*crab as i64 - pos as i64).abs();
            for i in 0..dist {
                candidate += i + 1;
            }
        }
        if candidate < cheapest {
            cheapest = candidate;
        }
    }
    cheapest
}

fn parse(buf: &str) -> Vec<u32> {
    let mut out = vec![];
    for line in buf.split('\n') {
        for tok in line.split(',') {
            let val = tok.parse::<u32>();
            if !val.is_err() {
                out.push(val.unwrap());
            }
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let crabs = parse(&buf);

    println!("part1: {}", part1(&crabs));
    println!("part2: {}", part2(&crabs));
}

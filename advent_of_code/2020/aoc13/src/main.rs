use std::io::{stdin, Read};

struct Schedule {
    tstart: u64,
    ids: Vec<Option<u64>>,
}

fn next_run(id: u64, tnow: u64) -> u64 {
    let mut cur = tnow;
    loop {
        cur += 1;
        if cur % id == 0 {
            return cur;
        }
    }
}

impl Schedule {
    fn from_str(buf: &str) -> Self {
        let toks: Vec<&str> = buf.split_whitespace().collect();
        let tstart: u64 = toks[0].parse::<u64>().unwrap();
        let sched = toks[1];

        let mut ids = vec![];

        for tok in sched.split(",") {
            match tok {
                "x" => ids.push(None),
                id => ids.push(Some(id.parse::<u64>().unwrap())),
            }
        }
        Schedule {
            tstart: tstart,
            ids: ids,
        }
    }
}

fn part1(s: &Schedule) -> u64 {
    let mut next_id = 0;
    let mut wait_time = 0;
    let mut cur = s.tstart;
    while next_id == 0 {
        cur += 1;
        for id in &s.ids {
            if id.is_none() {
                continue;
            }
            let id = id.unwrap();
            if cur % id == 0 {
                next_id = id;
                wait_time = cur - s.tstart;
                break;
            }
        }
    }
    next_id * wait_time
}

fn part2(s: &Schedule) -> u64 {
    let mut cur = s.tstart;
    loop {
        cur += 1;

        let mut next_runs = vec![];
        for (idx, id) in s.ids.iter().enumerate() {
            if let Some(i) = id {
                next_runs.push(next_run(*i, cur));
            } else {
                next_runs.push(0);
            }

            for (idx, n) in next_runs.iter().enumerate() {
                if *n == 0 {
                    continue;
                }

                if cur + idx as u64 != *n {
                    break;
                }
            }
            // println!("{:?}", next_runs);
        }
        break;
    }
    cur
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let s = Schedule::from_str(&buf);
    println!("part1: {}", part1(&s));
    // TODO: my current 1000496 is too low
    println!("part2: {}", part2(&s));
}

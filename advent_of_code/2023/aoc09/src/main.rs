use std::io::{stdin, Read};

fn part1(buf: &str) -> i32 {
    let grid = parse(buf);
    let mut out = 0;
    for row in &grid {
        out += extrapolate(row);
    }
    out
}

fn part2(buf: &str) -> i32 {
    let grid = parse(buf);
    let mut out = 0;
    for row in &grid {
        out += extrapolate_prev(row);
    }
    out
}

fn parse(buf: &str) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for l in buf.split('\n') {
        let toks : Vec<&str> = l.split_whitespace().collect();
        if toks.len() == 0 {
            continue;
        }
        let mut row = vec![];
        for t in toks {
            row.push(t.parse::<i32>().unwrap());
        }
        out.push(row);
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(&buf));
    println!("part2: {}", part2(&buf));
}

fn build_series(v: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut series: Vec<Vec<i32>> = vec![];
    series.push(v.clone());

    // build series until we have one with all zero's
    let mut top = &series[0];
    while !top.iter().all(|n| *n == 0) {
        let mut cur = vec![];
        for i in 0..top.len() - 1 {
            cur.push(top[i+1] - top[i]);
        }
        series.push(cur);
        top = &series[series.len() - 1];
    }
    series
}

fn extrapolate(v: &Vec<i32>) -> i32 {
    let mut series = build_series(&v);

    // figure out the next value in each series by adding
    // the last value from each one above (if present)
    let mut delta = 0;
    let mut last = 0;
    for i in 0..series.len() {
        let idx = series.len() - i - 1;
        if idx + 1 < series.len() {
            let prev = &series[idx+1];
            delta = prev[prev.len() - 1];
        }

        let cur = &mut series[idx];
        last = cur[cur.len() - 1] + delta;
        cur.push(last);
    }
    last
}

fn extrapolate_prev(v: &Vec<i32>) -> i32 {
    let mut series = build_series(&v);

    // like above, but figuring previous values this time
    //
    // there's probably some magical way to extend the original
    // function to take functions that plug in the right behavior,
    // but honestly I'd rather just have a couple different functions. :)
    let mut delta = 0;
    let mut last = 0;
    for i in 0..series.len() {
        let idx = series.len() - i - 1;
        if idx + 1 < series.len() {
            let prev = &series[idx+1];
            delta = prev[0];
        }

        let cur = &mut series[idx];
        last = cur[0] - delta;
        cur.insert(0, last);
    }
    last
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extrapolate() {
   // 10  13  16  21  30  45  68
   //    3   3   5   9  15  23
   //      0   2   4   6   8
   //        2   2   2   2
   //          0   0   0
        let seq = vec![10,  13,  16,  21,  30,  45];
        assert_eq!(extrapolate(&seq), 68);
    }

    #[test]
    fn test_extrapolate_prev() {
        let seq = vec![10,  13,  16,  21,  30,  45];
        assert_eq!(extrapolate_prev(&seq), 5);
    }
}

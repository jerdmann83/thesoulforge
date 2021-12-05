use std::collections::VecDeque;
use std::io::{stdin, Read};

fn part1(buf: &str) {
    let mut last = i32::MIN;
    let mut count = 0;
    for l in buf.split('\n') {
        let num = l.parse::<i32>();
        if num.is_err() {
            continue;
        }

        let num = num.unwrap();
        if last == i32::MIN {
            last = num;
            continue;
        }

        if num > last {
            count += 1;
        }
        last = num;
    }
    println!("{}", count);
}

type WindowT = VecDeque<i32>;

struct Window {
    vals: VecDeque<i32>,
}

impl Window {
    fn new(slots: u32) -> Self {
        let mut out = VecDeque::new();
        for i in 0..slots {
            out.push_back(i32::MAX);
        }
        Self { vals: out }
    }

    fn is_complete(&self) -> bool {
        for i in &self.vals {
            if *i == i32::MAX {
                return false;
            }
        }
        return true;
    }

    fn insert(&mut self, num: i32) {
        self.vals.push_back(num);
        self.vals.pop_front();
    }

    fn sum(&self) -> i32 {
        let mut out = 0;
        for i in &self.vals {
            out += i;
        }
        out
    }
}

fn part2(buf: &str, window_size: u32) {
    let mut count = 0;
    let mut cur = Window::new(window_size);
    let mut last = Window::new(window_size);
    for l in buf.split('\n') {
        let num = l.parse::<i32>();
        if num.is_err() {
            continue;
        }

        let num = num.unwrap();
        cur.insert(num);
        if cur.is_complete() && last.is_complete() {
            if cur.sum() > last.sum() {
                count += 1;
            }
        }
        last.insert(num);
    }
    println!("{}", count);
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    part1(&buf);
    part2(&buf, 3);
}

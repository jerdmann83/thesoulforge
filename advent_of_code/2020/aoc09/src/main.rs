use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Xmas {
    seq: Vec<u64>,
    idx: usize,
    window_len: u64,
}

impl Xmas {
    fn from_str(s: &str, preamble_len: u64) -> Option<Self> {
        let mut seq: Vec<u64> = vec![];
        let mut i = 0;

        for l in s.split_whitespace() {
            if l.len() > 0 {
                seq.push(l.parse::<u64>().unwrap());
            }
        }
        if seq.len() < preamble_len as usize {
            return None;
        }
        return Some(Xmas {
            seq: seq,
            idx: preamble_len as usize,
            window_len: preamble_len,
        });
    }

    fn current(&self) -> u64 {
        return self.seq[self.idx];
    }

    fn window(&self) -> &[u64] {
        let begin = self.idx - self.window_len as usize;
        return &self.seq[begin..self.idx];
    }

    fn part1(&mut self) -> Option<u64> {
        while self.idx < self.seq.len() {
            let window = &self.window();
            // println!("{:?} {}", window, self.idx);
            let cur = &self.current();
            if !any_two(&self.window(), cur) {
                return Some(*cur);
            }
            self.idx += 1
        }
        None
    }

    fn part2(&mut self, goal: &u64) -> Option<&[u64]> {
        let mut sum: u64 = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        let end = self.seq.len();
        while x < end {
            while y <= end {
                sum = self.seq[x..y].iter().sum();
                if sum == *goal {
                    return Some(&self.seq[x..y]);
                } else if sum > *goal {
                    x += 1;
                    y = x + 1;
                } else {
                    y += 1;
                }
            }
        }
        None
    }
}

fn any_two(seq: &[u64], goal: &u64) -> bool {
    let mut x = 0;
    let mut y = 1;
    while y < seq.len() {
        if seq[x] + seq[y] == *goal {
            return true;
        }
        y += 1;
        if y == seq.len() {
            x += 1;
            y = x + 1;
        }
    }
    false
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut xm = Xmas::from_str(&buf, 25).unwrap();
    let goal = xm.part1();
    println!("part1: {:?}", goal);

    let gr = xm.part2(&goal.unwrap()).unwrap();
    println!(
        "part2: {}",
        gr.iter().min().unwrap() + gr.iter().max().unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let seq = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let mut xm = Xmas::from_str(&seq, 5).unwrap();
        assert_eq!(xm.part1(), Some(127));
    }
}

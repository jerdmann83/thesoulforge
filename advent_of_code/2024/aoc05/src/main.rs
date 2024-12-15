use std::io::{stdin, Read};
use std::collections::HashMap;
use std::collections::VecDeque;


type Run = Vec<u32>;
type Runs = Vec<Run>;
type Rules = HashMap<u32, Vec<u32>>;
#[derive(Clone, Debug)]
struct Pages {
    rules: Rules,
    runs: Runs,
}

impl Pages {
    fn from_str(s: &str) -> Self {
        let mut rules: Rules = HashMap::new();
        let mut runs = vec![];
        for (idx,chunk) in s.split("\n\n").enumerate() {
            match idx {
                0 => {
                    for l in chunk.split("\n") {
                        let toks : Vec<&str> = l.split("|").collect();
                        if toks.len() != 2 {
                            continue;
                        }
                        let lhs = toks[0].parse::<u32>().unwrap();
                        let rhs = toks[1].parse::<u32>().unwrap();
                        let targets = rules.entry(lhs).or_insert(vec![]);
                        targets.push(rhs);
                    }
                },
                1 => {
                    for l in chunk.split("\n") {
                        let toks : Vec<&str> = l.split(",").collect();
                        if toks.len() < 2 {
                            continue;
                        }
                        let mut run = vec![];
                        for tok in toks {
                            run.push(tok.parse::<u32>().unwrap());
                        }
                        runs.push(run);
                    }
                }
                _ => {}
            }
        }
        Self{rules, runs}
    }

    fn is_linked(&self, src: u32, dst: u32) -> bool {
        let mut frontier = VecDeque::from([src]);
        // we check if src is linked to dst by any number of hops in the ruleset
        // 1 might be connected to 2 in any case like:
        // 1 -> 2
        // 1 -> 3 -> 2
        while frontier.len() > 0 {
            let cur = frontier.pop_front().unwrap();
            // let next_dsts = self.rules.get(&cur).or_else(vec![]);
            if !self.rules.contains_key(&cur) {
                continue
            }
            let next_dsts = self.rules.get(&cur).unwrap();
            if next_dsts.len() == 0 {
                continue;
            }
            for next_dst in next_dsts {
                if *next_dst == dst {
                    return true;
                }
                frontier.push_back(*next_dst);
            }
        }
        false
    }

    fn is_valid(&self, run: &Run) -> bool {
        let mut li = 0;
        let mut ri = 1;
        while li + 1 < run.len() {
            println!("{} {}", ri, li);
            let lhs = run[li];
            let rhs = run[ri];
            if self.is_linked(rhs, lhs) {
                return false;
            }
            ri += 1;
            if ri == run.len() {
                li += 1;
                ri = li + 1;
            }
        }
        true
    }

    fn do_runs(&self) -> Runs {
        let mut out = vec![];
        for r in &self.runs {
            if self.is_valid(r) {
                out.push(r.clone());
            }
        }
        out
    }
}

fn part1(p: Pages) -> u32 {
    println!("{:?}", p);
    let mut out = 0;
    let rows = p.do_runs();
    for r in &rows {
        let idx = r.len() / 2;
        let val = r[idx];
        out += val;
    }
    out
}

fn part2(p: Pages) -> u32 {
    todo!();
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let pages = Pages::from_str(&buf);
    println!("part1: {}", part1(pages.clone()));
    // println!("part2: {}", part2(pages.clone()));
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn fake_fail() {
        let s = "11|22

22,11";
        let p = Pages::from_str(s);
        println!("{:?}", p);
        assert_eq![p.do_runs().len(), 0];
    }

    #[test]
    fn fake_success() {
        let s = "11|22
22|33

33,44,55
22,33,55
11,22,33";
        let p = Pages::from_str(s);
        println!("{:?}", p);
        assert_eq![p.do_runs().len(), 3];
    }

    #[test]
    fn actual() {
        let s = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let p = Pages::from_str(s);
        assert_eq![part1(p), 143];
    }
}

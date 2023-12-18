use std::io::{stdin, Read};
use std::collections::HashMap;

fn dir_idx(dir: char) -> usize {
    match dir {
        'L' => return 0,
        'R' => return 1,
        _ => unreachable!(),
    }
}

fn part1(buf: &str) -> u32 {
    let mut map = Map::from_str(buf);

    let mut cur = START;
    while cur != END {
        let dir = map.steps[map.step % map.steps.len()];
        let idx = dir_idx(dir);
        let n = &map.nodes[cur];
        cur = &n[idx];

        map.step += 1;
    }
    map.step as u32
}

fn part2(buf: &str) -> u32 {
    let mut map = Map::from_str(buf);
    let mut ghosts : Vec<String> = vec![];
    for (n,_) in &map.nodes {
        if n.ends_with("A") {
            ghosts.push(n.to_string());
        }
    }

    loop {
        let dir = map.steps[map.step % map.steps.len()];
        map.step += 1;

        let idx = dir_idx(dir);
        for gi in 0..ghosts.len() {
            let g = &mut ghosts[gi];
            *g = map.nodes[g][idx].clone();
        }

        if ghosts.iter().all(|g| g.ends_with("Z")) {
            break;
        }
    }

    map.step as u32
}

type Nodes = HashMap<String, Vec<String>>;
struct Map {
    nodes: Nodes,
    steps: Vec<char>,
    step: usize,
}

const START : &str = "AAA";
const END   : &str = "ZZZ";

impl Map {
    fn from_str(b: &str) -> Self {
        let sections : Vec<&str> = b.split("\n\n").collect();
        let steps : Vec<char> = sections[0].chars().collect();

        let mut nodes : Nodes = HashMap::new();
        for n in sections[1].split("\n") {
            let toks : Vec<&str> = n.split('=').collect();
            if toks.len() != 2 {
                continue;
            }

            let src = toks[0].trim();
            let dst_toks = toks[1].split(',');
            let mut dsts : Vec<String> = vec![];
            for d in dst_toks {
                dsts.push(d.trim().replace(&['(', ')'], ""));
            }
            nodes.insert(src.to_string(), dsts);
        }
        Self{nodes, steps, step: 0}
    }
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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".to_string();
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn part2example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".to_string();
        assert_eq!(part2(&input), 6);
    }

}

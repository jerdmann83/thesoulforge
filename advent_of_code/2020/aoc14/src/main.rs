use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

struct Port {
    mem: Vec<u64>,
    mask: Vec<char>,
}

// Hashmap-based storage for v2 mode.  Here we have to support the entire 36-bit
// address range, and given we're storing 64-bit values that means an allocation
// of ~64GB which is just nuts.
//
// We can get away with a "sparse" allocation of just the allocations made in
// the few hundred lines or so of input.  The great majority of address space is
// never touched.
struct Port_v2 {
    mem: HashMap<usize, u64>,
    mask: Vec<char>,
}

impl Port {
    fn new(mem_addresses: usize, mask_bits: usize) -> Self {
        Port {
            mem: vec![0; mem_addresses],
            mask: vec!['X'; mask_bits],
        }
    }
}

impl Port_v2 {
    fn new(mem_addresses: usize, mask_bits: usize) -> Self {
        Port_v2 {
            mem: HashMap::new(),
            mask: vec!['0'; mask_bits],
        }
    }

    fn set(&mut self, addr: usize, val: u64) {
        let addrs = self.v2_addresses();
        for addr in addrs {
            self.mem.insert(addr, val);
        }
    }

    fn v2_addresses(&self) -> Vec<usize> {
        let mut xs: Vec<usize> = vec![];
        let mut out: Vec<usize> = vec![];

        for (idx, b) in self.mask.iter().enumerate() {
            if *b == 'X' {
                xs.push(idx);
            }
        }

        if xs.len() == 0 {
            out.push(to_addr(&self.mask));
            return out;
        }

        let mut m = self.mask.clone();
        for idx in &xs {
            m[*idx] = '0';
        }

        for lim in 0..m.len() {
            for indexes in xs.iter().combinations(lim) {
                let mut m2 = m.clone();
                for idx in &indexes {
                    m2[**idx] = '1';
                }
                out.push(to_addr(&m2));
            }
        }

        out
    }
}

fn to_addr(c: &[char]) -> usize {
    let mut out = 0;
    for (idx, b) in c.iter().enumerate() {
        match *b {
            '0' => {}
            '1' => {
                out |= 1 << (c.len() - idx - 1);
            }
            _ => unreachable!(),
        }
    }
    out
}

impl Port {
    fn set(&mut self, addr: u64, val: u64) {
        let mut eval = val;
        for (idx, b) in self.mask.iter().enumerate() {
            match b {
                'X' => continue,
                '0' => {
                    let m = u64::MAX ^ (1 << (self.mask.len() - idx - 1));
                    eval &= m;
                }
                '1' => eval |= 1 << (self.mask.len() - idx - 1),
                _ => unreachable!(),
            }
        }
        self.mem[addr as usize] = eval;
    }
}

fn part1(buf: &str) -> u64 {
    let mut p = Port::new(65536, 36);

    for l in buf.split("\n") {
        let toks: Vec<&str> = l.split(" = ").collect();
        if toks.len() < 2 {
            continue;
        }

        match toks[0] {
            "mask" => {
                p.mask = toks[1].chars().collect();
            }
            _ => {
                let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
                for cap in re.captures_iter(&l) {
                    let addr = cap[1].parse::<u64>().unwrap();
                    let val = cap[2].parse::<u64>().unwrap();
                    p.set(addr, val);
                    break;
                }
            }
        }
    }

    let mut out = 0;
    for val in p.mem {
        out += val;
    }
    out
}

fn part2(buf: &str) -> u64 {
    let mut p = Port_v2::new(36, 36);

    for l in buf.split("\n") {
        let toks: Vec<&str> = l.split(" = ").collect();
        if toks.len() < 2 {
            continue;
        }

        match toks[0] {
            "mask" => {
                p.mask = toks[1].chars().collect();
            }
            _ => {
                let re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
                for cap in re.captures_iter(&l) {
                    let addr = cap[1].parse::<usize>().unwrap();
                    let val = cap[2].parse::<u64>().unwrap();
                    p.set(addr, val);
                    break;
                }
            }
        }
    }

    let mut out = 0;
    for (_, val) in p.mem {
        out += val;
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    // println!("part1: {}", part1(&buf));
    // 1253623125495 too low...
    println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn mem() {
        let mut p = Port::new(32, 32);
        p.set(0, 15);
        assert_eq!(p.mem[0], 15);

        p.mask[31] = '0';
        p.set(0, 15);
        assert_eq!(p.mem[0], 14);

        p.mask[31] = '1';
        p.mask[25] = '0';
        p.set(0, 0xFFFF);
        assert_eq!(p.mem[0], 65471);
    }

    #[test]
    fn v2() {
        let mut p = Port_v2::new(4, 4);
        p.mask[0] = '1';
        p.mask[2] = 'X';
        p.mask[3] = 'X';
        let addrs = p.v2_addresses();
        assert_eq!(addrs, vec![8, 10, 9, 11]);

        let expect: Vec<char> = "1011".chars().collect();
        assert_eq!(to_addr(&expect), 11);
    }
}

use itertools::Itertools;
use regex::Regex;
use std::io::{stdin, Read};

struct Port {
    mem: Vec<u64>,
    mask: Vec<char>,
}

impl Port {
    fn new(mem_addresses: usize, mask_bits: usize) -> Self {
        Port {
            mem: vec![0; mem_addresses],
            mask: vec!['X'; mask_bits],
        }
    }

    fn new_v2(mem_addresses: usize, mask_bits: usize) -> Self {
        Port {
            mem: vec![0; mem_addresses],
            mask: vec!['0'; mask_bits],
        }
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
        let mut cur = 1;
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

    fn set_v2(&mut self, addr: u64, val: u64) {
        let addrs = self.v2_addresses();
        for addr in addrs {
            println!("{:?}", addr);
            self.mem[addr] = val;
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

        println!("{:?}", out);
        out
    }
}

fn part1(buf: &str) -> u64 {
    let mut p = Port::new(65536, 36);

    for l in buf.split("\n") {
        let toks: Vec<&str> = l.split(" = ").collect();
        if toks.len() < 2 {
            continue;
        }

        // mask = X111000X0101100001000000100011X0000X
        // mem[4812] = 133322396
        // mem[39136] = 1924962
        match toks[0] {
            "mask" => {
                p.mask = toks[1].chars().collect();
                // println!("==> {:?}", p.mask);
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
    let mut p = Port::new_v2(65536, 36);

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
                    p.set_v2(addr, val);
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

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    println!("part1: {}", part1(&buf));
    // my addresses are huge for part2 for some reason
    // definitely doing something wrong...
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
        let mut p = Port::new_v2(4, 4);
        p.mask[0] = '1';
        p.mask[2] = 'X';
        p.mask[3] = 'X';
        let addrs = p.v2_addresses();
        assert_eq!(addrs, vec![8, 10, 9, 11]);

        let expect: Vec<char> = "1011".chars().collect();
        assert_eq!(to_addr(&expect), 11);
    }
}

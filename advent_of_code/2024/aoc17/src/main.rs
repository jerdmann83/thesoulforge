use std::io::{stdin, Read};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Device {
    ins: Vec<u32>,
    reg: [u32; 3],
    ip: usize,
}

const ADV : u32 = 0;
const BXL : u32 = 1;
const BST : u32 = 2;
const JNZ : u32 = 3;
const BXC : u32 = 4;
const OUT : u32 = 5;
const BDV : u32 = 6;
const CDV : u32 = 7;

impl Device {
    fn from_str(s: &str) -> Self {
        let chunks : Vec<&str> = s.split("\n\n").collect();
        assert![chunks.len() == 2];
        let mut reg : [u32; 3] = [0; 3];
        let mut rp = 0;
        for l in chunks[0].split('\n') {
            for tok in l.split(' ').rev() {
                reg[rp] = tok.parse::<u32>().unwrap();
                rp += 1;
                break;
            }
        }
        let ip = 0;

        let toks : Vec<&str> = chunks[1].split(' ').collect();
        let mut ins = vec![];
        for tok in toks[1].split(',') {
            ins.push(tok.trim().parse::<u32>().unwrap());
        }
        Self{ ins, reg, ip }
    }

    fn div(&mut self, reg: usize, arg: u32) {
        // numerator always from the A register
        let val = self.reg[0] / 2_u32.pow(arg);
    
        // write to the passed register
        self.reg[reg] = val;
    }

    fn combo(&self, arg: u32) -> u32 {
        if arg < 4 { arg } else { self.reg[arg as usize - 4] }
    }

    fn run(&mut self, out: &mut Vec<u32>) {
        while self.ip + 1 < self.ins.len() {
            let ins = self.ins[self.ip];
            let arg = self.ins[self.ip + 1];
            match ins {
                ADV => self.div(0, self.combo(arg)),
                BXL => self.reg[1] ^= arg,
                BST => self.reg[1] = self.combo(arg) % 8,
                JNZ => {
                    if self.reg[0] > 0 {
                        self.ip = arg as usize;
                        continue;
                    }
                },
                BXC => self.reg[1] ^= self.reg[2],
                OUT => out.push(self.combo(arg) % 8),
                BDV => self.div(1, self.combo(arg)),
                CDV => self.div(2, self.combo(arg)),
                _ => {},
            }
            self.ip += 2;
        }
    }

    fn dump(vals: &Vec<u32>) -> String {
        let mut out = String::new();
        for (i,v) in vals.into_iter().enumerate() {
            if i > 0 {
                out.push_str(&format!["{}", ',']);
            }
            out.push_str(&format!["{}", v]);
        }
        out
    }
}

fn part1(buf: &str) -> String {
    let mut d = Device::from_str(buf);
    let mut vals = vec![];
    d.run(&mut vals);
    Device::dump(&vals)
}

fn part2(buf: &str) -> u32 {
    let mut out = 1;
    let mut d = Device::from_str(buf);
    let mut vals = vec![];
    loop {
        d.ip = 0;
        d.reg[0] = out;
        d.reg[1] = 0;
        d.reg[2] = 0;
        d.run(&mut vals);
        if vals == d.ins {
            return out;
        }
        out += 1;
        if out % 1000000 == 0 {
            println!("{:?}", out);

        }
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
        let s = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq![part1(s), "4,6,3,5,6,3,5,2,1,0".to_string()];
    }

    #[test]
    fn example2() {
        let s = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq![part2(s), 117440];
    }
}

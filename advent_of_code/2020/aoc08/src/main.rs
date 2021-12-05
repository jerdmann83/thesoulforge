use std::fmt;
use std::io::{stdin, Read};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Option<Self> {
        let toks: Vec<&str> = s.split_whitespace().collect();
        if toks.len() != 2 {
            return None;
        }

        let sign = &toks[1][0..1];
        let num = &toks[1][1..];
        let mut val = num.parse::<i32>().unwrap();
        match sign {
            "-" => val *= -1,
            "+" => {}
            _ => return None,
        }

        match toks[0] {
            "nop" => return Some(Self::NOP(val)),
            "acc" => return Some(Self::ACC(val)),
            "jmp" => return Some(Self::JMP(val)),
            _ => return None,
        }
    }
}

#[derive(Debug)]
struct Cpu {
    acc: i32,
    ip: i32,
    icount: Vec<i32>,
    prog: Vec<Instruction>,
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "acc={} ip={} len={}",
            &self.acc,
            &self.ip,
            &self.prog.len()
        )
    }
}

impl Cpu {
    fn from_str(s: &str) -> Self {
        // nop +0
        // acc +1
        // jmp +4
        let mut prog: Vec<Instruction> = vec![];
        for l in s.split('\n') {
            if l.len() > 0 {
                prog.push(Instruction::from_str(l).unwrap());
            }
        }

        Cpu {
            acc: 0,
            ip: 0,
            icount: vec![0; prog.len()],
            prog: prog,
        }
    }

    fn run_until(&mut self) -> Option<i32> {
        while self.ip < self.prog.len() as i32 {
            let mut cur = &mut self.icount[self.ip as usize];
            let ins = &self.prog[self.ip as usize];
            // println!("{:?} i={} acc={} cur={}", ins, self.ip, self.acc, cur);
            if *cur == 1 {
                return Some(self.acc);
            }
            *cur += 1;
            match ins {
                Instruction::NOP(_) => {
                    self.ip += 1;
                }
                Instruction::JMP(x) => {
                    self.ip += x;
                }
                Instruction::ACC(x) => {
                    self.acc += x;
                    self.ip += 1;
                }
            }
        }
        None
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut cpu = Cpu::from_str(&buf);
    println!("part1: {:?}", &cpu.run_until());

    let len = cpu.prog.len();
    let mut out: Option<i32> = None;
    for i in 0..cpu.prog.len() {
        let mut cpu = Cpu::from_str(&buf);
        let ins: Instruction;
        match &cpu.prog[i] {
            Instruction::NOP(x) => ins = Instruction::JMP(*x),
            Instruction::JMP(x) => ins = Instruction::NOP(*x),

            Instruction::ACC(x) => ins = Instruction::ACC(*x),
        }
        // println!("{:?} {:?}", &cpu.prog[i], ins);
        cpu.prog[i] = ins;
        let res = cpu.run_until();
        if res.is_none() {
            out = Some(cpu.acc);
            break;
        }
    }

    println!("part2: {:?}", out);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let prog = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let mut cpu = Cpu::from_str(prog);
        assert_eq!(cpu.run_until(), Some(5));
    }
}

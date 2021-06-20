use std::mem;

mod q7;

const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn deconstruct_f32(f: f32) -> (u32, u32, u32) {
    let n_: u32 = unsafe { std::mem::transmute(f) };

    let sign = (n_ >> 31) & 1;
    let mantissa = (n_ >> 23) & 0xff;
    let fraction = 0b00000000_01111111_11111111_11111111 & n_;

    (sign, mantissa, fraction)
}

fn decode_f32_parts(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa: f32 = 1.0;
    for i in 0..23_u32 {
        let one_at_bit_i = 1 << i;
        if (one_at_bit_i & fraction) != 0 {
            mantissa += 2_f32.powf((i as f32) - 23.0);
        }
    }

    (signed_1, exponent, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
    sign * exponent * mantissa
}

struct CPU {
    registers: [u8; 16],
    memory: [u8; 4096],
    memory_ptr: usize,
    stack: [u16; 16],
    stack_ptr: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            memory_ptr: 0,
            stack: [0; 16],
            stack_ptr: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.memory_ptr += 2;

            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = ((opcode & 0x000f) >> 0) as u8;

            let nn = opcode & 0x0fff;
            let _kk = (opcode & 0x00ff) as u8;
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xe, 0xe) => self.ret(),
                (0x2, _, _, _) => self.call(nn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode 0x{:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = &mut self.stack_ptr;
        let stack = &mut self.stack;
        if *sp >= stack.len() {
            panic!("stack overflow!");
        }

        stack[*sp] = self.memory_ptr as u16;
        *sp += 1;
        self.memory_ptr = addr as usize;
    }

    fn ret(&mut self) {
        let sp = &mut self.stack_ptr;
        if *sp == 0 {
            panic!("stack underflow!");
        }

        *sp -= 1;
        self.memory_ptr = self.stack[*sp as usize] as usize;
    }

    fn read_opcode(&self) -> u16 {
        let mp = self.memory_ptr;
        let b1 = self.memory[mp] as u16;
        let b2 = self.memory[mp + 1] as u16;

        b1 << 8 | b2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, is_overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;
        if is_overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn memset(&mut self, pos: usize, op: u16) {
        let hi: u8 = ((op & 0xff00) >> 8) as u8;
        let lo: u8 = (op & 0x00ff) as u8;
        self.memory[pos] = hi;
        self.memory[pos + 1] = lo;
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        // adds
        let mut cpu = CPU::new();
        cpu.memset(0, 0x8014);
        cpu.memset(2, 0x8024);
        cpu.memset(4, 0x8034);
        cpu.registers[0] = 10;
        cpu.registers[1] = 5;
        cpu.registers[2] = 10;
        cpu.registers[3] = 5;
        cpu.run();
        assert_eq!(cpu.registers[0], 30);

        // overflow add
        let mut cpu = CPU::new();
        cpu.memset(0, 0x8014);
        let base = 12;
        cpu.registers[0] = base;
        cpu.registers[1] = 0xff;
        cpu.run();
        assert_eq!(cpu.registers[0], base - 1);
        assert_eq!(cpu.registers[0xF], 1);
    }

    #[test]
    fn test_call() {
        let mut cpu = CPU::new();
        // two calls
        cpu.memset(0, 0x2100);
        cpu.memset(2, 0x2100);

        // two add opcodes
        cpu.memset(0x100, 0x8014);
        cpu.memset(0x102, 0x8014);
        // ret
        cpu.memset(0x104, 0x00ee);

        cpu.registers[0] = 10;
        cpu.registers[1] = 5;
        cpu.run();
        assert_eq!(cpu.registers[0], 30);
    }
}

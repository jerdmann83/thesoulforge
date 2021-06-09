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
    current_op: u32,
    registers: [u8; 2],
}

impl CPU {
    fn new() -> Self {
        CPU {
            current_op: 0,
            registers: [0; 2],
        }
    }

    fn read_opcode(&self) -> u32 {
        self.current_op
    }

    fn run_one(&mut self) {
        let opcode = self.read_opcode();
        let c = ((opcode & 0xf000) >> 12) as u8;
        let x = ((opcode & 0x0f00) >> 8) as u8;
        let y = ((opcode & 0x00f0) >> 4) as u8;
        let d = ((opcode & 0x000f) >> 0) as u8;
        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode 0x{:04x}", opcode),
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.current_op = 0x8014;
    cpu.registers[0] = 10;
    cpu.registers[1] = 5;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.current_op = 0x8014;
        cpu.registers[0] = 10;
        cpu.registers[1] = 5;
        cpu.run_one();
        assert_eq!(cpu.registers[0], 15);
    }
}

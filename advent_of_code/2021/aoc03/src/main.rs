use std::io::{stdin, Read};

type ValsT = Vec<u32>;

fn extract_bit(val: u32, pos: u32) -> u32 {
    let rot = val.rotate_right(pos);
    let bit = rot & 0x1;
    bit
}

fn part1(vals: &ValsT, sz: u32) {
    let mut gamma: u32 = 0;

    for pos in 1..sz + 1 {
        let mut sum = 0;
        for val in vals {
            let bit = extract_bit(*val, sz - pos);
            sum += bit;
        }
        gamma = gamma.rotate_left(1);
        if sum * 2 >= vals.len() as u32 {
            gamma += 1;
        }
    }

    assert!(gamma < (1 as u32).rotate_left(13));
    let mut mask: u32 = 0;
    for _ in 0..sz {
        mask = mask.rotate_left(1);
        mask += 1;
    }
    let epsilon = (!gamma) & mask;
    println!("{:b} {:b} | {} {}", gamma, epsilon, gamma, epsilon);
    println!("{}", gamma * epsilon);
}

#[derive(PartialEq)]
enum Rating {
    Oxygen,
    CO2,
}

fn get_part2_rating(mut vals: ValsT, sz: u32, rating: Rating) -> u32 {
    let mut pos = 1;
    while vals.len() > 1 {
        let mut sum = 0;
        for val in &vals {
            let bit = extract_bit(*val, sz - pos);
            sum += bit;
        }

        let mut keep_bit = 0;
        if sum * 2 >= vals.len() as u32 {
            keep_bit = 1;
        }
        if rating == Rating::CO2 {
            keep_bit = if keep_bit == 0 { 1 } else { 0 };
        }

        let prev = vals.len();
        vals.retain(|&val| extract_bit(val, sz - pos) == keep_bit);
        assert!(prev > vals.len());
        pos += 1;
    }
    vals[0]
}

fn part2(vals: &ValsT, sz: u32) {
    let g1 = get_part2_rating(vals.clone(), sz, Rating::Oxygen);
    let g2 = get_part2_rating(vals.clone(), sz, Rating::CO2);
    println!("{}", g1 * g2);
}

fn convert(s: &str) -> u32 {
    let mut out: u32 = 0;
    let chars: Vec<char> = s.chars().collect();
    for c in chars {
        out = out.rotate_left(1);
        if c == '1' {
            out += 1;
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut vals = vec![];
    let mut sz = 0;
    for l in buf.split('\n') {
        if l.len() == 0 {
            continue;
        }
        if sz == 0 {
            sz = l.len() as u32;
        }

        vals.push(convert(l));
    }

    part1(&vals, sz);
    part2(&vals, sz);
}

mod test {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert("0100"), 4);
    }

    #[test]
    fn test_extract_bit() {
        assert_eq!(extract_bit(0b0111, 0), 1);
        assert_eq!(extract_bit(0b0111, 1), 1);
        assert_eq!(extract_bit(0b0111, 2), 1);
        assert_eq!(extract_bit(0b0111, 3), 0);
    }
}

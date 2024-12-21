use std::io::{stdin, Read};

#[derive(Debug)]
struct Disk {
    blocks: Vec<u32>,
}

const EMPTY : u32 = u32::MAX;

impl Disk {
    fn from_map(s: &str) -> Self {
        let mut blocks = vec![];
        // 12345
        for (idx,c) in s.chars().enumerate() {
            let val = char::to_digit(c, 10);
            if val.is_none() {
                break;
            }
            let val = val.unwrap();
            let fill : u32;
            if idx % 2 == 0 {
                let id = idx / 2;
                fill = id as u32;
            } else {
                fill = EMPTY;
            }
            for _ in 0..val {
                blocks.push(fill);
            }
        }
        Self { blocks }
    }
}

fn part1(buf: &str) -> usize {
    let mut d = Disk::from_map(buf);
    let mut li = 0;
    let mut ri = d.blocks.len() - 1;
    while li < ri {
        while d.blocks[li] != EMPTY {
            li += 1;
        }
        while d.blocks[ri] == EMPTY {
            ri -= 1;
        }
        if li > ri {
            break;
        }
        let tmp = d.blocks[ri];
        d.blocks[li] = tmp;
        d.blocks[ri] = EMPTY;
        li += 1;
        ri -= 1;
    }

    let mut out : usize = 0;
    for (idx,val) in d.blocks.iter().enumerate() {
        let val = *val;
        if val == EMPTY {
            break;
        }

        out += idx * val as usize;
    }
    out
}

fn part2(buf: &str) -> u32 {
    todo!();
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
        let s = "2333133121414131402";
        assert_eq![part1(s), 1928];
    }

    #[test]
    fn example2() {
    }
}

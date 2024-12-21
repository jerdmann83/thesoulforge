use std::io::{stdin, Read};

#[derive(Debug)]
struct Disk {
    blocks: Vec<char>,
}

const EMPTY : char = '.';

fn to_digit(c: char) -> Option<u32> {
    char::to_digit(c, 10)
}
fn from_digit(d: u32) -> Option<char> {
    char::from_digit(d, 10)
}

impl Disk {
    fn from_map(s: &str) -> Self {
        let mut blocks = vec![];
        // 12345
        for (idx,c) in s.chars().enumerate() {
            let val = to_digit(c);
            if val.is_none() {
                break;
            }
            let val = val.unwrap();
            let fill : char;
            if idx % 2 == 0 {
                let id = idx / 2;
                println!("{:?}", id);
                fill = from_digit(id as u32).unwrap();
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

fn part1(buf: &str) -> u32 {
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
        println!("swap {} {}", li, ri);
        let tmp = d.blocks[ri];
        d.blocks[li] = tmp;
        d.blocks[ri] = EMPTY;
        li += 1;
        ri -= 1;
    }

    let mut out = 0;
    for (idx,c) in d.blocks.iter().enumerate() {
        if *c == EMPTY {
            break;
        }

        let val = to_digit(*c).unwrap();
        out += idx as u32 * val;
    }
    out as u32
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

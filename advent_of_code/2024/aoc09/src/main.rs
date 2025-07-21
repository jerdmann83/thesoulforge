use std::io::{stdin, Read};

#[derive(Debug)]
struct Disk {
    blocks: Vec<u32>,
}

const EMPTY : u32 = u32::MAX;

#[derive(Debug, PartialEq)]
struct FatPtr {
    pos: usize,
    len: usize,
}

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


    fn find_block(&self, t: u32) -> Option<FatPtr> {
        let mut li = 0;
        let mut in_target = false;
        while li < self.blocks.len() {
            if self.blocks[li] == t {
                in_target = true;
                li += 1;
                continue;
            }
            li += 1;
        }
        if li >= self.blocks.len() {
            return None;
        }
        let mut ri = li + 1;
        while ri < self.blocks.len() {
            println!("ri {:?}", ri);
            if self.blocks[ri] != t {
                continue;
            }
            ri += 1;
        }
        let len = ri - li + 1;
        Some(FatPtr{ pos: li, len })
    }

    fn find_block_rev(&self, t: u32) -> Option<FatPtr> {
        let mut ri = (self.blocks.len() - 1) as i64;
        while ri > -1 {
            if self.blocks[ri as usize] != t {
                continue;
            }
            ri -= 1;
        }
        if ri < 0 {
            return None;
        }
        let mut li = ri;
        while li > -1 {
            if self.blocks[li as usize] != t {
                continue;
            }
            li += 1;
        }
        let len = (ri - li + 1) as usize;
        Some(FatPtr{ pos: li as usize, len })
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
        // TODO: probably a more elegant way to write this loop?
        // I don't love having the bounds-check condition in two places...
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

fn part2(buf: &str) -> usize {
    let mut d = Disk::from_map(buf);
    let mut li = 0;
    let mut ri = d.blocks.len() - 1;
    while li < ri {
    }

    let mut out = 0;
    out
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
    fn find() {
        let s = "2333133121414131402";
        let d = Disk::from_map(s);
        let lp = d.find_block(1).unwrap();
        let rp = d.find_block(1).unwrap();
        println!("{:?} {:?}", lp, rp);
        assert_eq![lp, rp];
    }

    #[test]
    fn example1() {
        let s = "2333133121414131402";
        assert_eq![part1(s), 1928];
    }

    #[test]
    fn example2() {
        let s = "2333133121414131402";
        assert_eq![part2(s), 2858];
    }
}

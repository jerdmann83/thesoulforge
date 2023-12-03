use std::io::{stdin, Read};

type Elf = Vec<u32>;

fn parse(buf: &str) -> Vec<Elf> {
    let mut out = vec![];
    for chunk in buf.split("\n\n") {
        let mut elf = vec![];
        for l in chunk.split('\n') {
            let l = l.trim();
            let val = l.parse::<u32>();
            if val.is_err() {
                println!("{:?}", val);
                continue;
            }
            elf.push(val.unwrap());
        }
        out.push(elf);
    }
    out
}

fn part1(buf: &str) -> u32 {
    let elves = parse(buf);
    let mut max = 0;
    for e in elves {
        let cur = e.into_iter().reduce(|a, b| a+b).unwrap();
        max = std::cmp::max(max, cur);
    }
    max
}

fn part2(buf: &str) -> u32 {
    let elves = parse(buf);
    let mut sums = vec![];
    for e in elves {
        let cur = e.into_iter().reduce(|a, b| a+b).unwrap();
        sums.push(cur);
    }
    sums.sort();
    let mut sum = 0;
    assert!(sums.len() > 2);
    for i in 0..3 {
        sum += sums[sums.len() - i - 1];
    }
    sum
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
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn example2() {
        todo!();
    }
}

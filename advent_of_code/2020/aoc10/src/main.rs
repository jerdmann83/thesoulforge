use std::fmt;
use std::io::{stdin, Read};

fn part1(adapters: &Vec<u64>) -> u64 {
    let mut cur = 0;
    let mut diffs: Vec<u64> = vec![0; 9];
    for a in adapters {
        diffs[(a - cur) as usize] += 1;
        cur = *a;
    }
    diffs[1] * diffs[3]
}

fn part2(adapters: &Vec<u64>) -> u64 {
    let mut x = 0;
    let mut y = 0;
    0
}

fn part2_impl(v: &[u64]) -> u64 {
    0
    // for a in adapters {

    // }

    // let mut diffs: Vec<u64> = vec![0; 9];
    // for a in adapters {
    //     diffs[(a - cur) as usize] += 1;
    //     cur = *a;
    // }
    // diffs[1] * diffs[3]
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut adapters: Vec<u64> = vec![];
    for l in buf.split_whitespace() {
        if l.len() > 0 {
            adapters.push(l.parse::<u64>().unwrap());
        }
    }
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    println!("{:?}", part1(&adapters));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let a = vec![1, 2, 5];
        // 1, 2, 5
        // 2, 5
        assert_eq!(part2(&a), 2);
    }
}

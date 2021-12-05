use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

type CharSet = HashSet<char>;

fn everyone(buf: &str) -> usize {
    let mut out: usize = 0;
    let mut group: Vec<CharSet> = vec![];
    let mut chars: CharSet = HashSet::new();

    for l in buf.split('\n') {
        if l.len() == 0 {
            for c in &chars {
                if group.iter().all(|p| p.contains(&c)) {
                    out += 1;
                }
            }
            chars.clear();
            group.clear();
            continue;
        }

        let mut cur = HashSet::new();
        for c in l.chars() {
            cur.insert(c);
            chars.insert(c);
        }
        group.push(cur);
    }
    out
}

fn anyone(buf: &str) -> usize {
    let mut out: usize = 0;
    let mut answers = HashSet::new();

    for l in buf.split('\n') {
        if l.len() == 0 {
            out += answers.len();
            answers.clear();
            continue;
        }

        for c in l.chars() {
            answers.insert(c);
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    println!("part1: {}", anyone(&buf));
    println!("part2: {}", everyone(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(everyone("a\na\na\n"), 1);
        assert_eq!(everyone("a\nb\ncde\n"), 0);
        assert_eq!(everyone("abcd\nbcd\nbcde\n"), 3);
    }
}

use std::io::{stdin, Read};

fn get_points_part1(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_points_part2(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn get_opener(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

// todo: more elegant thing to represent the symmetrical nature between
// openers and closers...
fn get_closer(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn is_open(c: char) -> bool {
    get_closer(c).is_some()
}

type Nav = Vec<Vec<char>>;

fn parse(buf: &str) -> Nav {
    let mut out = vec![];
    for l in buf.split('\n') {
        let row: Vec<char> = l.chars().collect();
        out.push(row);
    }
    out
}

struct ScanResult {
    illegal: Option<char>,
    stack: Vec<char>,
}

impl ScanResult {
    fn from_illegal(ill: char) -> Self {
        ScanResult {
            illegal: Some(ill),
            stack: vec![],
        }
    }

    fn from_stack(stack: Vec<char>) -> Self {
        ScanResult {
            illegal: None,
            stack: stack,
        }
    }
}

fn scan(row: &[char]) -> ScanResult {
    let mut toks: Vec<char> = vec![];
    for c in row {
        let pair = get_opener(*c);
        if let Some(expect) = pair {
            let open = toks.pop();
            if open.is_none() || open.unwrap() != expect {
                return ScanResult::from_illegal(*c);
            }
        }
        if is_open(*c) {
            toks.push(*c);
        }
    }
    return ScanResult::from_stack(toks);
}

fn part1(n: &Nav) -> u32 {
    let mut out = 0;
    for row in n {
        let result = scan(&row);
        out += get_points_part1(result.illegal.unwrap_or(' '));
    }
    out
}

fn part2(n: &Nav) -> u32 {
    for row in n {
        let mut out = 0;
        let result = scan(&row);
        if result.illegal.is_some() {
            continue;
        }
        println!("{:?}", result.stack);
        for tok in result.stack.rev() {
            out *= 5;
            if let Some(closer) = get_closer(tok) {
                out += get_points_part2(closer);
            }
        }
        println!("{:?}", out);
    }
    0
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf);
    let nav = parse(&buf);
    println!("part1: {:?}", part1(&nav));
    println!("part2: {:?}", part2(&nav));
}

mod test {
    use super::*;
    #[test]
    fn test_illegal() {
        let row: Vec<char> = "{([])}".chars().collect();
        assert!(scan(&row).illegal.is_none());
        let row: Vec<char> = "{([)]}".chars().collect();
        assert!(scan(&row).illegal == Some(')'));
    }
}

use crate::substring;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum Action {
    Add,
    Remove,
}

#[derive(Debug, PartialEq)]
struct Diff {
    lhs: usize,
    rhs: usize,
    action: Action,
}
impl Diff {
    pub fn new(lhs: usize, rhs: usize, action: Action) -> Self {
        Self { lhs, rhs, action }
    }
}
type DiffResult = Vec<Diff>;

fn diff_impl(lhs: &str, rhs: &str) -> DiffResult {
    let mut lit = lhs.split("\n");
    let mut rit = rhs.split("\n");
    let mut ll = lit.next();
    let mut rl = rit.next();
    let mut out: DiffResult = vec![];
    while ll.is_some() && rl.is_some() {
        let ln = ll.unwrap();
        let rn = rl.unwrap();
        println!("{} {}", ln, rn);
        let lcs = substring::subsequence(ln, rn);
        if lcs == ln.len() {
            ll = lit.next();
            rl = rit.next();
            continue;
        }
        // out.push(Diff::new(ln.to_string(), rn.to_string(), Action::Add));

        ll = lit.next();
        rl = rit.next();
    }
    out
}

fn diff(args: &[String]) {
    if args.len() < 2 {
        eprintln!("expect: [file1] [file2]");
        todo!();
    }

    let mut lbuf = String::new();
    let mut rbuf = String::new();
    let mut lf = File::open(&args[0]).unwrap();
    let mut rf = File::open(&args[1]).unwrap();
    lf.read_to_string(&mut lbuf);
    rf.read_to_string(&mut rbuf);
    let res = diff_impl(&lbuf, &rbuf);
    println!("{:?}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let f1 = "Line one.\n";
        let f2 = "Line one.\nLine two.\n";
        let out = diff_impl(&f1, &f2);
        assert_eq!(out[0], Diff::new(1, 1, Action::Add));
    }
}

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;

mod diff;
mod substring;
mod try_iter;

fn lookup(args: &[String]) {
    let mut buf = String::new();
    let mut f = File::open("/usr/share/dict/words").unwrap();
    f.read_to_string(&mut buf).unwrap();

    let mut words = vec![];
    let re = Regex::new(r"^[a-z]+$").unwrap();
    for l in buf.split('\n') {
        if re.is_match(l) {
            words.push(l.to_string());
        }
    }

    let mut max = 0;
    let mut closest = vec![];
    for word in args {
        for i in 0..words.len() {
            let cur = substring::subsequence(&word, &words[i]);
            if cur > max {
                closest.clear();
                max = cur;
            }
            if cur == max {
                max = cur;
                closest.push(words[i].to_string());
            }
        }
        println!(
            "top {} words by subsequence (score {}):",
            closest.len(),
            max
        );
        for c in &closest {
            println!("  {}", c);
        }

        println!();
        let mut vals = vec![];
        let mut max = 0;
        for c in &closest {
            let cur = substring::substring(&word, c);
            if cur.len() > max {
                max = cur.len();
                vals.clear();
            }
            if cur.len() == max {
                vals.push(c);
            }
        }
        println!("top {} words by substring (score {}):", vals.len(), max);
        for c in &vals {
            println!("  {}", c);
        }
    }
}

fn bin(vals: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = vals.len() - 1;
    while l <= r {
        let mid = (l + r) / 2;
        let cur = vals[mid];
        if cur == target {
            return mid as i32;
        }
        if cur < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    return -1;
}

pub fn main() {
    let in_args: Vec<String> = env::args().collect();
    if in_args.len() < 2 {
        eprintln!("expect: [mode] [args...]");
        todo!();
    }

    let mode = &in_args[1];
    let args = &in_args[2..];

    // match mode.as_ref() {
    //     "lookup" => lookup(args),
    //     "diff" => diff.diff(args),
    //     &_ => todo!(),
    // }
}

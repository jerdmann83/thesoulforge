use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;

mod substring;

pub fn main() {
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
    let mut first = true;
    for word in env::args() {
        if first {
            first = false;
            continue;
        }
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

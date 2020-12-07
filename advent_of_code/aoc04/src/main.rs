use regex::Regex;
use std::collections::HashMap;
use std::io::{stdin, Read};

//     byr (Birth Year) - four digits; at least 1920 and at most 2002.
//     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//     hgt (Height) - a number followed by either cm or in:
//         If cm, the number must be at least 150 and at most 193.
//         If in, the number must be at least 59 and at most 76.
//     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//     pid (Passport ID) - a nine-digit number, including leading zeroes.
//     cid (Country ID) - ignored, missing or not.

type MapT = HashMap<String, String>;
type RuleMapT = HashMap<String, fn(&str) -> bool>;

struct Validator {
    rules: RuleMapT,
}

impl Validator {
    fn new() -> Self {
        let mut rules: RuleMapT = HashMap::new();
        rules.insert("byr".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 1919 && year <= 2002;
            }
            return false;
        });
        rules.insert("iyr".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 2010 && year <= 2020;
            }
            return false;
        });
        rules.insert("eyr".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 2020 && year <= 2030;
            }
            return false;
        });
        rules.insert("hgt".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{2,3})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 1919 && year <= 2002;
            }
            return false;
        });
        rules.insert("hcl".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 1919 && year <= 2002;
            }
            return false;
        });
        rules.insert("ecl".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 1919 && year <= 2002;
            }
            return false;
        });
        rules.insert("pid".to_string(), |s: &str| {
            let re = Regex::new(r"(\d{4})").unwrap();
            for cap in re.captures_iter(&s) {
                let year = cap[1].parse::<u32>().unwrap();
                return year >= 1919 && year <= 2002;
            }
            return false;
        });
        Validator { rules: rules }
    }

    fn is_valid(&self, m: &MapT) -> bool {
        for (key, rule) in self.rules.iter() {
            if !m.contains_key(key) {
                return false;
            } else if !rule(&m[key]) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    // no hashmap literals in rust? not sure either way yet
    // rules.insert("iyr".to_string());
    // rules.insert("eyr".to_string());
    // rules.insert("hgt".to_string());
    // rules.insert("hcl".to_string());
    // rules.insert("ecl".to_string());
    // rules.insert("pid".to_string());
    // rules.insert("cid".to_string());

    let mut valid = 0;
    let mut cur: MapT = HashMap::new();
    let v = Validator::new();

    for l in buf.split('\n') {
        // empty line indicates end of record
        if l.len() == 0 {
            if v.is_valid(&cur) {
                valid += 1
            }
            cur = HashMap::new();
            continue;
        }
        for chunk in l.split_whitespace() {
            let toks: Vec<&str> = chunk.split(':').collect();
            if toks.len() < 2 {
                continue;
            }
            cur.insert(toks[0].to_string(), toks[1].to_string());
        }
    }
    println!("{}", valid);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid() {
        let v = Validator::new();
        let mut m: MapT = HashMap::new();
        // 1920-2002 valid
        m.insert("byr".to_string(), "1950".to_string());
        assert!(v.is_valid(&m));

        m.insert("byr".to_string(), "2080".to_string());
        assert!(!v.is_valid(&m));

        let mut m: MapT = HashMap::new();
        assert!(!v.is_valid(&m));
    }
}

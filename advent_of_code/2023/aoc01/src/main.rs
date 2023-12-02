use std::io::{stdin, Read};

fn part1(buf: String) -> u32 {
    let mut vals = vec![];
    for l in buf.split('\n') {
        let mut digits = vec![];
        for c in l.chars() {
            let val = c.to_digit(10);
            match val {
                Some(x) => digits.push(x),
                None => {},
            }
        }
        if digits.len() < 1 {
            continue;
        }
        vals.push((digits[0] * 10) + digits[digits.len() - 1]);
    }
    return vals.into_iter().reduce(|a, b| a + b).unwrap();
}

const NUMBERS : [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn digit_from_str(s: &str) -> (Option<u8>, usize) {
    let mut val = 1;
    for n in NUMBERS {
        if s == n {
            return (Some(val), s.len());
        }
        val += 1;
    }

    (None, 0)
}

fn part2(buf: String) -> u32 {
    let mut vals = vec![];
    for l in buf.split('\n') {
        let mut digits = vec![];
        for mut i in 0..l.len() {
            let c = l.get(i..i).unwrap();
            let val = c.parse::<u32>();
            match val {
                Result::Ok(x) => digits.push(x),
                _ => {
                    let remain : &str = l.get(i..).unwrap();
                    let result = digit_from_str(remain);
                    match result {
                        ( Some(x), len ) => {
                            digits.push(x as u32);
                            i += len;
                        },
                            _ => {},
                    }
                }
            }
        }
        if digits.len() < 1 {
            continue;
        }
        vals.push((digits[0] * 10) + digits[digits.len() - 1]);
    }
    return vals.into_iter().reduce(|a, b| a + b).unwrap();
}


fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part1: {}", part1(buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();
        assert!(part1(input) == 142);
    }

    #[test]
    fn example2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".to_string();
        assert!(part2(input) == 281);
    }
}

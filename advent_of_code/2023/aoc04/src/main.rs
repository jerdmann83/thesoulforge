use std::io::{stdin, Read};

#[derive(Debug)]
struct Card {
    num: usize,
    winners: Vec<u32>,
    numbers: Vec<u32>,
    new: bool,
}

impl Card {
    fn get_matches(&self) -> u32 {
        let mut out = 0;
        for w in &self.winners {
            for n in &self.numbers {
                if w == n {
                    out += 1;
                }
            }
        }
        out
    }

    fn get_score(&self) -> u32 {
        let matches = self.get_matches();
        if matches == 0 {
            return 0;
        }

        let out = (2 as u32).pow(matches - 1);
        out
    }
}

type Cards = Vec<Card>;
fn parse(buf: &str) -> Cards {
    let mut out : Cards = vec![];
    for l in buf.split('\n') {
        let top_toks : Vec<&str> = l.split(':').collect();
        if top_toks.len() != 2 {
            continue;
        }
        let left_toks : Vec<&str> = top_toks[0].split_whitespace().collect();
        if left_toks.len() != 2 {
            continue;
        }
        let num = left_toks[1].parse::<usize>().unwrap();

        let right_toks : Vec<&str> = top_toks[1].split('|').collect();
        if right_toks.len() != 2 {
            continue;
        }
        let mut winners = vec![];
        let mut numbers = vec![];
        for n in right_toks[0].split_whitespace() {
            winners.push(n.parse::<u32>().unwrap());
        }
        for n in right_toks[1].split_whitespace() {
            numbers.push(n.parse::<u32>().unwrap());
        }
        out.push(Card{ num, winners, numbers, new: true });
    }
    out
}

fn part1(buf: &str) -> u32 {
    let cards = parse(buf);
    let mut out = 0;
    for c in cards {
        out += c.get_score()
    }
    out
}

fn part2(buf: &str) -> u32 {
    let cards = parse(buf);
    let mut counts = vec![0; cards.len()];
    for idx in 0..cards.len() {
        let card = &cards[idx];

        let slot = &mut counts[idx];
        *slot += 1;
        let repeat = *slot;
        println!("{}: {} times", idx, repeat);
        for _ in 0..repeat {
            let m = card.get_matches() as usize;
            for match_offset in 0..m {
                let next_idx  = idx + match_offset + 1;
                if next_idx >= cards.len() {
                    break;
                }
                counts[next_idx] += 1;
            }
        }
    }
    let mut out = 0;
    for count in counts {
        out += count;
    }
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

    const INPUT : &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 30);
    }
}

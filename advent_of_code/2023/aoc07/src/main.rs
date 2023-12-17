use std::io::{stdin, Read};
use std::collections::HashMap;
use std::cmp::Ordering;

const CARDS : [char; 13] = [
    'A',
    'K',
    'Q',
    'J',
    'T',
    '9',
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2' ];

fn card_val(c: char) -> usize {
    for idx in 0..CARDS.len() {
        if c == CARDS[idx] {
            return CARDS.len() - idx;
        }
    }
    unreachable!();
}

#[derive(Clone, Copy, Debug)]
enum HandType {
    FiveK,
    FourK,
    FullH,
    ThreeK,
    TwoP,
    Pair,
    High,
}

fn hand_type_to_val(h: HandType) -> u32 {
    match h {
        HandType::FiveK => return 6,
        HandType::FourK => return 5,
        HandType::FullH => return 4,
        HandType::ThreeK => return 3,
        HandType::TwoP => return 2,
        HandType::Pair => return 1,
        HandType::High => return 0,
    }
}

type Cards = Vec<char>;
fn hand_type(cards: &Cards) -> HandType {
    let mut vals : HashMap<char, u32> = HashMap::new();
    for c in cards {
        if !vals.contains_key(c) {
            vals.insert(*c, 0);
        }
        *vals.get_mut(c).unwrap() += 1;
    }
    match vals.len() {
        1 => return HandType::FiveK,
        2 => {
            for (_c, num) in vals {
                if num == 4 {
                    return HandType::FourK;
                } else if num == 3 {
                    return HandType::FullH;
                }
            }
            unreachable!();
        },
        3 => {
            for (_c, num) in vals {
                if num == 3 {
                    return HandType::ThreeK;
                }
            }
            return HandType::TwoP;
        },
        4 => return HandType::Pair,
        5 => return HandType::High,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: u32,
    htype: HandType,
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Self {
        let htype = hand_type(&cards);
        Self{cards, bid, htype}
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let lval = hand_type_to_val(self.htype);
        let rval = hand_type_to_val(rhs.htype);
        let diff = lval as i32 - rval as i32;

        if diff < 0 {
            return Some(Ordering::Less);
        } else if diff > 0 {
            return Some(Ordering::Greater);
        }

        for i in 0..self.cards.len() {
            let lval = card_val(self.cards[i]);
            let rval = card_val(rhs.cards[i]);
            let diff = lval as i32 - rval as i32;
            if diff < 0 {
                return Some(Ordering::Less);
            } else if diff > 0 {
                return Some(Ordering::Greater);
            }
        }
        unreachable!();
    }
}

impl PartialEq for Hand {
    fn eq(&self, rhs: &Self) -> bool {
        self.cards == rhs.cards
    }

    fn ne(&self, rhs: &Self) -> bool {
        !(*self == *rhs)
    }
}

// Apparently this bit is a signal to the compiler that the equality
// relationship defined above is a total ordering and not a partial one.
// Eq has no methods of its own?
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, rhs: &Self) -> Ordering {
        // partial ordering implemented above provides a total ordering
        // already :)
        self.partial_cmp(rhs).unwrap()
    }
}

fn parse(b: &str) -> Vec<Hand> {
    let mut out = vec!();
    for l in b.split('\n') {
        let toks : Vec<&str> = l.split_whitespace().collect();
        if toks.len() != 2 {
            continue;
        }
        let cards: Vec<char> = toks[0].chars().collect();
        let bid = toks[1].parse::<u32>().unwrap();
        out.push(Hand::new(cards, bid));
    }
    out
}

fn part1(buf: &str) -> u32 {
    let mut hands = parse(buf);
    hands.sort();
    let mut out = 0;
    for i in 0..hands.len() {
        let score = (i as u32 + 1) * hands[i].bid;
        out += score;
    }
    out
}

fn part2(buf: &str) -> u32 {
    todo!();
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

    const INPUT : &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn cmp() {
        let hands = parse("AAA22 1
22JJ3 2
AAJJ3 3
23455 4");
        println!("{:?}", hands);
        assert!(hands[0] > hands[1]);
        assert!(hands[2] > hands[1]);
        assert!(hands[2] > hands[3]);
    }

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 5905);
    }
}

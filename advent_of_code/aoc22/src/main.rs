use std::collections::HashSet;
use std::io::{stdin, Read};

type Deck = Vec<u32>;

fn parse(buf: &str) -> Vec<Deck> {
    let mut decks: Vec<Deck> = vec![];
    let mut cur: &mut Deck;
    for chunk in buf.split("Player") {
        if chunk.len() == 0 {
            continue;
        }
        let mut deck = vec![];
        for tok in chunk.split('\n') {
            let val = tok.parse::<u32>().unwrap_or(0);
            if val == 0 {
                continue;
            }
            deck.push(val);
        }
        decks.push(deck);
    }
    decks
}

fn score(deck: &Deck) -> u32 {
    let mut score = 0;
    for i in 0..deck.len() {
        let mul = deck.len() - i;
        score += deck[i] * mul as u32;
    }
    score
}

fn part1(decks: &Vec<Deck>) {
    assert!(decks.len() == 2);
    let mut d1 = decks[0].clone();
    let mut d2 = decks[1].clone();
    while d1.len() > 0 && d2.len() > 0 {
        let win: &mut Deck;
        let lose: &mut Deck;
        if d1[0] > d2[0] {
            win = &mut d1;
            lose = &mut d2;
        } else {
            lose = &mut d1;
            win = &mut d2;
        }
        let mut cards = vec![];
        cards.push(lose.remove(0));
        cards.push(win.remove(0));
        win.push(cards.pop().unwrap());
        win.push(cards.pop().unwrap());
    }
    let winner: &Deck;
    if d1.len() > 0 {
        winner = &d1;
    } else {
        winner = &d2;
    }
    println!("part1: {}", score(&winner));
}

fn part2(decks: &Vec<Deck>, game: usize) -> usize {
    assert!(decks.len() == 2);
    let mut d1 = decks[0].clone();
    let mut d2 = decks[1].clone();

    let mut winidx;
    let mut seen: HashSet<Vec<Deck>> = HashSet::new();
    let mut round = 0;
    while d1.len() > 0 && d2.len() > 0 {
        // infinite loop prevention check
        let subdecks = vec![d1[1..].to_vec(), d2[1..].to_vec()];
        if !seen.insert(subdecks.to_vec()) {
            if game == 1 {
                println!("breaker part2: {:?} {:?} {}", d1, d2, score(&d1));
                return 0;
            }
            return 1;
        }

        round += 1;
        println!("-- Round {} (Game {}) --", round, game);
        // println!("{:?}", d1);
        // println!("{:?}", d2);
        let c1 = d1[0];
        let c2 = d2[0];

        if c1 < d1.len() as u32 && c2 < d2.len() as u32 {
            winidx = part2(&subdecks, game + 1);
            assert!(winidx < decks.len());
        } else {
            if c1 > c2 {
                winidx = 0;
            } else {
                winidx = 1;
            }
        }

        let win: &mut Deck;
        let lose: &mut Deck;
        if winidx == 0 {
            win = &mut d1;
            lose = &mut d2;
        } else {
            lose = &mut d1;
            win = &mut d2;
        }
        let mut cards = vec![];
        cards.push(lose.remove(0));
        cards.push(win.remove(0));
        win.push(cards.pop().unwrap());
        win.push(cards.pop().unwrap());
    }

    // original root game?
    if game == 1 {
        let winner: &Deck;
        if d1.len() > 0 {
            winner = &d1;
        } else {
            winner = &d2;
        }
        println!("part2: {:?} {:?} {}", d1, d2, score(&winner));
        return 0;
    }

    // otherwise subgame result
    if d1.len() > 0 {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let decks = parse(&buf);
    // part1(&decks);
    part2(&decks, 1);
}

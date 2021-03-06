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
    let mut cur = 1;
    for i in 0..deck.len() {
        let mul = deck.len() - i;
        score += deck[i] * mul as u32;
    }
    score
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let decks = parse(&buf);
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
    println!("{:?} {}", d1, score(&d1));
    println!("{:?} {}", d2, score(&d2));
}

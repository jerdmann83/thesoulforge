use std::io::{stdin, Read};

fn solve(mut fishes: Vec<u8>, days: u16) -> usize {
    let mut new_fishes = vec![];
    for _day in 0..days {
        new_fishes.clear();
        println!("day {:?}", _day);
        for i in 0..fishes.len() {
            let fish = &mut fishes[i];
            if *fish == 0 {
                *fish = 6;
                new_fishes.push(8);
            } else {
                *fish -= 1;
            }
        }
        fishes.append(&mut new_fishes);
    }
    fishes.len()
}

fn parse(buf: &str) -> Vec<u8> {
    let mut out = vec![];
    for line in buf.split('\n') {
        for tok in line.split(',') {
            let val = tok.parse::<u8>();
            if !val.is_err() {
                out.push(val.unwrap());
            }
        }
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let fishes = parse(&buf);

    for days in vec![80, 256] {
        let num = solve(fishes.clone(), days);
        println!("{} days: {}", days, num);
    }
}

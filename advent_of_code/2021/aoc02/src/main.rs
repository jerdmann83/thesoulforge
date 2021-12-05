use std::collections::VecDeque;
use std::io::{stdin, Read};

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    horiz: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            horiz: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn part1(moves: &Vec<Move>) {
    let mut p = Position::new();
    for m in moves {
        match m {
            Move::Up(val) => p.depth -= val,
            Move::Down(val) => p.depth += val,
            Move::Forward(val) => p.horiz += val,
        }
    }
    println!("{:?}", p.horiz * p.depth);
}

fn part2(moves: &Vec<Move>) {
    let mut p = Position::new();
    for m in moves {
        match m {
            Move::Up(val) => p.aim -= val,
            Move::Down(val) => p.aim += val,
            Move::Forward(val) => {
                p.horiz += val;
                p.depth += p.aim * val;
            }
        }
    }
    println!("{:?}", p.horiz * p.depth);
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut moves: Vec<Move> = vec![];
    for l in buf.split('\n') {
        let tokens: Vec<&str> = l.split(' ').collect();
        if tokens.len() < 2 {
            continue;
        }
        let cur: Option<Move>;
        let val = tokens[1].parse::<i32>().unwrap();
        match tokens[0] {
            "up" => cur = Some(Move::Up(val)),
            "down" => cur = Some(Move::Down(val)),
            "forward" => cur = Some(Move::Forward(val)),
            _ => cur = None,
        }
        if cur.is_none() {
            continue;
        }
        moves.push(cur.unwrap());
    }

    part1(&moves);
    part2(&moves);
}

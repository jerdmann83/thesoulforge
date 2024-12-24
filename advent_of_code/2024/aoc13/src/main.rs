use std::io::{stdin, Read};
use util::Point;
use std::collections::HashMap;

type Buttons = HashMap<char, Point>;
#[derive(Debug)]
struct Game {
    buttons: Buttons,
    prize: Point,
}

fn parse(s: &str) -> Vec<Game> {
    let mut out = vec![];
    for c in s.split("\n\n") {
        let mut buttons : Buttons = HashMap::new();
        let mut prize = Point::new(-1, -1);
        for l in c.split("\n") {
            let toks : Vec<&str> = l.split(":").collect();
            let tt : Vec<&str> = toks[0].split(' ').collect();
            let ttype = tt[0];
            println!("{:?} {:?}", ttype, toks);
            let toks : Vec<&str> = toks[1].split(" ").collect();
            match ttype {
                "Button" => {
                    for pt in &toks[1..] {
                        println!("{:?}", pt);
                        // buttons[toks[1].chars()[0]] = 
                    }
                }
                "Prize" => {
                    // for pt in &toks[2..] {
                    //     println!("{:?}", pt);
                    //     // buttons[toks[1].chars()[0]] = 
                    // }
                    // continue;
                }
                _ => todo!(),
            }
        }
        assert!(prize.x > -1);
        assert!(buttons.len() > 0);
        out.push(Game{ buttons, prize });
    }
    out
}

fn part1(buf: &str) -> u32 {
    let games = parse(buf);
    println!("{:?}", games);
    todo!();
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

    #[test]
    fn example1() {
        let s = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq![part1(s), 480];
    }

    #[test]
    fn example2() {
    }
}

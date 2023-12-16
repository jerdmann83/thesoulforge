use std::io::{stdin, Read};

fn get_wins(r: &Race) -> usize {
    let mut wins = 0;
    for i in 1..r.time-1 {
        let remain = r.time - i;
        let dist = remain * i;
        if dist > r.dist {
            wins += 1;
        }
    }
    wins
}

fn part1(buf: &str) -> usize {
    let races = parse(buf, Part::Part1);
    let mut all_wins = vec![];
    for r in races {
        all_wins.push(get_wins(&r));
    }
    let mut out = 1;
    for win in all_wins {
        out *= win;
    }
    out
}

struct Race {
    time: usize,
    dist: usize,
}

enum Part {
    Part1,
    Part2
}

fn parse(s: &str, p: Part) -> Vec<Race> {
    let mut times = vec![];
    let mut dists = vec![];
    for l in s.split('\n') {
        let toks : Vec<&str> = l.split(':').collect();
        if toks.len() != 2 {
            continue
        }

        let nums : &mut Vec<usize>;
        match toks[0] {
            "Time" => nums = &mut times,
            "Distance" => nums = &mut dists,
            _ => todo!(),
        }

        match p {
            Part::Part1 => {
                for t in toks[1].split_whitespace() {
                    nums.push(t.parse::<usize>().unwrap());
                }
            },
            Part::Part2 => {
                let s : String = toks[1].chars().filter(|c| !c.is_whitespace()).collect();
                nums.push(s.parse::<usize>().unwrap());
            }
        }
    }

    let mut out = vec![];
    assert_eq!(times.len(), dists.len());
    for i in 0..times.len() {
        out.push(Race{time: times[i], dist: dists[i] });
    }
    out
}

fn part2(buf: &str) -> usize {
    let races = parse(buf, Part::Part2);
    get_wins(&races[0])
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

    const INPUT : &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 71503);
    }
}

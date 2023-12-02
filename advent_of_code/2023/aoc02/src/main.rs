use std::io::{stdin, Read};

fn is_possible(set: &Cubes, pool: &Cubes) -> bool {
    return set.red <= pool.red
        && set.green <= pool.green
        && set.blue <= pool.blue;
}

fn part1(buf: &str) -> u32 {
    let mut ids = vec![];
    for l in buf.split('\n') {
        let game = Game::from_str(l);
        if game.is_none() {
            continue;
        }
        let game = game.unwrap();
        let pool = Cubes::new(12, 13, 14);
        let mut possible = true;
        for set in game.sets {
            if !is_possible(&set, &pool) {
                possible = false;
                break;
            }
        }
        if possible {
            ids.push(game.id);
        }
    }
    return ids.into_iter().reduce(|a, b| a + b ).unwrap();
}

fn part2(buf: &str) -> u32 {
    let mut powers = vec![];
    for l in buf.split('\n') {
        let game = Game::from_str(l);
        if game.is_none() {
            continue;
        }
        let game = game.unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in game.sets {
            min_red = std::cmp::max(min_red, set.red);
            min_green = std::cmp::max(min_green, set.green);
            min_blue = std::cmp::max(min_blue, set.blue);
        }
        powers.push(min_red * min_green * min_blue);
    }
    return powers.into_iter().reduce(|a, b| a + b ).unwrap();
}

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Cubes{ red, green, blue }
    }
}

struct Game {
    id: u32,
    sets: Vec<Cubes>,
}


impl Game {
    fn from_str(s : &str) -> Option<Self> {
        let mut out = Self {
            id: 0, sets: vec!{}
        };
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let top_toks : Vec<&str> = s.split(':').collect();
        if top_toks.len() != 2 {
            return None;
        }

        let game_toks : Vec<&str> = top_toks[0].split(' ').collect();
        if game_toks.len() != 2 {
            return None;
        }

        out.id = game_toks[1].parse::<u32>().unwrap();
        out.sets = vec![];

        for set_entry in top_toks[1].split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for mut entry in set_entry.split(',') {
                let slot: &mut u32;
                entry = entry.trim();
                let toks : Vec<&str>= entry.split(' ').collect();
                assert_eq!(toks.len(), 2);
                let label = &toks[1];
                match label {
                    &"red" => slot = &mut red,
                    &"green" => slot = &mut green,
                    &"blue" => slot = &mut blue,
                    _ => todo!(),
                }
                *slot = toks[0].parse::<u32>().unwrap();
            }
            out.sets.push(Cubes{red, green, blue});
        }
        Some(out)
    }
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
    const INPUT : &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 2286);
    }
}

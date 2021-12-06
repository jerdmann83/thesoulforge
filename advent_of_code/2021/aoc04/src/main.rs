use std::io::{stdin, Read};

fn part1() {}

type TilesT = [[u32; 5]; 5];
#[derive(Debug)]
struct Board {
    tiles: TilesT,
}

impl Board {
    fn new(tiles: TilesT) -> Self {
        Board { tiles: tiles }
    }
}

struct Game {
    boards: Vec<Board>,
    moves: Vec<u32>,
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut first = true;
    let mut moves = vec![];
    let mut tiles: TilesT = [[0; 5]; 5];
    let mut tile_row = 0;
    let mut boards = vec![];
    for l in buf.split('\n') {
        if first {
            first = false;
            for tok in l.split(',') {
                moves.push(tok.parse::<u32>().unwrap());
            }
            continue;
        }

        let toks: Vec<&str> = l.split_whitespace().collect();
        if toks.len() < 5 {
            continue;
        }
        if tile_row == 5 {
            boards.push(Board::new(tiles));
            tile_row = 0;
        }

        let mut pos = 0;
        for tok in toks {
            tiles[tile_row][pos] = tok.parse::<u32>().unwrap();
            pos += 1;
        }
        tile_row += 1;
    }
    assert!(tile_row == 5);
    boards.push(Board::new(tiles));
    println!("{:?}", boards);
}

mod test {
    use super::*;

    #[test]
    fn test_convert() {}
}

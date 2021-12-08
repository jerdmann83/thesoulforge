use std::io::{stdin, Read};

type Grid<T> = [[T; 5]; 5];
#[derive(Clone, Debug)]
struct Board {
    tiles: Grid<u32>,
    marks: Grid<bool>,
    winning_move: Option<u32>,
}

impl Board {
    fn new(tiles: Grid<u32>) -> Self {
        Board {
            tiles: tiles,
            marks: [[false; 5]; 5],
            winning_move: None,
        }
    }

    fn is_bingo(&self, move_y: usize, move_x: usize) -> bool {
        let mut win;
        // TODO: func/trait these repeat blocks?
        // TODO: use the newly-set x+y as an optimization
        // don't need to check the entire board...
        for y in 0..self.marks.len() {
            win = true;
            for x in 0..self.marks[y].len() {
                win &= self.marks[y][x];
            }
            if win {
                return win;
            }
        }

        for x in 0..self.marks[0].len() {
            win = true;
            for y in 0..self.marks[x].len() {
                win &= self.marks[y][x];
            }
            if win {
                return win;
            }
        }
        false
    }

    // set a tile number (if present).
    // return true if this move creates a bingo for the first time
    fn set(&mut self, num: u32) -> bool {
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if self.tiles[y][x] == num {
                    self.marks[y][x] = true;

                    if self.is_bingo(y, x) && self.winning_move.is_none() {
                        let mv = Some(self.tiles[y][x]);
                        self.winning_move = mv;
                        return true;
                    }
                }
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                if !self.marks[y][x] {
                    sum += self.tiles[y][x];
                }
            }
        }
        sum
    }

    fn score(&self) -> u32 {
        match self.winning_move {
            Some(mv) => self.sum_unmarked() * mv,
            None => 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Game {
    boards: Vec<Board>,
    moves: Vec<u32>,
}

fn part1(mut game: Game) -> u32 {
    for mv in game.moves {
        for board in &mut game.boards {
            if board.set(mv) {
                return board.score();
            }
        }
    }
    unreachable!();
}

fn part2(mut game: Game) -> u32 {
    let mut winners = vec![];
    for mv in game.moves {
        for board in &mut game.boards {
            if board.set(mv) {
                winners.push(board.clone());
            }
        }
    }
    winners[winners.len() - 1].score()
}

fn parse(buf: &str) -> Game {
    let mut first = true;
    let mut moves = vec![];
    let mut tiles: Grid<u32> = [[0; 5]; 5];
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

    Game {
        boards: boards,
        moves: moves,
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let game = parse(&buf);

    println!("part1: {}", part1(game.clone()));
    println!("part2: {}", part2(game.clone()));
}

mod test {
    use super::*;

    #[test]
    fn test_convert() {}
}

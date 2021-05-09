use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read};

type CharGridT = Vec<Vec<char>>;

#[derive(Clone, Debug)]
struct Tile {
    grid: CharGridT,
    id: u32,
    stable: bool,
}
type TileGridT = Vec<Vec<Tile>>;

impl Tile {
    fn new(g: CharGridT, id: u32) -> Self {
        Tile {
            grid: g,
            id: id,
            stable: false,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

fn parse(buf: &str) -> Vec<Tile> {
    let mut cur_id = 0;
    let mut cur_grid: CharGridT = vec![];
    let mut out: Vec<Tile> = vec![];
    for l in buf.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let toks: Vec<&str> = l.split_whitespace().collect();
        if toks.len() == 2 {
            let id: Vec<&str> = toks[1].split(':').collect();
            if cur_id > 0 {
                out.push(Tile::new(cur_grid, cur_id));
            }
            cur_id = id[0].parse::<u32>().unwrap();
            cur_grid = vec![];
            continue;
        }

        cur_grid.push(l.chars().collect());
    }

    if cur_grid.len() > 0 {
        out.push(Tile::new(cur_grid, cur_id));
    }

    out
}

fn rotate_tile(t: &mut Tile) {
    t.grid = rotate(&t.grid);
}

fn rotate(g: &CharGridT) -> CharGridT {
    let mut out: CharGridT = vec![];
    let mut itrs = vec![];
    // izip in the itertools package is a more idiomatic way of doing the below
    // however, I can't yet get my head around how to do it with an arbitrary
    // number of iterators, eg a collection/vec of them. just do it by hand
    for row in g.iter().rev() {
        itrs.push(row.iter());
    }

    let mut done = false;
    while !done {
        let mut next: Vec<char> = vec![];
        for itr in &mut itrs {
            match itr.next() {
                Some(n) => next.push(*n),
                None => {
                    done = true;
                    break;
                }
            }
        }
        if done {
            break;
        }
        out.push(next.to_vec());
    }
    out
}

fn is_horizontal_match(left: &mut CharGridT, right: &mut CharGridT) -> bool {
    // print_grids(left, right);
    for y in 0..left.len() {
        let x = left[y].len() - 1;
        let lrune = left[y][x];
        let rrune = right[y][0];
        if lrune != rrune {
            return false;
        }
    }
    true
}

fn is_vertical_match(top: &mut CharGridT, bot: &mut CharGridT) -> bool {
    // print_grids(top, bot);
    let y = top[0].len() - 1;
    for x in 0..top[0].len() - 1 {
        let trune = top[y][x];
        let brune = bot[0][x];
        if trune != brune {
            return false;
        }
    }
    true
}

fn is_vertical_match_tile(top: &mut Tile, bot: &mut Tile) -> bool {
    is_vertical_match(&mut top.grid, &mut bot.grid)
}

fn is_horizontal_match_tile(left: &mut Tile, right: &mut Tile) -> bool {
    is_horizontal_match(&mut left.grid, &mut right.grid)
}

fn try_match(left: &mut CharGridT, right: &mut CharGridT) -> bool {
    for _i in 0..3 {
        for _j in 0..3 {
            if is_horizontal_match(left, right) {
                return true;
            }
            *right = rotate(right);
        }
        *left = rotate(left);
    }
    return false;
}

// super lame, learn to Trait this guy
fn print_grid(v: &TileGridT) {
    for row in v {
        for tile in row {
            print!("{} ", tile.id);
        }
        print!("\n");
    }
}

fn print_grids(left: &CharGridT, right: &CharGridT) {
    for i in 0..left.len() {
        println!("{:?} {:?}", left[i], right[i]);
    }
}

fn random_layout(tiles: &[Tile]) -> TileGridT {
    let mut remain = HashMap::new();
    let mut remain_ids = vec![];
    let mut out = vec![];

    for t in tiles {
        remain.insert(t.id, t);
        remain_ids.push(t.id);
    }

    let row_len = (tiles.len() as f64).sqrt() as usize;
    let mut cur: Vec<Tile> = vec![];
    let mut rng = rand::thread_rng();

    let mut count = 0;
    while remain.len() > 0 {
        let id_pos = rng.gen::<usize>() % remain_ids.len();
        let id = remain_ids.remove(id_pos);
        if let Some(t) = remain.remove(&id) {
            if count > 0 && count % row_len == 0 {
                out.push(cur);
                cur = vec![];
            }
            let mut hack = t.clone();
            shuffle(&mut hack);
            cur.push(hack);
            count += 1;
        }
    }

    if cur.len() > 0 {
        out.push(cur);
    }
    out
}

// rotate a tile between 0 and 3 times
fn shuffle(t: &mut Tile) {
    let mut rng = thread_rng();
    let num = rng.gen::<u16>() % 3;
    for _ in 0..num {
        rotate_tile(t);
    }
}

fn score_grid(mut tg: TileGridT) -> u32 {
    let mut y1 = 0;
    let mut y2 = 1;
    while y2 < tg.len() {
        let mut x = 0;
        while x < tg[y1].len() {
            if !is_vertical_match_tile(&mut tg[y1][x].clone(), &mut tg[y2][x].clone()) {
                return 0;
            }
            x += 1;
        }
        y1 += 1;
        y2 += 1;
    }

    let mut x1 = 0;
    let mut x2 = 1;
    while x2 < tg[0].len() {
        let mut y = 0;
        while y < tg.len() {
            if !is_horizontal_match_tile(&mut tg[y][x1].clone(), &mut tg[y][x2].clone()) {
                return 0;
            }
            y += 1;
        }
        y1 += 1;
        y2 += 1;
    }

    let y = tg.len() - 1;
    let x = tg[0].len() - 1;
    tg[0][0].id * tg[y][0].id * tg[y][x].id * tg[0][x].id
}

fn part1(t: &[Tile]) -> u32 {
    let mut score: u32;
    let mut count: u32 = 0;
    loop {
        let rl = random_layout(t);
        // print_grid(&rl);
        score = score_grid(rl);
        if score > 0 {
            break;
        }
        count += 1;
        if count % 10000 == 0 {
            println!("{:?} iterations...", count);
        }
    }
    score
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let tiles = parse(&buf);
    println!("part1: {:?}", part1(&tiles));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rotate() {
        let g: CharGridT = vec![
            ['1', '2', '3'].to_vec(),
            ['4', '5', '6'].to_vec(),
            ['7', '8', '9'].to_vec(),
        ];
        let gr = rotate(&g);

        assert_eq!(
            gr,
            vec![
                ['7', '4', '1'].to_vec(),
                ['8', '5', '2'].to_vec(),
                ['9', '6', '3'].to_vec()
            ]
        );

        let g: CharGridT = vec![
            ['1', '2', '3', '4'].to_vec(),
            ['4', '5', '6', '4'].to_vec(),
            ['7', '8', '9', '4'].to_vec(),
        ];
        let gr = rotate(&g);

        assert_eq!(
            gr,
            vec![
                ['7', '4', '1'].to_vec(),
                ['8', '5', '2'].to_vec(),
                ['9', '6', '3'].to_vec(),
                ['4', '4', '4'].to_vec(),
            ]
        );
    }

    #[test]
    fn test_try_match() {
        // two grids that match on edge straight away
        let mut g1: CharGridT = vec![['1', '2'].to_vec(), ['4', '5'].to_vec()];
        let mut g2: CharGridT = vec![['2', '8'].to_vec(), ['5', '9'].to_vec()];
        assert!(try_match(&mut g1, &mut g2));

        // one that now won't
        g2 = vec![['3', '8'].to_vec(), ['6', '9'].to_vec()];
        assert!(!try_match(&mut g1, &mut g2));

        // two that will if we rotate the grids
        g1 = vec![['1', '7'].to_vec(), ['4', '2'].to_vec()];
        g2 = vec![['1', '8'].to_vec(), ['7', '2'].to_vec()];
        assert!(try_match(&mut g1, &mut g2));
    }

    #[test]
    fn test_vertical_match() {
        let mut g1: CharGridT = vec![['1', '2'].to_vec(), ['4', '5'].to_vec()];
        let mut g2: CharGridT = vec![['4', '5'].to_vec(), ['3', '9'].to_vec()];
        assert!(is_vertical_match(&mut g1, &mut g2));
        assert!(!is_vertical_match(&mut g2, &mut g1));
    }

    fn test_part1() {
        // let tiles = vec![
        //     Tile::new(
        //         vec![
        //             "#..".chars().collect(),
        //             "#..".chars().collect(),
        //             "...".chars().collect(),
        //         ],
        //         111,
        //     ),
        //     Tile::new(
        //         vec![
        //             "#..".chars().collect(),
        //             "#..".chars().collect(),
        //             "...".chars().collect(),
        //         ],
        //         112,
        //     ),
        //     Tile::new(
        //         vec![
        //             "#..".chars().collect(),
        //             "#..".chars().collect(),
        //             "...".chars().collect(),
        //         ],
        //         113,
        //     ),
        //     Tile::new(
        //         vec![
        //             "#..".chars().collect(),
        //             "#..".chars().collect(),
        //             "...".chars().collect(),
        //         ],
        //         114,
        //     ),
        // ];
    }
}

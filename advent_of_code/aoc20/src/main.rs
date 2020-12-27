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
    // posx: Option<u32>,
    // posy: Option<u32>,
    // facing in degrees.  0 is right
    // face: Option<u32>,
}
type TileGridT = Vec<Vec<Tile>>;

impl Tile {
    fn new(g: &CharGridT, id: u32) -> Self {
        Tile {
            grid: g.to_vec(),
            id: id,
            stable: false,
            // posx: None,
            // posy: None,
            // face: None,
        }
    }
}

impl fmt::Display for Tile {
    // This trait requires `fmt` with this exact signature.
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
                out.push(Tile::new(&cur_grid, cur_id));
            }
            cur_id = id[0].parse::<u32>().unwrap();
            cur_grid = vec![];
            continue;
        }

        cur_grid.push(l.chars().collect());
    }

    if cur_grid.len() > 0 {
        out.push(Tile::new(&cur_grid, cur_id));
    }

    out
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

// super lame, learn to Trait this guy
fn print_grid(v: &TileGridT) {
    for row in v {
        for tile in row {
            print!("{} ", tile.id);
        }
        print!("\n");
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
            cur.push(t.clone());
            count += 1;
        }
    }

    if cur.len() > 0 {
        out.push(cur);
    }
    out
}

fn shuffle(tiles: &mut TileGridT) {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    loop {
        // if x > 0 {
        // }
    }
}

fn shuffle_single(tiles: &mut TileGridT) {}

fn part1(t: &[Tile]) -> Option<u32> {
    loop {
        let rl = random_layout(t);

        break;
    }
    // example: 20899048083289
    None
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

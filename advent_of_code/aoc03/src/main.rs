use std::io::{stdin, Read};

#[derive(Debug)]
enum Terrain {
    Clear,
    Tree,
}
impl Terrain {
    fn from_char(c: &char) -> Terrain {
        match c {
            '#' => Terrain::Tree,
            '.' => Terrain::Clear,
            _ => Terrain::Clear,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn add(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

fn num_trees(map: &MapT, slope: &Point) -> usize {
    let mut out: usize = 0;
    let mut cur = Point { x: 0, y: 0 };
    let x_lim = map[0].len();
    while cur.y < map.len() {
        if cur.x >= x_lim {
            cur.x -= x_lim
        }
        match map[cur.y][cur.x] {
            Terrain::Tree => out += 1,
            Terrain::Clear => {}
        }
        cur.add(&slope);
    }
    out
}

type MapT = Vec<Vec<Terrain>>;

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut map: MapT = vec![];
    for row in buf.split_whitespace() {
        let mut new_row: Vec<Terrain> = vec![];
        for c in row.chars() {
            new_row.push(Terrain::from_char(&c));
        }
        map.push(new_row);
    }

    let slopes: Vec<Point> = vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ];

    let mut out = 1;
    for s in slopes {
        let trees = num_trees(&map, &s);
        out *= trees;
    }
    println!("{}", out);
}

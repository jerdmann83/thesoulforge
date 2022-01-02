use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

type Map = HashMap<String, Vec<String>>;

const START: &str = "start";
const END: &str = "end";

fn parse(buf: &str) -> Map {
    let mut map = Map::new();
    for l in buf.split('\n') {
        let toks: Vec<&str> = l.split('-').collect();
        if toks.len() != 2 {
            continue;
        }
        // there has to be a more ergonomic way to do this
        // undirected graph hashmap thing...
        if !map.contains_key(toks[0]) {
            map.insert(toks[0].to_string(), vec![]);
        }
        let vals = map.get_mut(toks[0]).unwrap();
        vals.push(toks[1].to_string());

        if !map.contains_key(toks[1]) {
            map.insert(toks[1].to_string(), vec![]);
        }
        let vals = map.get_mut(toks[1]).unwrap();
        vals.push(toks[0].to_string());
    }
    map
}

type Path = Vec<String>;

#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
}

fn visit(map: &Map, src: &str, dst: &str, part: Part) -> Vec<Path> {
    let mut cur: Path = vec![];
    let mut paths: Vec<Path> = vec![];
    let mut visited = Visited::new();
    visit_impl(&map, src, dst, part, &mut cur, &mut paths, &mut visited);
    paths
}

struct Visited {
    count: HashMap<String, u32>,
}

impl Visited {
    fn new() -> Self {
        Visited {
            count: HashMap::new(),
        }
    }

    fn visit(&mut self, node: &str) -> u32 {
        // really wish hashmap implemented IndexMut
        // even C++'s map semantics seem simple compared to this...
        if !self.count.contains_key(node) {
            self.count.insert(node.to_string(), 0);
        }
        let n = self.count.get_mut(node).unwrap();
        *n += 1;
        *n
    }

    fn unvisit(&mut self, node: &str) {
        if !self.count.contains_key(node) {
            return;
        }
        *self.count.get_mut(node).unwrap() -= 1;
    }

    fn get(&self, node: &str) -> u32 {
        if !self.count.contains_key(node) {
            return 0;
        }
        *self.count.get(node).unwrap()
    }

    fn get_counts(&self) -> HashMap<String, u32> {
        self.count.clone()
    }
}

fn is_large_room(node: &str) -> bool {
    node.as_bytes()[0].is_ascii_uppercase()
}

fn visit_impl(
    map: &Map,
    src: &str,
    dst: &str,
    part: Part,
    cur: &mut Path,
    paths: &mut Vec<Path>,
    visited: &mut Visited,
) {
    visited.visit(src);
    cur.push(src.to_string());

    if src == dst {
        println!("{:?}", cur);
        paths.push(cur.clone());
    } else {
        if map.contains_key(src) {
            for adj in &map[src] {
                let visit: bool;
                match part {
                    Part::Part1 => {
                        visit = visited.get(adj) == 0 || is_large_room(adj);
                    }
                    Part::Part2 => {
                        if adj == START {
                            visit = false;
                        } else if is_large_room(adj) {
                            visit = true;
                        } else {
                            let mut small_revisits = 0;
                            for room in visited.get_counts() {
                                if is_large_room(&room.0) {
                                    continue;
                                }
                                if room.1 > 1 {
                                    small_revisits += 1;
                                    println!("{} {:?}", room.0, small_revisits);
                                }
                            }
                            visit = small_revisits < 1;
                        }
                    }
                }
                if visit {
                    visit_impl(&map, adj, dst, part, cur, paths, visited);
                }
            }
        }
    }

    cur.pop();
    visited.unvisit(src);
}

fn part1(map: &Map) -> Vec<Path> {
    return visit(map, START, END, Part::Part1);
}

fn serialize(path: &Path) -> String {
    let mut buf = String::new();
    for p in path {
        buf = format!("{}-{}", buf, p);
    }
    buf
}

fn part2(map: &Map) -> Vec<Path> {
    let paths = visit(map, START, END, Part::Part2);
    let mut out = vec![];
    // let mut seen = HashSet::new();
    for p in paths {
        // let key = serialize(&p);
        // if seen.contains(&key) {
        //     continue;
        // }
        // seen.insert(key);
        out.push(p);
    }
    out
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let map = parse(&buf);

    println!("part1: {}", part1(&map).len());
    println!("part2: {}", part2(&map).len());
}

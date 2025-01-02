use std::io::{stdin, Read};
use std::collections::HashMap;
use util::Point;

type Grid = Vec<Vec<u32>>;
type Moves = Vec<char>;
#[derive(Clone, Copy, Debug, PartialEq)]
enum EntityType {
    Robot,
    Empty,
    Wall,
    Box,
}
#[derive(Clone, Copy, Debug)]
struct Entity {
    et: EntityType,
    // position key-field
    // for multi-cell entities, this will store the leftmost cell
    pos: Point,
}
impl Entity {
    fn new(et: EntityType, pos: Point) -> Self {
        Self { et, pos }
    }
}

type Entities = HashMap<u32, Entity>;
struct Map {
    grid: Grid,
    entities: Entities,
    robot: u32,
}

impl Map {
    pub fn get(&self, p: Point) -> Option<Entity> {
        if p.x < 0 || p.y < 0 
            || p.x as usize >= self.grid[0].len()
                || p.y as usize >= self.grid.len() {
                    return None;
                }
        let handle = self.grid[p.y as usize][p.x as usize];
        Some(self.entities[&handle])
    }

    pub fn get_robot(&self) -> Entity {
        assert![self.robot > 0];
        self.entities[&self.robot].clone()
    }

    pub fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let handle = self.grid[y][x];
                let e = self.entities[&handle];
                let c = match e.et {
                    EntityType::Empty => ".",
                    EntityType::Wall =>  "#",
                    EntityType::Robot => "@",
                    EntityType::Box => if e.pos == Point::new(x as i32, y as i32) { "[" } else { "]" },
                };
                print!("{}", c);
            }
            print!("{}", "\n");
        }
    }
}

fn parse(s: &str) -> ( Map, Moves ) {
    let chunks : Vec<&str> = s.split("\n\n").collect();
    assert_eq![chunks.len(), 2];
    let mut grid : Grid = vec![];
    let mut entities : Entities = HashMap::new();
    let mut handle = 1;
    let mut robot = 0;
    for (yi,l) in chunks[0].split('\n').enumerate() {
        let mut row : Vec<u32> = vec![];
        for (xi,c) in l.chars().enumerate() {
            let mut pt = Point::new((xi * 2) as i32, yi as i32);
            match c {
                '#' => {
                    // each wall is its own entity
                    // this doesn't really matter much either way
                    // it's not like we have destructible terrain, at least not yet? :)
                    entities.insert(handle, Entity::new(EntityType::Wall, pt));
                    row.push(handle);
                    handle += 1;

                    pt.x += 1;
                    entities.insert(handle, Entity::new(EntityType::Wall, pt));
                    row.push(handle);
                    handle += 1;
                },
                'O' => {
                    entities.insert(handle, Entity::new(EntityType::Box, pt));
                    // one box entity occupies two cells in the row
                    row.push(handle);
                    row.push(handle);
                    handle += 1;
                },
                '.' => {
                    entities.insert(handle, Entity::new(EntityType::Empty, pt));
                    row.push(handle);
                    handle += 1;

                    pt.x += 1;
                    entities.insert(handle, Entity::new(EntityType::Empty, pt));
                    row.push(handle);
                    handle += 1;
                },
                '@' => {
                    entities.insert(handle, Entity::new(EntityType::Robot, pt));
                    row.push(handle);
                    robot = handle;
                    handle += 1;

                    pt.x += 1;
                    entities.insert(handle, Entity::new(EntityType::Empty, pt));
                    row.push(handle);
                    handle += 1;
                },
                _   => {},
            };
        }
        grid.push(row);
    }

    let moves : Moves = chunks[1].chars().collect();
    assert![robot > 0];
    ( Map { grid, entities, robot }, moves )
}

fn score(g: &Grid) -> u32 {
    let mut out = 0;
    for y in 0..g.len() {
        for x in 0..g[y].len() {
            // match g[y][x] {
            //     'O' => out += (100 * y) + x,
            //     _ => {},
            // }
        }
    }
    out as u32
}

fn part2(buf: &str) -> u32 {
    let (mut m, moves) = parse(buf);
    for mv in &moves {
        let dir : Point;
        match mv {
            '<' => dir = Point::new(-1, 0),
            '>' => dir = Point::new(1, 0),
            '^' => dir = Point::new(0, -1),
            'v' => dir = Point::new(0, 1),
            _ => continue,
        }

        let pt = m.get_robot().pos + dir;
        let next = m.get(pt);
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        match next.et {
            EntityType::Wall => {},
            EntityType::Empty => {
            },
            EntityType::Box => {
            },
            // '#' => {},
            // '.' => {
            //     m.swap(robot, pt);
            //     robot = pt;
            // }
            // '[' | ']' => {
            //     let mut boxes : Vec<Box2> = vec![];
            //     let mut frontier : Vec<char> = vec![];
            //     let locs = box_locs(pt, next);
            //     frontier.push(m.get(locs.0).unwrap());
            //     frontier.push(m.get(locs.1).unwrap());
            //     while frontier.len() > 0 {
            //         let pts = frontier.pop().unwrap();
            //         let tile1 = m.get(pts.0);
            //         let tile2 = m.get(pts.1);
            //         if tile1.is_none() || tile2.is_none() {
            //             continue;
            //         }
            //         let tile1 = tile1.unwrap();
            //         let tile2 = tile2.unwrap();
            //         // handle horizontal vs vertical move
            //         if dir.x != 0 {
            //         }
            //     }
            // }
            _ => {},
        }
    }

    score(&m.grid)
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("part2: {}", part2(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example2() {
        let s = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq![part2(s), 9021];
    }

    #[test]
    fn test_print() {
        let s = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let (m, _moves) = parse(s);
        m.print();
    }
}

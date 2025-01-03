use std::io::{stdin, Read};
use std::collections::{ HashMap, HashSet, VecDeque };
use util::Point;

type Grid = Vec<Vec<u32>>;
type Moves = Vec<Dir>;
#[derive(Clone, Copy, Debug, PartialEq)]
enum EntityType {
    Robot,
    Wall,
    Box,
}

const NO_ENTITY : u32 = 0;
#[derive(Clone, Copy, Debug)]
struct Entity {
    et: EntityType,
    // position key-field
    // for multi-cell entities, this will store the leftmost cell
    pos: Point,
    // denormalization:  store the handle inline with each entity
    // in theory we could just always look it up from the position field, but:
    // 1. it's convenient to have it in places where we already know the entity
    // 2. it's theoretically a little more efficient to save the grid lookup
    handle: u32,
    // how many cells wide a given entity is
    sz: usize,
}

impl Entity {
    fn new(et: EntityType, pos: Point, handle: u32) -> Self {
        let sz;
        match et {
            EntityType::Box => sz = 2,
            _ => sz = 1,
        }
        Self { et, pos, handle, sz }
    }

    fn cells(&self) -> Vec<Point> {
        let mut out = vec![];
        for x in 0..self.sz {
            let pt = self.pos + Point::new(x as i32, 0);
            out.push(pt);
        }
        out
    }

    fn nbrs(&self, d: Dir) -> Vec<Point> {
        let mov = from_dir(d);
        // horizontal moves
        if mov.x < 0 {
            return vec![self.pos + mov];
        } 
        if mov.x > 0 {
            let offset = Point::new(self.sz as i32 - 1, 0);
            return vec![self.pos + offset + mov];
        } 

        // vertical moves
        let mut out = vec![];
        for x in 0..self.sz {
            let offset = Point::new(x as i32, 0);
            let pt = self.pos + offset + mov;
            out.push(pt);
        }
        return out;
    }
}

type Entities = HashMap<u32, Entity>;
struct Map {
    grid: Grid,
    entities: Entities,
    robot: u32,
    dirty: Vec<u32>,
}

impl Map {
    pub fn get_handle(&self, p: Point) -> Option<u32> {
        if p.x < 0 
            || p.y < 0 
            || p.x as usize >= self.grid[0].len()
                || p.y as usize >= self.grid.len() {
                    return None;
        }
        Some(self.grid[p.y as usize][p.x as usize])
    }

    pub fn get_entity(&self, p: Point) -> Option<Entity> {
        if let Some(handle) = self.get_handle(p) {
            if handle == NO_ENTITY {
                return None;
            }
            return Some(self.entities[&handle]);
        }
        None
    }

    pub fn get_robot_handle(&self) -> u32 {
        assert![self.robot > 0];
        self.robot
    }

    pub fn get_robot_pos(&self) -> Point {
        self.entities[&self.get_robot_handle()].pos
    }

    pub fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let handle = self.grid[y][x];
                let c : char;
                if handle == NO_ENTITY {
                    c = '.';
                } else {
                    let e = self.entities[&handle];
                    c = match e.et {
                        EntityType::Wall =>  '#',
                        EntityType::Robot => '@',
                        EntityType::Box => if e.pos == Point::new(x as i32, y as i32) { '[' } else { ']' },
                    };
                }
                print!("{}", c);
            }
            print!("{}", "\n");
        }
        print!("{}", "\n");
    }

    fn redraw(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                self.grid[y][x] = NO_ENTITY;
            }
        }
        for (_, e) in &self.entities {
            for cell in e.cells() {
                self.grid[cell.y as usize][cell.x as usize] = e.handle;
            }
        }
    }

    pub fn move_robot(&mut self, d: Dir) {
        let e = self.entities[&self.robot];
        let dst = e.pos + from_dir(d);
        let sh  = self.get_handle(e.pos).unwrap();
        let dh  = self.get_handle(dst).unwrap_or(0);

        let se = self.entities.get(&sh).unwrap();
        let de = self.entities.get(&dh);

        match de {
            None => {
                let mov = dst;
                self.entities.entry(sh).and_modify(|se| se.pos = mov);
                self.dirty.push(sh);
            },
            Some(de) => {
                let mut frontier : VecDeque<Point> = de.cells().into();
                let mut boxes : HashSet<u32> = HashSet::new();
                let mut ok = true;
                while frontier.len() > 0 {
                    let pt = frontier.pop_front().unwrap();
                    let en = self.get_entity(pt);
                    if en.is_none() {
                        continue;
                    }
                    let en = en.unwrap();
                    self.dirty.push(en.handle);
                    match en.et {
                        EntityType::Box => {
                            boxes.insert(en.handle);
                            for nbr in en.nbrs(d) {
                                if !frontier.contains(&nbr) {
                                    frontier.push_back(nbr);
                                }
                            }
                        }
                        // if any box pushed would hit a wall, we're done
                        EntityType::Wall => {
                            ok = false;
                            frontier.clear();
                            continue;
                        }
                        _ => {},
                    }
                }

                if ok {
                    // move robot and all pushed boxes
                    let mov = from_dir(d);
                    self.entities.entry(sh).and_modify(|se| se.pos = se.pos + mov);
                    for bh in boxes {
                        self.entities.entry(bh).and_modify(|e| {
                            e.pos = e.pos + mov;
                        });
                    }
                }
            }
        }

        self.redraw();
    }

    fn score(&self) -> u32 {
        let mut out = 0;
        for (_handle, e) in &self.entities {
            if e.et != EntityType::Box {
                continue;
            }
            out += (e.pos.y * 100) + e.pos.x;
        }
        out as u32
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
                    let et = EntityType::Wall;
                    entities.insert(handle, Entity::new(et, pt, handle));
                    row.push(handle);
                    handle += 1;

                    pt.x += 1;
                    entities.insert(handle, Entity::new(et, pt, handle));
                    row.push(handle);
                    handle += 1;
                },
                'O' => {
                    let et = EntityType::Box;
                    entities.insert(handle, Entity::new(et, pt, handle));
                    // one box entity occupies two cells in the row
                    row.push(handle);
                    row.push(handle);
                    handle += 1;
                },
                '.' => {
                    row.push(NO_ENTITY);
                    row.push(NO_ENTITY);
                },
                '@' => {
                    entities.insert(handle, Entity::new(EntityType::Robot, pt, handle));
                    row.push(handle);
                    robot = handle;
                    handle += 1;

                    row.push(NO_ENTITY);
                },
                _   => {},
            };
        }
        grid.push(row);
    }

    let mut moves : Moves = vec![];
    for c in chunks[1].chars() {
        let mv = match c {
            '^' => Dir::Up,
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => continue,
        };
        moves.push(mv);
    }
    assert![robot > 0];
    let dirty = vec![];
    ( Map { grid, entities, robot, dirty }, moves )
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn from_dir(d: Dir) -> Point {
    match d {
        Dir::Left  => return Point::new(-1, 0),
        Dir::Right => return Point::new(1, 0),
        Dir::Up    => return Point::new(0, -1),
        Dir::Down  => return Point::new(0, 1),
    }
}

fn solve(buf: &str) -> u32 {
    let (mut m, moves) = parse(buf);
    for mv in &moves {
        m.move_robot(*mv);
    }
    m.score()
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    println!("solve: {}", solve(&buf));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example2() {
        let s = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq![solve(s), 9021];
    }

    #[test]
    fn moves() {
        let s = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

^<<";
        let (mut m, _moves) = parse(s);
        m.print();

        m.move_robot(Dir::Up);
        m.print();
        m.move_robot(Dir::Left);
        m.print();
        m.move_robot(Dir::Down);
        m.print();
        m.move_robot(Dir::Left);
        m.print();
        m.move_robot(Dir::Left);
        m.print();
        m.move_robot(Dir::Down);
        m.print();
    }

    #[test]
    fn score() {
        let s = "#######
#....O#
#.....#
#....@#
#.....#
#.....#
#######

^<<";
        let (m, _moves) = parse(s);
        m.print();
        println!("{:?}", m.score());
    }
}

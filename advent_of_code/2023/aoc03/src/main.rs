use std::io::{stdin, Read};

fn part1(buf: &str) -> u32 {
    let sc = Schematic::from_str(buf);
    println!("{:?}", sc);
    let vals = sc.get_part_numbers();
    vals.into_iter().reduce(|a, b| a+b).unwrap()
}

fn part2(buf: &str) -> u32 {
    todo!();
}

#[derive(Debug)]
enum Field {
    Digit(u8),
    Symbol(char),
    Empty,
}

type Grid = Vec<Vec<Field>>;
#[derive(Debug)]
struct Schematic {
    grid: Grid,
}

impl Schematic {
    fn from_str(s: &str) -> Self {
        let mut grid : Grid = vec![];
        let mut last : usize = 0;
        for l in s.split('\n') {
            let mut row = vec![];
            let chars : Vec<char> = l.chars().collect();
            if chars.is_empty() {
                continue;
            }
            if last == 0 {
                last = chars.len();
            } else {
                assert_eq!(last, chars.len());
            }

            for c in chars {
                let field : Field;
                let val = c.to_digit(10);
                match val {
                    Some(x) => field = Field::Digit(x as u8),
                    _ => {
                        if c == '.' {
                            field = Field::Empty;
                        } else {
                            field = Field::Symbol(c);
                        }
                    }
                }
                row.push(field);
            }
            grid.push(row);
        }
        Self {
            grid,
        }
    }

    fn get_numbers(row: &Vec<Field>) -> Vec<Vec<(u8, usize)>> {
        let mut out = vec![];

        let mut digits = vec![];
        for x in 0..row.len() {
            let cell = &row[x];
            if let Field::Digit(val) = cell {
                digits.push((val, x));
            }
        }
        let mut last : i32 = -1;
        let mut cur = vec![];
        for d in digits {
            if last == -1 || d.1 as i32 - last > 1 {
                last = d.1 as i32;
                if cur.len() > 0 {
                    // for 
                    // out.push(cur.clone());
                }
                cur.push((*d.0, d.1));
                println!("{:?}", d);
            }
        }
        out
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let out = vec![];
        for y in 0..self.grid.len() {
            // let mut digits = vec![];
            // for x in 0..self.grid[y].len() {
            //     let cell = &self.grid[y][x];
            //     if let Field::Digit(val) = cell {
            //         digits.push((val, x));
            //     }
            // }
            // let mut last = -1;
            // let mut numbers = vec![];
            // let mut cur = vec![];
            // for d in digits {
            //     if last == -1 or d.1 - last > 1 {
            //         last = last.1;
            //         if cur.len() > 0 {
            //             numbers.push(
            //     }
            //     println!("{:?}", d);
            // }
        }
        out
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

    #[test]
    fn get_numbers() {
        let s = Schematic::from_str("..35..633.");
        println!("{:?}", s);
        let fields = Schematic::get_numbers(&s.grid[0]);
        println!("{:?}", fields);
        assert_eq!(fields,
            vec![(3, 2), 
                (5, 3)
                (6, 6)
                (3, 7)
                (3, 8)
                ]);
    }

    #[test]
    fn example1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn example2() {
        todo!();
    }
}

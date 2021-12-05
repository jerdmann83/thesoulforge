use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Tracker {
    row: (u32, u32),
    col: (u32, u32),
}

impl Tracker {
    fn new() -> Self {
        Tracker {
            row: (0, 127),
            col: (0, 7),
        }
    }

    fn select(&mut self, cmd: &str) {
        for c in cmd.chars() {
            self.on_char(&c);
        }
    }

    fn on_char(&mut self, c: &char) {
        match c {
            'F' => self.row.1 = mid_low(self.row.0, self.row.1),
            'B' => self.row.0 = mid_high(self.row.0, self.row.1),
            'L' => self.col.1 = mid_low(self.col.0, self.col.1),
            'R' => self.col.0 = mid_high(self.col.0, self.col.1),
            _ => todo!(),
        }
    }

    fn get_seat_id(&self) -> Option<u32> {
        if self.row.0 != self.row.1 || self.col.0 != self.col.1 {
            return None;
        }
        return Some((self.row.0 * 8) + self.col.0);
    }
}

fn mid_low(x: u32, y: u32) -> u32 {
    (x + y) / 2
}
fn mid_high(x: u32, y: u32) -> u32 {
    (x + y) / 2 + 1
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut seat_ids: Vec<u32> = vec![];
    let mut ids = HashSet::new();
    for l in buf.split_whitespace() {
        let mut t = Tracker::new();
        t.select(l);
        if let Some(sid) = t.get_seat_id() {
            seat_ids.push(sid);
            ids.insert(sid);
        }
    }

    seat_ids.sort();
    let min_seat_id = seat_ids[0];
    let max_seat_id = seat_ids[seat_ids.len() - 1];

    for idx in min_seat_id..max_seat_id {
        if !ids.contains(&idx) {
            println!("{}", idx);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn partition() {
        // sequence grabbed from the example
        let mut t = Tracker::new();
        t.on_char(&'F');
        assert_eq!(t.row.0, 0);
        assert_eq!(t.row.1, 63);

        t.on_char(&'B');
        assert_eq!(t.row.0, 32);
        assert_eq!(t.row.1, 63);

        t.on_char(&'F');
        assert_eq!(t.row.0, 32);
        assert_eq!(t.row.1, 47);

        t.on_char(&'B');
        assert_eq!(t.row.0, 40);
        assert_eq!(t.row.1, 47);
    }
}

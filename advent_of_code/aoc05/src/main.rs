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

    let mut max_seat_id: u32 = 0;
    let mut seat_ids: Vec<u32> = vec![];
    for l in buf.split_whitespace() {
        let mut t = Tracker::new();
        for c in l.chars() {
            t.on_char(&c);
        }
        if let Some(sid) = t.get_seat_id() {
            seat_ids.push(sid);
        }
    }
    seat_ids.sort();
    for idx in &seat_ids[1..seat_ids.len() - 2] {
        if &seat_ids[(idx + 1) as usize] - &seat_ids[(idx - 1) as usize] == 2 {
            println!("{:?}", &seat_ids[*idx as usize]);
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

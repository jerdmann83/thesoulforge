use std::io::{stdin, Read};

type NumT = usize;
type NumVec = Vec<NumT>;

/// Simple tracking struct for an n-ary vector of integers.
#[derive(Debug)]
struct Cursor {
    v: NumVec,
    lim: usize,
}

impl Cursor {
    pub fn new(lim: usize, slots: usize) -> Self {
        let mut c = Cursor {
            v: vec![],
            lim: lim,
        };
        for _ in 0..slots {
            c.v.push(0);
        }
        c
    }

    /// Increment values forward like clockwork.  A 3-dimensional vector with a
    /// lim of 60 will increment from 0.0.0 to 59.59.59, one step for each
    /// invocation of the method.
    pub fn bump(&mut self) -> bool {
        let mut cur = 0;
        while cur < self.v.len() {
            self.v[cur] += 1;
            if self.v[cur] < self.lim {
                return true;
            }
            self.v[cur] = 0;
            cur += 1;
        }
        cur < self.v.len()
    }

    /// Clockwork bump plus skip non-unique slot combinations.
    pub fn bump_unique(&mut self) -> bool {
        self.bump();
        while !self.is_unique() {
            if !self.bump() {
                return false;
            }
        }
        return true;
    }

    /// Return if all cursor slots contain unique values.  Values
    /// like 12.34.56 will return true while 1.1.2 will return false.
    pub fn is_unique(&self) -> bool {
        if self.v.len() <= 1 {
            return true;
        }

        let mut x = 0;
        let mut y = 1;
        while x < self.v.len() - 1 {
            if self.v[x] == self.v[y] {
                return false;
            }
            y += 1;
            if y == self.v.len() {
                x += 1;
                y = x + 1;
            }
        }
        return true;
    }

    /// Returns the selected slots of a given input vector.  If our cursor is
    /// 0.1.2 and the input is 10.11.12.13..., the returned vector is
    /// 10.11.12
    pub fn select(&self, v: &NumVec) -> Option<NumVec> {
        let mut out: NumVec = vec![];
        for slot in &self.v {
            out.push(v[*slot]);
        }
        Some(out)
    }
}

fn solve(target: NumT, nums: &NumVec, lim: usize) -> Option<NumT> {
    let mut cur = Cursor::new(nums.len(), lim);
    let mut sum: NumT;
    while cur.bump_unique() {
        let vals = cur.select(nums);
        sum = vals.unwrap().iter().sum();

        if sum == target {
            let vals = cur.select(nums);
            return Some(vals.unwrap().iter().product::<NumT>());
        }
    }
    None
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut nums: NumVec = vec![];
    for l in buf.split_whitespace() {
        nums.push(l.parse::<NumT>().unwrap());
    }

    match solve(2020, &nums, 3) {
        Some(n) => println!("{}", n),
        None => println!("{}", "no solution"),
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bump() {
        let mut c = Cursor::new(20, 3);
        let mut i = 0;
        while c.bump() {
            i += 1
        }
        // Magic values stolen from a web permutations calculator thing.
        assert_eq!(i, 7999);

        let mut c = Cursor::new(20, 3);
        let mut i = 0;
        while c.bump_unique() {
            i += 1
        }
        assert_eq!(i, 6840);
    }

    #[test]
    fn select() {
        let mut c = Cursor::new(5, 3);
        c.bump_unique();
        c.bump_unique();

        // cursor is [3, 1, 0]
        let input = vec![50, 51, 52, 53, 54, 55];
        assert_eq!(c.select(&input).unwrap(), vec![53, 51, 50]);
    }
}

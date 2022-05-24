type Lookup = Vec<Vec<usize>>;

fn make_lookup(lhs: &str, rhs: &str) -> Lookup {
    let mut out = vec![];
    for _ in 0..lhs.len() {
        out.push(vec![0 as usize; rhs.len()]);
    }
    out
}

pub fn substring(lhs: &str, rhs: &str) -> String {
    let mut lookup = make_lookup(lhs, rhs);
    let mut max_val = 0;
    let mut max_x = 0;
    for y in 0..lhs.len() {
        for x in 0..rhs.len() {
            let same = rhs.as_bytes()[x] == lhs.as_bytes()[y];
            if !same {
                lookup[y][x] = 0;
                continue;
            }

            let cur_val;
            let last_x = x as i64 - 1;
            let last_y = y as i64 - 1;
            if last_x < 0 || last_y < 0 {
                lookup[y][x] = 1;
                cur_val = 1;
            } else {
                lookup[y][x] = lookup[last_y as usize][last_x as usize] + 1;
                cur_val = lookup[y][x];
            }
            if cur_val > max_val {
                max_val = cur_val;
                max_x = x;
            }
        }
    }

    let mut out = String::new();
    if max_val > 0 {
        let mut cur_x = max_x + 1 - max_val;
        for _ in 0..max_val {
            out = format!("{}{}", out, rhs.as_bytes()[cur_x] as char);
            cur_x += 1;
        }
    }
    out
}

pub fn subsequence(lhs: &str, rhs: &str) -> usize {
    let mut lookup = make_lookup(lhs, rhs);
    let mut max_val = 0;
    for y in 0..lhs.len() {
        for x in 0..rhs.len() {
            let cur_val;
            let last_x = x as i64 - 1;
            let last_y = y as i64 - 1;
            let same = rhs.as_bytes()[x] == lhs.as_bytes()[y];
            if !same {
                let top = if last_y >= 0 {
                    lookup[last_y as usize][x]
                } else {
                    0
                };
                let left = if last_x >= 0 {
                    lookup[y][last_x as usize]
                } else {
                    0
                };
                lookup[y][x] = std::cmp::max(top, left);
                continue;
            }

            if last_x < 0 || last_y < 0 {
                lookup[y][x] = 1;
                cur_val = 1;
            } else {
                lookup[y][x] = lookup[last_y as usize][last_x as usize] + 1;
                cur_val = lookup[y][x];
            }
            if cur_val > max_val {
                max_val = cur_val;
            }
        }
    }

    max_val
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_substring() {
        let mut vals = vec!["foobar", "noloobar", "oobar"];
        assert_eq!(substring(vals[0], vals[1]), vals[2].to_string());
        assert_eq!(substring(vals[1], vals[0]), vals[2].to_string());

        assert_eq!(substring("foobar", "noloobar"), "oobar".to_string());

        vals = vec!["zubat", "rubatees", "ubat"];
        assert_eq!(substring(vals[0], vals[1]), vals[2].to_string());
        assert_eq!(substring(vals[1], vals[0]), vals[2].to_string());

        vals = vec!["floozy", "floory", "floo"];
        assert_eq!(substring(vals[0], vals[1]), vals[2].to_string());
        assert_eq!(substring(vals[1], vals[0]), vals[2].to_string());
    }

    #[test]

    fn test_subsequence() {
        let vals = vec!["floopsy", "totipsy"];
        let expect = 4;
        assert_eq!(subsequence(vals[0], vals[1]), expect);
        assert_eq!(subsequence(vals[1], vals[0]), expect);
    }
}

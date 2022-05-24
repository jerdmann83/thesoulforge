pub struct Parens;

impl Parens {
    pub fn is_valid(s: &str) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            println!("{}", c);
            match c {
                '[' | '(' | '{' => {
                    stack.push(c);
                }
                ']' | ')' | '}' => {
                    let expect: char;
                    if c == ']' {
                        expect = '[';
                    } else if c == ')' {
                        expect = '(';
                    } else if c == '}' {
                        expect = '{';
                    } else {
                        unreachable!()
                    }
                    if stack.pop().unwrap_or(' ') != expect {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Parens;

    #[test]
    fn it_works() {
        assert!(Parens::is_valid("[[[]]]"));
        assert!(Parens::is_valid("[[()[{}](())]]"));
        assert!(!Parens::is_valid("[[(){}]("));
    }
}

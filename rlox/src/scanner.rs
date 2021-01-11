use crate::lox::*;
use crate::token::*;
use crate::token_type::*;
use std::cell::RefCell;

// rust has builtins for these but they allow for lots of stuff in the ut8 range
// that I don't want to deal with here.  just roll my own
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

fn is_alphanum(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

fn keyword(s: &str) -> Option<TokenType> {
    match s {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: RefCell<usize>,
    line: usize,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        Scanner {
            source: s.to_string(),
            tokens: vec![],
            start: 0,
            current: RefCell::new(0),
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        *self.current.borrow() >= self.source.len()
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // at the beginning of the next lexeme
            self.start = *self.current.borrow();
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }

            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }

            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }

            '/' => {
                if self.is_match('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    Lox::error(self.line, "unterminated string");
                    return;
                }

                // consume the closing "
                self.advance();

                // grab the contents in between the "'s
                let b = &self.source.as_bytes()[self.start + 1..*self.current.borrow() - 1];
                let s = String::from_utf8(b.to_vec()).unwrap();
                self.add_token(TokenType::String(s));
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}

            '\n' => {
                self.line += 1;
            }

            n => {
                if is_digit(n) {
                    while is_digit(self.peek()) {
                        self.advance();
                    }

                    // make sure there's another digit behind the ., then
                    // consume the . itself
                    if self.peek() == '.' && is_digit(self.peek_next()) {
                        self.advance();
                    }

                    while is_digit(self.peek()) {
                        self.advance();
                    }

                    let val = self.source[self.start..*self.current.borrow()]
                        .parse::<f64>()
                        .unwrap();
                    self.add_token(TokenType::Number(val));
                } else if is_alpha(n) {
                    while is_alphanum(self.peek()) {
                        self.advance();
                    }
                    let cur = &self.current_string();
                    let tt = keyword(cur);
                    match tt {
                        Some(t) => self.add_token(t),
                        None => self.add_token(TokenType::Identifier(cur.to_string())),
                    }
                } else {
                    Lox::error(self.line, &format!("unexpected '{}'", n));
                    todo!();
                }
            }
        }
    }

    fn advance(&self) -> char {
        self.bump_current(1);
        return self.current_char();
    }

    fn bump_current(&self, c: usize) {
        self.current.replace_with(|&mut cur| cur + c);
    }

    fn is_match(&self, expect: char) -> bool {
        if self.peek() == expect {
            self.bump_current(1);
            return true;
        }
        false
    }

    fn current_char(&self) -> char {
        // lazy assume ascii and just index by byte position
        // will totally explode or do something stupid in the (unlikely) case we
        // have like emoji in the source code
        let c = self.source.as_bytes()[*self.current.borrow() - 1] as char;
        c
    }

    fn current_string(&self) -> String {
        let b = &self.source.as_bytes()[self.start..*self.current.borrow()];
        String::from_utf8(b.to_vec()).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[*self.current.borrow()] as char
    }

    fn peek_next(&self) -> char {
        if *self.current.borrow() + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[*self.current.borrow() + 1] as char
    }

    fn add_token(&mut self, tt: TokenType) {
        let text = &self.source[self.start..*self.current.borrow()];
        let token = Token::new(tt, text, self.line);
        self.tokens.push(token);
    }
}

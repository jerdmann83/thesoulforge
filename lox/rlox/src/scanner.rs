use crate::lox::*;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        Scanner {
            source: s.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        let c = 'a';
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
                    self.current += 1;
                } else {
                    self.add_token(TokenType::Bang);
                }
            }

            n => Lox::error(self.line, &format!("unexpected '{}'", n)),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.next_char();
    }

    // would really like to current++ here but I can't figure out how to call a
    // &mut method from another &mut method yet.  git gud at rust
    fn is_match(&self, expect: char) -> bool {
        self.next_char() == expect
    }

    fn next_char(&self) -> char {
        // lazy assume ascii and just index by byte position
        // will totally explode or do something stupid in the (unlikely) case we
        // have like emoji in the source code
        self.source.as_bytes()[self.current - 1] as char
    }

    fn peek(&self, expect: char) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.next_char()

        // if self.next_char() != expect {
        //     return false;
        // }

        // self.current += 1;
        // return true;
    }

    // fn add_empty_token(&mut self, tt: TokenType) {
    //     self.add_token(
    // }

    fn add_token(&mut self, tt: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(tt, text.to_string(), self.line));
    }
}

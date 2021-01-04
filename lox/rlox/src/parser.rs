use crate::expr::*;
use crate::lox::*;
use crate::token::*;
use crate::token_type::*;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

pub type ParseResult = Result<Expr, ParseError>;
type ConsumeResult = Result<(), ParseError>;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: RefCell::new(0),
        }
    }

    pub fn parse(&self) -> ParseResult {
        self.expression()
    }

    fn is_match(&self, tts: &[TokenType]) -> bool {
        for tt in tts {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&self, tt: &TokenType, msg: &str) -> ConsumeResult {
        if self.check(tt) {
            self.advance();
            return Ok(());
        }
        Err(ParseError::new(msg))
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        let cur = self.peek().ttype;
        let out = std::mem::discriminant(&cur) == std::mem::discriminant(&tt);
        out
    }

    fn advance(&self) {
        if !self.is_at_end() {
            self.bump_current(1);
        }
    }

    fn peek(&self) -> Token {
        // todo: return a reference, figure out explicit lifetimes
        self.tokens[*self.current.borrow()].clone()
    }

    fn previous(&self) -> Token {
        // todo: same here
        self.tokens[*self.current.borrow() - 1].clone()
    }

    fn error(&self, t: &Token, msg: &str) -> ParseError {
        Lox::error(t.line, msg);
        ParseError::new(msg)
    }

    fn bump_current(&self, c: usize) {
        self.current.replace_with(|&mut cur| cur + c);
    }

    fn is_at_end(&self) -> bool {
        assert!(*self.current.borrow() < self.tokens.len());
        self.peek().ttype == TokenType::EOF
    }

    /// grammar rules:
    /// expression     -> equality ;
    /// equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
    /// comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    /// term           -> factor ( ( "-" | "+" ) factor )* ;
    /// factor         -> unary ( ( "/" | "*" ) unary )* ;
    /// unary          -> ( "!" | "-" ) unary
    ///                | primary ;
    /// primary        -> NUMBER | STRING | "true" | "false" | "nil"
    ///                | "(" expression ")" ;

    fn expression(&self) -> ParseResult {
        self.equality()
    }

    fn equality(&self) -> ParseResult {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::Equal]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn comparison(&self) -> ParseResult {
        let mut expr = self.term()?;
        // println!("comparison: {:?}", expr);

        while self.is_match(&[
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::LessEqual,
            TokenType::Less,
        ]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn term(&self) -> ParseResult {
        let mut expr = self.factor()?;
        // println!("term: {:?}", expr);

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn factor(&self) -> ParseResult {
        let mut expr = self.unary()?;
        // println!("factor: {:?}", expr);

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(operator, expr, right);
            // println!("add factor: {:?}", expr);
        }

        Ok(expr)
    }

    fn unary(&self) -> ParseResult {
        let mut expr = self.primary()?;
        // println!("unary: {:?}", expr);

        while self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::new_unary(operator, right);
        }

        Ok(expr)
        // self.primary()
    }

    fn primary(&self) -> ParseResult {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::new_literal(self.previous()));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::new_literal(self.previous()));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::new_literal(self.previous()));
        }
        // possibly a more idiomatic way to do this exists.  for now just hack
        // in arbitrary values as the matching logic only cares about the
        // variant types themselves, not the contained values
        if self.is_match(&[TokenType::Number(0.0), TokenType::String("".to_string())]) {
            return Ok(Expr::new_literal(self.previous()));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "expect ')' after expression.")?;
            return Ok(Expr::new_grouping(&[expr]));
        }

        Err(ParseError::new(
            &format!("no rule for expression '{:?}'", self.peek()).to_string(),
        ))
    }

    fn synchronize(&self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}

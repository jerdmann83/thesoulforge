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

        self.previous().ttype == *tt
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
        println!("{} {}", *self.current.borrow(), self.tokens.len());
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

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn factor(&self) -> ParseResult {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn unary(&self) -> ParseResult {
        let mut expr = self.primary()?;

        while self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::new_unary(operator, right);
            return Ok(expr);
        }

        self.primary()
    }

    fn primary(&self) -> ParseResult {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::new_literal_default(TokenType::False));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::new_literal_default(TokenType::True));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::new_literal_default(TokenType::Nil));
        }
        // todo: how to match on tuple variants where we don't care about the
        // contained value?  hack in arbitrary values for now
        if self.is_match(&[TokenType::Number(0.0), TokenType::String("".to_string())]) {
            return Ok(Expr::new_literal_default(self.previous().ttype));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(&TokenType::RightParen, "expect ')' after expression.");
        }

        Err(ParseError::new("no rule for expression."))
    }

    fn synchronize(&self) {
        self.advance();

        while (!self.is_at_end()) {
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

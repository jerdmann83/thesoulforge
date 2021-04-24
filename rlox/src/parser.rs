use crate::expr::*;
use crate::lox::*;
use crate::stmt::*;
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

pub type ParseResult = Result<Vec<Stmt>, ParseError>;

type StmtResult = Result<Stmt, ParseError>;
type ExprResult = Result<Expr, ParseError>;

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
        let mut stmts = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.error(&self.previous(), &e.msg);
                    return Err(e);
                }
            }
        }

        Ok(stmts)
    }

    fn is_match(&self, tts: &[TokenType]) -> bool {
        for tt in tts {
            if self.check(tt.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&self, tt: TokenType, msg: &str) -> ConsumeResult {
        if self.check(tt) {
            self.advance();
            return Ok(());
        }
        Err(ParseError::new(msg))
    }

    fn check(&self, tt: TokenType) -> bool {
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

    fn declaration(&self) -> StmtResult {
        if self.is_match(&[TokenType::Var]) {
            return self.var_declaration();
        }

        match self.statement() {
            Ok(stmt) => return Ok(stmt),
            Err(err) => {
                self.synchronize();
                return Err(err);
            }
        }
    }

    fn var_declaration(&self) -> StmtResult {
        self.consume(TokenType::Identifier(String::new()), "expect variable name")?;
        let tok = self.previous();

        if self.is_match(&[TokenType::Semicolon]) {
            return Ok(Stmt::new_var(&tok));
        }

        if !self.is_match(&[TokenType::Equal]) {
            return Err(ParseError::new(&format!("expect '=' after '{:?}'", tok)));
        }

        let initializer = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "expect ';' after variable declaration",
        )?;
        return Ok(Stmt::new_var_init(&tok, &initializer));
    }

    fn statement(&self) -> StmtResult {
        if self.is_match(&[TokenType::If]) {
            return self.if_stmt();
        }
        if self.is_match(&[TokenType::Print]) {
            return self.print_stmt();
        }
        if self.is_match(&[TokenType::While]) {
            return self.while_stmt();
        }
        if self.is_match(&[TokenType::For]) {
            return self.for_stmt();
        }
        if self.is_match(&[TokenType::LeftBrace]) {
            return self.block();
        }
        return self.expr_stmt();
    }

    fn if_stmt(&self) -> StmtResult {
        self.consume(TokenType::LeftParen, "expect '(' after if")?;
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "expect ')' after condition")?;

        let then = self.statement()?;
        let mut els = None;
        if self.is_match(&[TokenType::Else]) {
            els = Some(self.statement()?);
        }

        Ok(Stmt::new_if(&cond, &then, &els))
    }

    fn for_stmt(&self) -> StmtResult {
        self.consume(TokenType::LeftParen, "expect '(' after for")?;
        let mut init: Option<Stmt>;
        if self.is_match(&[TokenType::Semicolon]) {
            init = None;
        } else if self.is_match(&[TokenType::Var]) {
            init = Some(self.var_declaration()?);
        } else {
            init = Some(self.expr_stmt()?);
        }

        let mut cond: Option<Expr>;
        if self.check(TokenType::Semicolon) {
            cond = None;
        } else {
            cond = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "expect ';' after loop condition")?;

        let mut incr: Option<Expr>;
        if self.check(TokenType::RightParen) {
            incr = None;
        } else {
            incr = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "expect ';' after loop condition")?;

        let body = self.statement()?;

        // desugar the above components into a base expr
        // if let Some(incr_exp) = incr {
        //     body = Stmt::new_block(
        //         body,
        //         Stmt::new_expr());
        // }
        todo!();
    }

    fn print_stmt(&self) -> StmtResult {
        let val = self.expression()?;
        self.consume(TokenType::Semicolon, "expect ';' after print statement")?;
        return Ok(Stmt::new_print(&val));
    }

    fn while_stmt(&self) -> StmtResult {
        self.consume(TokenType::LeftParen, "expect '(' after while");
        let cond = self.expression()?;
        self.consume(TokenType::RightParen, "expect ')' after condition");
        let body = self.statement()?;

        Ok(Stmt::new_while(&cond, &body))
    }

    fn block(&self) -> StmtResult {
        let mut stmts = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "expect '}' after block")?;
        Ok(Stmt::new_block(&stmts))
    }

    fn expr_stmt(&self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "expect ';' after statement")?;
        Ok(Stmt::new_expr(&expr))
    }

    fn expression(&self) -> ExprResult {
        self.assignment()
    }

    fn assignment(&self) -> ExprResult {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous();
            let val = self.assignment()?;

            if expr.etype == ExprType::Variable {
                let name = expr.token;
                return Ok(Expr::new_assign(name, val));
            }

            self.error(&equals, "invalid assignment target");
        }

        Ok(expr)
    }

    fn or(&self) -> ExprResult {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::new_or(expr, op, right);
        }
        Ok(expr)
    }

    fn and(&self) -> ExprResult {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Expr::new_and(expr, op, right);
        }
        Ok(expr)
    }

    fn equality(&self) -> ExprResult {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn comparison(&self) -> ExprResult {
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

    fn term(&self) -> ExprResult {
        let mut expr = self.factor()?;
        // println!("term: {:?}", expr);

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::new_binary(operator, expr, right);
        }

        Ok(expr)
    }

    fn factor(&self) -> ExprResult {
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

    fn unary(&self) -> ExprResult {
        while self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    fn primary(&self) -> ExprResult {
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
            self.consume(TokenType::RightParen, "expect ')' after expression.")?;
            return Ok(Expr::new_grouping(&expr));
        }

        if self.is_match(&[TokenType::Identifier(String::new())]) {
            return Ok(Expr::new_var(self.previous()));
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast_printer::*;

    #[test]
    fn test_parser() {
        let p = Parser::new(&[
            Token::new(TokenType::Number(5.0), "5", 1),
            Token::new(TokenType::Equal, "=", 1),
            Token::new(TokenType::Number(1.0), "1", 1),
            Token::new(TokenType::Plus, "+", 1),
            Token::new(TokenType::Number(9.0), "9", 1),
            Token::new(TokenType::Minus, "-", 1),
            Token::new(TokenType::Number(4.0), "4", 1),
            Token::new(TokenType::EOF, "", 1),
        ]);
        let stmts = p.parse().unwrap();
        println!("{:?}", AstPrinter::serialize_stmts(&stmts));
    }
}

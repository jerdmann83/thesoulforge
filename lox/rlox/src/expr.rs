use std::cell::RefCell;
use std::rc::Rc;

use crate::token::*;
use crate::token_type::*;

pub type ExprRef = Rc<RefCell<Expr>>;

#[derive(Clone, Copy, Debug)]
pub enum ExprType {
    Binary,
    Unary,
    Grouping,
    Literal,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub etype: ExprType,
    pub token: Token,
    pub children: Vec<ExprRef>,
}

impl Expr {
    pub fn new_binary(token: Token, left: ExprRef, right: ExprRef) -> ExprRef {
        let e = Expr {
            etype: ExprType::Binary,
            token: token,
            children: vec![left, right],
        };
        Rc::new(RefCell::new(e))
    }

    pub fn new_unary(token: Token, node: ExprRef) -> ExprRef {
        let e = Expr {
            etype: ExprType::Unary,
            token: token,
            children: vec![node],
        };
        Rc::new(RefCell::new(e))
    }

    pub fn new_grouping(exprs: &[ExprRef]) -> ExprRef {
        let e = Expr {
            etype: ExprType::Grouping,
            // hack in a token type because I really don't want to deal with
            // token nullability in however many places these things appear
            token: Token::new(TokenType::EOF, "", 0),
            children: exprs.to_vec(),
        };
        Rc::new(RefCell::new(e))
    }

    pub fn new_literal(token: Token) -> ExprRef {
        let e = Expr {
            etype: ExprType::Literal,
            token: token,
            children: vec![],
        };
        Rc::new(RefCell::new(e))
    }
}

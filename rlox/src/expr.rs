use crate::token::*;
use crate::token_type::*;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExprType {
    Assign,
    Binary,
    Unary,
    Grouping,
    Literal,
    Variable,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub etype: ExprType,
    pub token: Token,
    pub children: Vec<Expr>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("type:{:?} children:", self.etype));
        for c in &self.children {
            buf.push_str(&format!("{}", c));
        }
        write!(f, "{}", buf)
    }
}

impl Expr {
    pub fn new_assign(token: Token, val: Expr) -> Expr {
        let e = Expr {
            etype: ExprType::Assign,
            token: token,
            children: vec![val],
        };
        e
    }

    pub fn new_binary(token: Token, left: Expr, right: Expr) -> Expr {
        let e = Expr {
            etype: ExprType::Binary,
            token: token,
            children: vec![left, right],
        };
        e
    }

    pub fn new_unary(token: Token, node: Expr) -> Expr {
        let e = Expr {
            etype: ExprType::Unary,
            token: token,
            children: vec![node],
        };
        e
    }

    pub fn new_grouping(expr: &Expr) -> Expr {
        let e = Expr {
            etype: ExprType::Grouping,
            // hack in a token type because I really don't want to deal with
            // token nullability in however many places these things appear
            token: Token::new(TokenType::EOF, "", 0),
            children: vec![expr.clone()],
        };
        e
    }

    pub fn new_literal(token: Token) -> Expr {
        let e = Expr {
            etype: ExprType::Literal,
            token: token,
            children: vec![],
        };
        e
    }

    pub fn new_var_init(token: Token, initializer: &Expr) -> Expr {
        let e = Expr {
            etype: ExprType::Variable,
            token: token,
            children: vec![initializer.clone()],
        };
        e
    }

    pub fn new_var(token: Token) -> Expr {
        let e = Expr {
            etype: ExprType::Variable,
            token: token,
            children: vec![],
        };
        e
    }
}

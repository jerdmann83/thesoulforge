use crate::token::*;
use crate::token_type::*;

#[derive(Clone, Copy, Debug)]
pub enum ExprType {
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

/// expression grammar
/// expression     → literal
///                | unary
///                | binary
///                | grouping ;
///
/// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
/// grouping       → "(" expression ")" ;
/// unary          → ( "-" | "!" ) expression ;
/// binary         → expression operator expression ;
/// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
///                | "+"  | "-"  | "*" | "/" ;
///
/// primary        → "true" | "false" | "nil"
///               | NUMBER | STRING
///               | "(" expression ")"
///               | IDENTIFIER ;

impl Expr {
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

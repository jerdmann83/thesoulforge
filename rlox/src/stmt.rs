use crate::expr::*;
use crate::token::*;

/// statement grammar
/// program        → declaration* EOF ;
///
/// declaration    → varDecl
///                | statement ;
///
/// varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;
/// statement      → exprStmt
///                | printStmt ;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}

impl Stmt {
    pub fn new_expr(expr: &Expr) -> Stmt {
        Stmt::Expr(expr.clone())
    }

    pub fn new_print(expr: &Expr) -> Stmt {
        Stmt::Print(expr.clone())
    }

    pub fn new_var(name: &Token) -> Stmt {
        Stmt::Var(name.clone(), None)
    }

    pub fn new_var_init(name: &Token, initializer: &Expr) -> Stmt {
        Stmt::Var(name.clone(), Some(initializer.clone()))
    }
}

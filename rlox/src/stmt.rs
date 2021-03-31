use crate::expr::*;
use crate::token::*;
use std::fmt;

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

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "expr:{}", expr),
            Stmt::Print(expr) => write!(f, "print expr:{}", expr),
            Stmt::Var(token, oexpr) => {
                if let Some(expr) = oexpr {
                    write!(f, "var:{} expr:{}", token.lexeme, expr)
                } else {
                    write!(f, "var:{} expr:none", token.lexeme)
                }
            }
        }
    }
}

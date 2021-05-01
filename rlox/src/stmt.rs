use crate::expr::*;
use crate::token::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    Block(Vec<Stmt>),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
}

impl Stmt {
    pub fn new_expr(expr: &Expr) -> Stmt {
        Stmt::Expr(expr.clone())
    }

    pub fn new_print(expr: &Expr) -> Stmt {
        Stmt::Print(expr.clone())
    }

    pub fn new_if(expr: &Expr, then: &Stmt, els: &Option<Stmt>) -> Stmt {
        Stmt::If(expr.clone(), Box::new(then.clone()), Box::new(els.clone()))
    }

    pub fn new_var(name: &Token) -> Stmt {
        Stmt::Var(name.clone(), None)
    }

    pub fn new_var_init(name: &Token, initializer: &Expr) -> Stmt {
        Stmt::Var(name.clone(), Some(initializer.clone()))
    }

    pub fn new_block(stmts: &Vec<Stmt>) -> Stmt {
        Stmt::Block(stmts.clone())
    }

    pub fn new_while(cond: &Expr, body: &Stmt) -> Stmt {
        Stmt::While(cond.clone(), Box::new(body.clone()))
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "\nexpr:{:?}", expr),
            Stmt::Print(expr) => write!(f, "\nprint expr:{:?}", expr),
            Stmt::If(expr, _then, _else) => write!(f, "\nif expr:{:?}", expr),
            Stmt::Block(stmts) => {
                write!(f, "\nblock")?;
                for stmt in stmts {
                    write!(f, "\n{:?}", stmt)?;
                }
                Ok(())
                // let mut rc;
            }
            Stmt::While(cond, block) => {
                write!(f, "\nwhile")?;
                write!(f, "\ncond {:?}", cond)?;
                write!(f, "\nblock {:?}", block)?;
                Ok(())
            }
            Stmt::Var(token, oexpr) => {
                if let Some(expr) = oexpr {
                    write!(f, "\nvar:{:?} expr:{:?}", token.lexeme, expr)
                } else {
                    write!(f, "\nvar:{:?} expr:none", token.lexeme)
                }
            }
        }
    }
}

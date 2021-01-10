use crate::expr::*;

/// statement grammar
/// program        → statement* EOF ;

/// statement      → exprStmt
///                | printStmt ;

/// exprStmt       → expression ";" ;
/// printStmt      → "print" expression ";" ;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}

// #[derive(Clone, Debug)]
// pub struct Stmt {
//     pub stype: StmtType,
//     // pub children: Vec<Stmt>,
// }

impl Stmt {
    pub fn new_expr(expr: &Expr) -> Stmt {
        Stmt::Expr(expr.clone())
    }

    pub fn new_print(expr: &Expr) -> Stmt {
        Stmt::Print(expr.clone())
    }
}

use crate::environment::*;
use crate::error::*;
use crate::expr::*;
use crate::lox::*;
use crate::stmt::*;
use crate::token::*;
use crate::token_type::*;
use crate::value::*;

use std::cell::RefCell;

pub type InterpreterResult = Result<Value, RuntimeError>;
pub type ExecuteResult = Result<(), RuntimeError>;

pub struct Interpreter {
    env: RefCell<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: RefCell::new(Environment::new()),
        }
    }

    pub fn interpret(&self, stmts: &Vec<Stmt>) -> ExecuteResult {
        for stmt in stmts {
            match self.execute(&stmt) {
                Err(n) => {
                    Lox::runtime_error(&n.msg);
                    return Err(n);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn execute(&self, stmt: &Stmt) -> ExecuteResult {
        self.eval_stmt(&stmt)
    }

    pub fn eval_stmt(&self, stmt: &Stmt) -> ExecuteResult {
        match stmt {
            Stmt::Expr(expr) => {
                let _ = self.eval(&expr)?;
            }
            Stmt::Var(token, expr) => self.eval_var(&token, &expr)?,
            Stmt::Print(expr) => self.eval_print(&expr)?,
        }
        Ok(())
    }

    fn eval(&self, expr: &Expr) -> InterpreterResult {
        match expr.etype {
            ExprType::Grouping => return self.eval(&expr.children[0]),
            ExprType::Assign => return self.eval_assign(&expr),
            ExprType::Literal => return self.eval_literal(&expr),
            ExprType::Binary => return self.eval_binary(&expr),
            ExprType::Unary => return self.eval_unary(&expr),
            ExprType::Variable => {
                let env = self.env.borrow();
                let name = &expr.token.lexeme;
                match env.get(name, &expr.token.line) {
                    Ok(v) => return Ok(v),
                    Err(e) => return Err(RuntimeError::new(&e.msg, 0)),
                }
            }
        }
    }

    fn eval_assign(&self, expr: &Expr) -> InterpreterResult {
        let val = self.eval(&expr.children[0])?;
        let mut env = self.env.borrow_mut();
        env.assign(&expr.token.lexeme, &val, &expr.token.line)?;
        Ok(val)
    }

    fn eval_literal(&self, expr: &Expr) -> InterpreterResult {
        match &expr.token.ttype {
            TokenType::String(s) => return Ok(Value::String(s.to_string())),
            TokenType::Number(n) => return Ok(Value::Number(*n)),
            TokenType::True => return Ok(Value::Bool(true)),
            TokenType::False => return Ok(Value::Bool(false)),
            TokenType::Nil => return Ok(Value::Nil),
            _ => {
                return Err(RuntimeError::new(
                    &format!("unhandled literal {:?}", expr.token.lexeme),
                    expr.token.line,
                ))
            }
        }
    }

    fn eval_binary(&self, expr: &Expr) -> InterpreterResult {
        let left = self.eval(&expr.children[0])?;
        let right = self.eval(&expr.children[1])?;
        if let (Value::Number(ln), Value::Number(rn)) = (&left, &right) {
            match expr.token.ttype {
                TokenType::Minus => return Ok(Value::Number(ln - rn)),
                TokenType::Plus => return Ok(Value::Number(ln + rn)),
                TokenType::Slash => return Ok(Value::Number(ln / rn)),
                TokenType::Star => return Ok(Value::Number(ln * rn)),

                TokenType::Greater => return Ok(Value::Bool(ln > rn)),
                TokenType::GreaterEqual => return Ok(Value::Bool(ln >= rn)),
                TokenType::Less => return Ok(Value::Bool(ln < rn)),
                TokenType::LessEqual => return Ok(Value::Bool(ln <= rn)),
                _ => {
                    return Err(RuntimeError::new(
                        &format!(
                            "unexpected operator {} for binary arguments {:?} and {:?}",
                            expr.token.lexeme, left, right
                        ),
                        expr.token.line,
                    ))
                }
            }
        }

        if let (Value::String(ls), Value::String(rs)) = (&left, &right) {
            match expr.token.ttype {
                TokenType::Plus => return Ok(Value::String(format!("{}{}", ls, rs))),
                _ => {
                    return Err(RuntimeError::new(
                        &format!(
                            "unexpected operator {} for string arguments {:?} and {:?}",
                            expr.token.lexeme, &ls, &rs
                        ),
                        expr.token.line,
                    ))
                }
            }
        }

        Err(RuntimeError::new(
            &format!(
                "unexpected binary arguments {:?} and {:?} for operator {}",
                left, right, expr.token.lexeme
            ),
            expr.token.line,
        ))
    }

    fn eval_unary(&self, expr: &Expr) -> InterpreterResult {
        let right = self.eval(&expr.children[0])?;
        if let Value::Number(n) = right {
            match expr.token.ttype {
                TokenType::Minus => return Ok(Value::Number(-n)),
                TokenType::Bang => return Ok(Value::Bool(Self::is_truthy(&right))),
                _ => {
                    return Err(RuntimeError::new(
                        &format!("unexpected unary argument {:?}", right),
                        expr.token.line,
                    ))
                }
            }
        }

        return Err(RuntimeError::new(
            &format!("unhandled {:?}", right),
            expr.token.line,
        ));
    }

    fn eval_var(&self, tok: &Token, initializer: &Option<Expr>) -> ExecuteResult {
        let mut val = Value::Nil;
        if let Some(expr) = initializer {
            val = self.eval(expr)?;
        }
        let mut env = self.env.borrow_mut();
        env.define(&tok.lexeme, &val);
        Ok(())
    }

    fn eval_print(&self, expr: &Expr) -> ExecuteResult {
        let val = self.eval(&expr)?;
        println!("{}", val);
        Ok(())
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Nil => false,
            Value::Number(_) => true,
            Value::String(_) => true,
        }
    }

    fn is_equal(a: &Value, b: &Value) -> InterpreterResult {
        if let (Value::Nil, Value::Nil) = (&a, &b) {
            return Ok(Value::Bool(true));
        }
        if let (Value::Number(ln), Value::Number(rn)) = (&a, &b) {
            return Ok(Value::Bool(ln == rn));
        }
        if let (Value::String(ls), Value::String(rs)) = (&a, &b) {
            return Ok(Value::Bool(ls == rs));
        }

        Ok(Value::Bool(false))
    }
}

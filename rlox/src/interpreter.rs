use crate::environment::*;
use crate::error::*;
use crate::expr::*;
use crate::function::*;
use crate::lox::*;
use crate::stmt::*;
use crate::token::*;
use crate::token_type::*;
use crate::value::*;

use std::mem;
use std::time::SystemTime;

pub type InterpreterResult = Result<Value, RuntimeError>;
pub type ExecuteResult = Result<(), RuntimeError>;

#[derive(Debug)]
pub struct Interpreter {
    env: Environment,
}

pub trait Callable {
    fn arity() -> u8;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Expr>) -> Value;
}

struct Clock {}

impl Callable for Clock {
    fn arity() -> u8 {
        0
    }
    fn call(&self, _: &mut Interpreter, _args: Vec<Expr>) -> Value {
        Value::Number(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as f64,
        )
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define(
            "clock",
            // lame hardcoded weird literal for now
            Value::Function(LoxFunction::new(&Stmt::new_expr(&Expr::new_literal(
                Token::new(TokenType::Number(1.0), "1.0", 0),
            )))),
        );
        env.bump();
        Interpreter { env: env }
    }

    // environments basically form a graph structure.  for example, if we're a few blocks deep,
    // then we start running some function calls, and each function opens more blocks in turn, the
    // env tree looks something like:
    //
    // |- globals -> 1 -> 2 -> 3
    // |     |
    // |     v
    // |   fn1 ----> 1
    // |
    // |
    // --> fn2 ----> 1 -> 2
    //
    // instead of dealing with Rc<RefCell> swap headaches, just expose a helper here that clones
    // the environment only at the globals level.  it's less than efficient but it's a hack that
    // buys me time while I continue the journey...
    pub fn new_function_env(&self) -> Environment {
        let mut env = self.env.clone();
        env.reset();
        env
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> ExecuteResult {
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

    pub fn execute(&mut self, stmt: &Stmt) -> ExecuteResult {
        self.eval_stmt(&stmt)
    }

    pub fn eval_stmt(&mut self, stmt: &Stmt) -> ExecuteResult {
        match stmt {
            Stmt::Expr(expr) => {
                let _ = self.eval(&expr)?;
            }
            Stmt::Var(token, expr) => self.eval_var(&token, &expr)?,
            Stmt::Block(stmts) => {
                self.env.bump();
                self.eval_block(stmts)?;
                self.env.debump();
            }
            Stmt::Print(expr) => self.eval_print(&expr)?,
            Stmt::If(expr, then, els) => self.eval_if(expr, then, els)?,
            Stmt::While(cond, body) => self.eval_while(cond, body)?,
        }
        Ok(())
    }

    pub fn eval_if(
        &mut self,
        cond: &Expr,
        then: &Box<Stmt>,
        els: &Box<Option<Stmt>>,
    ) -> ExecuteResult {
        if Self::is_truthy(&self.eval(&cond)?) {
            self.eval_stmt(&*then)?;
        } else {
            if let Some(stmt) = &**els {
                self.eval_stmt(&stmt)?;
            }
        }
        Ok(())
    }

    pub fn eval_while(&mut self, cond: &Expr, body: &Box<Stmt>) -> ExecuteResult {
        while Self::is_truthy(&self.eval(&cond)?) {
            self.eval_stmt(body)?;
        }
        Ok(())
    }

    pub fn eval_block(&mut self, stmts: &Vec<Stmt>) -> ExecuteResult {
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    pub fn eval_function_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: &mut Environment,
    ) -> ExecuteResult {
        mem::swap(&mut self.env, env);
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        mem::swap(&mut self.env, env);
        Ok(())
    }

    pub fn eval(&mut self, expr: &Expr) -> InterpreterResult {
        match expr.etype {
            ExprType::Grouping => return self.eval(&expr.children[0]),
            ExprType::Assign => return self.eval_assign(&expr),
            ExprType::Literal => return self.eval_literal(&expr),
            ExprType::Binary => return self.eval_binary(&expr),
            ExprType::Unary => return self.eval_unary(&expr),
            ExprType::Call => return self.eval_call(&expr),
            ExprType::Variable => {
                let name = &expr.token.lexeme;
                match self.env.get(name, expr.token.line) {
                    Ok(v) => return Ok(v),
                    Err(e) => return Err(RuntimeError::new(&e.msg, expr.token.line)),
                }
            }
            ExprType::Logical => return self.eval_logical(&expr),
        }
    }

    fn eval_assign(&mut self, expr: &Expr) -> InterpreterResult {
        let val = self.eval(&expr.children[0])?;
        self.env
            .assign(&expr.token.lexeme, val.clone(), expr.token.line)?;
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

    fn eval_binary(&mut self, expr: &Expr) -> InterpreterResult {
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

    fn eval_call(&mut self, _expr: &Expr) -> InterpreterResult {
        todo!();
    }

    fn eval_logical(&mut self, expr: &Expr) -> InterpreterResult {
        let left = self.eval(&expr.children[0])?;

        // short-circuit behaviors
        if expr.token.ttype == TokenType::Or {
            // or with a truthy left means we're done
            if Self::is_truthy(&left) {
                return Ok(left);
            }
        } else {
            // and with a falsy left also means we're done
            if !Self::is_truthy(&left) {
                return Ok(left);
            }
        }

        return self.eval(&expr.children[1]);
    }

    fn eval_unary(&mut self, expr: &Expr) -> InterpreterResult {
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

    fn eval_var(&mut self, tok: &Token, initializer: &Option<Expr>) -> ExecuteResult {
        let mut val = Value::Nil;
        if let Some(expr) = initializer {
            val = self.eval(expr)?;
        }
        self.env.define(&tok.lexeme, val);
        Ok(())
    }

    fn eval_print(&mut self, expr: &Expr) -> ExecuteResult {
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
            Value::Function(_) => true,
        }
    }

    #[allow(dead_code)]
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

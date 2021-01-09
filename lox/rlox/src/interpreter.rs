use crate::expr::*;
use crate::token_type::*;
use crate::value::*;

#[derive(Debug)]
pub struct RuntimeError {
    msg: String,
    line: usize,
}

impl RuntimeError {
    fn new(msg: &str, line: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line: line,
        }
    }
}

pub type InterpreterResult = Result<Value, RuntimeError>;

pub struct Interpreter {
    //
}

impl Interpreter {
    pub fn interpret(expr: &Expr) -> InterpreterResult {
        Self::eval(&expr)
    }

    fn eval(expr: &Expr) -> InterpreterResult {
        match expr.etype {
            ExprType::Grouping => return Self::eval(&expr.children[0]),
            ExprType::Literal => return Self::eval_literal(&expr),
            ExprType::Binary => return Self::eval_binary(&expr),
            ExprType::Unary => return Self::eval_unary(&expr),
        }
        //
    }

    fn eval_literal(expr: &Expr) -> InterpreterResult {
        match &expr.token.ttype {
            TokenType::String(s) => return Ok(Value::String(s.to_string())),
            TokenType::Number(n) => return Ok(Value::Number(*n)),
            TokenType::True => return Ok(Value::Bool(true)),
            TokenType::False => return Ok(Value::Bool(false)),
            _ => {
                return Err(RuntimeError::new(
                    &format!("unhandled literal {:?}", expr.token.lexeme),
                    expr.token.line,
                ))
            }
        }
    }

    fn eval_binary(expr: &Expr) -> InterpreterResult {
        let left = Self::eval(&expr.children[0])?;
        let right = Self::eval(&expr.children[1])?;
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
                        &format!("unexpected binary arguments {:?} and {:?}", left, right),
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

    fn eval_unary(expr: &Expr) -> InterpreterResult {
        let right = Self::eval(&expr.children[0])?;
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

use crate::expr::*;
use crate::interpreter::*;
use crate::stmt::*;
use crate::value::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LoxFunction {
    decl: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(stmt: &Stmt) -> Self {
        Self {
            decl: vec![stmt.clone()],
        }
    }
}

impl Callable for LoxFunction {
    fn arity() -> u8 {
        0
    }

    // fn(x, y);
    //   x + y
    //
    // fn(1, 2) -> 3
    //
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Expr>) -> Value {
        let mut env = interpreter.new_function_env();
        for i in 0..args.len() {
            let arg = &args[i];
            let res = interpreter.eval(arg);
            match res {
                Ok(val) => {
                    env.define(&arg.token.lexeme, val);
                }
                Err(_) => {
                    return Value::Nil;
                }
            }
        }

        let _res = interpreter.eval_function_block(&self.decl, &mut env);
        // let val: Value;
        // match res {
        //     Ok(v) => val = v,
        //     Err(_) => val = Value::Nil,
        // }
        // todo: no return values yet
        Value::Nil
    }
}

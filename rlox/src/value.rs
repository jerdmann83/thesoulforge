use crate::function::*;
use std::fmt;

// todo: PartialEq is only required for test comparisons at the moment
// however, PartialEq is kind of viral because every variant must also derive it
// this makes sense so will just keep rolling with it for now
//
// weird confidence boost:  several rust crates I looked at (hello serde!) invoke the below
// incantations all over the place
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function(LoxFunction),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out;
        match self {
            Value::Nil => out = format!("(nil)"),
            Value::Bool(val) => out = format!("{}", val),
            Value::Number(val) => out = format!("{}", val),
            Value::String(val) => out = format!("{}", val),
            Value::Function(_func) => out = format!("fn"),
        }
        write!(f, "{}", out)
    }
}

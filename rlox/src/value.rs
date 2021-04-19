use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out;
        match self {
            Value::Nil => out = format!("(nil)"),
            Value::Bool(val) => out = format!("{}", val),
            Value::Number(val) => out = format!("{}", val),
            Value::String(val) => out = format!("{}", val),
        }
        write!(f, "{}", out)
    }
}

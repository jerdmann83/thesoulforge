use crate::error::*;
use crate::value::*;
use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, val: &Value) {
        self.values.insert(name.to_string(), val.clone());
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        match self.values.get(name) {
            Some(val) => return Ok(val.clone()),
            None => {
                return Err(RuntimeError::new(
                    &format!("undefined variable '{}'", name),
                    0,
                ))
            }
        }
    }
    //
}

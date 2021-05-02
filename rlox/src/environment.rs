use crate::error::*;
use crate::value::*;
use std::collections::HashMap;

pub type ValMap = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct Environment {
    values: Vec<ValMap>,
    gen: usize,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: vec![HashMap::new()],
            gen: 0,
        }
    }

    // TODO: really want to raii these.  hack for now
    pub fn bump(&mut self) {
        self.gen += 1;
        while self.values.len() <= self.gen {
            self.values.push(HashMap::new());
        }
    }

    pub fn debump(&mut self) {
        if self.gen > 0 {
            self.gen -= 1;
        }
    }

    pub fn bump_num(&mut self, num: usize) {
        self.gen += num;
        while self.values.len() <= self.gen {
            self.values.push(HashMap::new());
        }
    }

    pub fn reset(&mut self) -> usize {
        let num = self.gen;
        self.gen = 0;
        num
    }

    pub fn define(&mut self, name: &str, val: Value) {
        let vals = &mut self.values[self.gen];
        vals.insert(name.to_string(), val);
    }

    pub fn assign(&mut self, name: &str, val: Value, line: usize) -> Result<(), RuntimeError> {
        self.assign_impl(name, val, line, self.gen)
    }

    fn assign_impl(
        &mut self,
        name: &str,
        val: Value,
        line: usize,
        gen: usize,
    ) -> Result<(), RuntimeError> {
        let vals = &mut self.values[gen];
        if vals.contains_key(name) {
            vals.insert(name.to_string(), val);
            return Ok(());
        }

        if gen == 0 {
            return Err(RuntimeError::new(
                &format!("undefined variable '{}'", name),
                line,
            ));
        }

        self.assign_impl(name, val, line, gen - 1)
    }

    pub fn get(&self, name: &str, line: usize) -> Result<Value, RuntimeError> {
        self.get_impl(name, line, self.gen)
    }

    fn get_impl(&self, name: &str, line: usize, gen: usize) -> Result<Value, RuntimeError> {
        let vals = &self.values[gen];
        if let Some(val) = vals.get(name) {
            return Ok(val.clone());
        }

        if gen == 0 {
            return Err(RuntimeError::new(
                &format!("undefined variable '{}'", name),
                line,
            ));
        }

        self.get_impl(name, line, gen - 1)
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    fn assert_get(env: &Environment, name: &str, expect: Value) {
        let val = env.get(name, 0);
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), expect);
    }

    #[cfg(test)]
    fn assert_none(env: &Environment, name: &str) {
        let val = env.get(name, 0);
        println!("{:?}", val);
        assert!(val.is_err());
    }

    #[test]
    fn env() {
        let mut env = Environment::new();
        env.define("a", Value::String("foo".to_string()));

        env.bump();
        env.assign("a", Value::String("bar".to_string()), 0)
            .unwrap();
        assert_get(&env, "a", Value::String("bar".to_string()));

        env.define("b", Value::String("baz".to_string()));
        assert_get(&env, "a", Value::String("bar".to_string()));

        env.bump();
        env.assign("a", Value::String("final".to_string()), 0)
            .unwrap();
        assert_get(&env, "a", Value::String("final".to_string()));

        env.debump();
        env.debump();
        assert_get(&env, "a", Value::String("final".to_string()));
        assert_none(&env, "b");
    }
}

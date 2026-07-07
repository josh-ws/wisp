use std::collections::HashMap;

use crate::value::Value;

pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.vars.get(key)
    }

    pub fn define(&mut self, key: String, value: Value) {
        self.vars.insert(key, value);
    }
}

use std::collections::HashMap;

use crate::value::Value;

pub type Env = usize;

struct Frame {
    vars: HashMap<String, Value>,
    parent: Option<Env>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            vars: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Env) -> Self {
        Frame {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }
}

pub struct Arena {
    frames: Vec<Frame>,
}

impl Arena {
    pub fn new() -> Self {
        let root = Frame::new();
        Self { frames: vec![root] }
    }

    pub fn root(&self) -> Env {
        0
    }

    pub fn child(&mut self, parent: Env) -> Env {
        let len = self.frames.len();
        self.frames.push(Frame::new_with_parent(parent));
        len
    }

    pub fn get(&self, env: Env, key: &str) -> Option<Value> {
        let mut curr = Some(env);
        while curr.is_some() {
            let frame = &self.frames[curr.unwrap()];
            if let Some(v) = frame.vars.get(key) {
                return Some(v.clone());
            }
            curr = frame.parent;
        }
        None
    }

    pub fn define(&mut self, env: Env, key: String, value: Value) {
        self.frames[env].vars.insert(key, value);
    }
}

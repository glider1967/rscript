use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::eval::Value;

pub struct Env {
    env: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Result<Value> {
        self.env.get(&name).context("undefined variable").cloned()
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.env.insert(name, val);
    }
}

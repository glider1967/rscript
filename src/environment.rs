use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{bail, Ok, Result};

use crate::eval::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env {
    env: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            outer: None,
        }
    }

    pub fn with_outer(outer: Rc<RefCell<Env>>) -> Self {
        Self {
            env: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: String) -> Result<Value> {
        if let Some(val) = self.env.get(&name) {
            Ok(val.clone())
        } else {
            if let Some(outer) = &self.outer {
                outer.borrow().get(name)
            } else {
                bail!("undefined variable")
            }
        }
    }

    pub fn set(&mut self, name: String, val: Value) {
        self.env.insert(name, val);
    }
}

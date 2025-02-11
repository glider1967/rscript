use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::bail;

use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeEnv {
    env: HashMap<String, Type>,
    outer: Option<Rc<RefCell<TypeEnv>>>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            outer: None,
        }
    }

    pub fn with_outer(outer: Rc<RefCell<TypeEnv>>) -> Self {
        Self {
            env: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, name: &str) -> anyhow::Result<Type> {
        if let Some(val) = self.env.get(name) {
            Ok(val.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().get(name)
        } else {
            bail!("type: undefined variable {name}");
        }
    }

    pub fn set(&mut self, name: String, val: Type) {
        self.env.insert(name, val);
    }

    pub fn entries(&self) -> std::collections::hash_map::Iter<'_, String, Type> {
        self.env.iter()
    }
}

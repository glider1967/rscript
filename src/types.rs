use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Func(Box<Type>, Box<Type>),
}

impl Type {
    pub fn func(t1: Type, t2: Type) -> Self {
        Type::Func(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Unit => write!(f, "unit"),
            Type::Func(t1, t2) => write!(f, "({t1} -> {t2})"),
        }
    }
}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anyhow::{bail, Ok, Result};

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

    pub fn get(&self, name: String) -> Result<Type> {
        if let Some(val) = self.env.get(&name) {
            Ok(val.clone())
        } else {
            if let Some(outer) = &self.outer {
                outer.borrow().get(name)
            } else {
                dbg!("{}", &self.env);
                bail!("type: undefined variable {name}");
            }
        }
    }

    pub fn set(&mut self, name: String, val: Type) {
        self.env.insert(name, val);
    }
}

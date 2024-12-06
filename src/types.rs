use core::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Func(Box<Type>, Box<Type>),
    TypeVar(u64, Rc<RefCell<Option<Type>>>), // 推論の際に使う型変数
    Quantifier(u64),                         // 量化された型変数
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
            Type::Func(t1, t2) => write!(f, "({t1} -> {t2})"),
            Type::TypeVar(id, _) => write!(f, "t{id}"),
            Type::Quantifier(id) => write!(f, "q{id}"),
        }
    }
}

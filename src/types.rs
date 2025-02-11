use core::fmt;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Constant(String),
    Func(Box<Type>, Box<Type>),
    TypeVar(u64, Rc<RefCell<Option<Type>>>), // 推論の際に使う型変数
    Quantifier(u64),                         // 量化された型変数
}

impl Type {
    pub fn func(t1: Type, t2: Type) -> Self {
        Type::Func(Box::new(t1), Box::new(t2))
    }

    pub fn constant(s: &str) -> Self {
        Type::Constant(s.into())
    }

    pub fn to_args_and_ret(&self) -> (Vec<Type>, Type) {
        let mut args = vec![];
        let mut ret = self;
        loop {
            match ret {
                Self::Func(arg, r) => {
                    args.push(*arg.clone());
                    ret = r
                }
                _ => break,
            }
        }
        (args, ret.clone())
    }

    // zonking - 確定した型変数を引きはがす
    pub fn zonk(&self) -> Type {
        match self {
            Type::Constant(s) => Type::Constant(s.clone()),
            Type::Func(arg, ret) => Type::func(Self::zonk(arg), Self::zonk(ret)),
            Type::TypeVar(_, t1) => match *(*t1).borrow() {
                Some(ref t1) => Self::zonk(t1),
                None => self.clone(),
            },
            Type::Quantifier(id) => Type::Quantifier(*id),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.zonk() {
            Type::Constant(s) => write!(f, "{s}"),
            Type::Func(t1, t2) => write!(f, "({t1} -> {t2})"),
            Type::TypeVar(id, _) => write!(f, "t{id}"),
            Type::Quantifier(id) => write!(f, "q{id}"),
        }
    }
}

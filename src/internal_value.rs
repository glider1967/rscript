use core::fmt;

use crate::{environment::Env, expression::Expr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Lambda(String, Box<Expr>, Env),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Lambda(v, _, _) => write!(f, "lambda ({v})"),
        }
    }
}
